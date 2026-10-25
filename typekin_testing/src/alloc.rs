use std::alloc::{
    GlobalAlloc,
    Layout,
    System,
};
use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::thread::AccessError;

thread_local! {
    // Const TLS initialization and Cell access do not allocate. Each test thread
    // watches only its own value, never allocations made by the test harness.
    static OBSERVATION: Cell<Option<AllocationObservation>> =
        Cell::new(Some(AllocationObservation {
            address: 0,
            deallocations: 0,
            reallocations: 0,
            _unsend: Default::default(),
        }));
}

struct AllocationObservation {
    address: usize,
    deallocations: usize,
    reallocations: usize,
    _unsend: PhantomData<Rc<()>>,
}

impl AllocationObservation {
    fn reset_for(
        &mut self,
        pointer: *const u8,
    ) {
        self.reallocations = 0;
        self.deallocations = 0;
        self.address = pointer.addr();
    }
}

fn try_with<T>(
    f: impl FnOnce(&mut AllocationObservation) -> T
) -> Result<T, AccessError> {
    return OBSERVATION.try_with(|it| {
        let mut ao = it.take().unwrap_or_else(|| AllocationObservation {
            address: 0,
            deallocations: 0,
            reallocations: 0,
            _unsend: Default::default(),
        });

        let ret = f(&mut ao);

        it.set(Some(ao));

        return ret;
    });
}

fn with<T>(f: impl FnOnce(&mut AllocationObservation) -> T) -> T {
    return try_with(f).unwrap();
}

#[derive(Debug)]
pub struct AllocationWatch {
    active: bool,
    _unsend: PhantomData<Rc<()>>,
}

impl !Send for AllocationWatch {}
impl !Sync for AllocationWatch {}

impl AllocationWatch {
    #[must_use]
    pub fn watch(pointer: *const u8) -> Self {
        with(|it| {
            if it.address != 0 {
                panic!(
                    "allocation watches cannot nest: {} <=> {}",
                    it.address,
                    pointer.addr()
                );
            }
            it.reset_for(pointer);
        });

        let this = Self {
            active: true,
            _unsend: Default::default(),
        };

        return this;
    }

    #[must_use]
    pub fn deallocations(&self) -> usize {
        return with(|it| it.deallocations);
    }

    pub fn assert_released(mut self) {
        self.active = false;

        let (addr, dealloc, alloc) = with(|it| {
            let observation = (it.address, it.deallocations, it.reallocations);
            it.reset_for(std::ptr::null());
            return observation;
        });

        assert_eq!(addr, 0);
        assert_eq!(dealloc, 1, "consumed allocation leaked");
        assert_eq!(alloc, 0, "spare capacity should be preserved");
    }
}

impl Drop for AllocationWatch {
    fn drop(&mut self) {
        if self.active {
            with(|it| it.reset_for(std::ptr::null()));
        }
    }
}

pub struct ObservedAllocator;

unsafe impl GlobalAlloc for ObservedAllocator {
    unsafe fn alloc(
        &self,
        layout: Layout,
    ) -> *mut u8 {
        return unsafe { System.alloc(layout) };
    }

    unsafe fn dealloc(
        &self,
        pointer: *mut u8,
        layout: Layout,
    ) {
        let _ = try_with(|it| {
            if it.address == pointer.addr() {
                // Disarm before releasing: later unrelated allocations can
                // reuse this address without being mistaken for a second drop.
                it.address = 0;
                it.deallocations += 1;
            }
        });
        unsafe { System.dealloc(pointer, layout) };
    }

    unsafe fn realloc(
        &self,
        pointer: *mut u8,
        layout: Layout,
        size: usize,
    ) -> *mut u8 {
        let replacement = unsafe { System.realloc(pointer, layout, size) };
        if !replacement.is_null() {
            with(|it| {
                if it.address == pointer.addr() {
                    it.reallocations += 1;
                }
            });
        }
        return replacement;
    }
}
