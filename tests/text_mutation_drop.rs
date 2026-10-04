extern crate alloc;

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use std::alloc::{
        GlobalAlloc,
        Layout,
        System,
    };
    use std::cell::Cell;
    use std::panic::AssertUnwindSafe;
    use std::rc::Rc;

    //noinspection SpellCheckingInspection
    const MY_WORD: &str = "safeé";

    #[derive(Clone, Copy)]
    struct AllocationObservation {
        address: usize,
        deallocations: usize,
        reallocations: usize,
    }

    thread_local! {
        // Const TLS initialization and Cell access do not allocate. Each test thread
        // watches only its own String, never allocations made by the test harness.
        static OBSERVATION: Cell<Option<AllocationObservation>> = const { Cell::new(None) };
    }

    struct ObservedAllocator;

    #[global_allocator]
    static ALLOCATOR: ObservedAllocator = ObservedAllocator;

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
            let _ = OBSERVATION.try_with(|observation| {
                if let Some(mut current) = observation.get()
                    && current.address == pointer as usize
                {
                    // Disarm before releasing: later unrelated allocations can
                    // reuse this address without being mistaken for a second drop.
                    current.address = 0;
                    current.deallocations += 1;
                    observation.set(Some(current));
                }
            });
            unsafe { System.dealloc(pointer, layout) };
        }

        unsafe fn alloc_zeroed(
            &self,
            layout: Layout,
        ) -> *mut u8 {
            return unsafe { System.alloc_zeroed(layout) };
        }

        unsafe fn realloc(
            &self,
            pointer: *mut u8,
            layout: Layout,
            size: usize,
        ) -> *mut u8 {
            let replacement = unsafe { System.realloc(pointer, layout, size) };
            if !replacement.is_null() {
                let _ = OBSERVATION.try_with(|observation| {
                    if let Some(mut current) = observation.get()
                        && current.address == pointer as usize
                    {
                        current.address = replacement as usize;
                        current.reallocations += 1;
                        observation.set(Some(current));
                    }
                });
            }
            return replacement;
        }
    }

    struct AllocationWatch {
        active: bool,
    }

    impl AllocationWatch {
        fn new(pointer: *const u8) -> Self {
            OBSERVATION.with(|observation| {
                assert!(
                    observation.get().is_none(),
                    "allocation watches cannot nest"
                );
                observation.set(Some(AllocationObservation {
                    address: pointer as usize,
                    deallocations: 0,
                    reallocations: 0,
                }));
            });
            return Self { active: true };
        }

        fn snapshot(&self) -> AllocationObservation {
            return OBSERVATION.with(|observation| observation.get().unwrap());
        }

        fn assert_released(mut self) {
            let observed = OBSERVATION
                .with(|observation| observation.replace(None).unwrap());
            self.active = false;
            assert_eq!(
                observed.deallocations, 1,
                "consumed String allocation leaked"
            );
            assert_eq!(observed.address, 0);
            assert_eq!(
                observed.reallocations, 0,
                "spare capacity should be preserved"
            );
        }
    }

    impl Drop for AllocationWatch {
        fn drop(&mut self) {
            if self.active {
                OBSERVATION.set(None);
            }
        }
    }

    fn valid_owned_text(value: &str) -> bool {
        if value == "panic" {
            panic!("validator panic");
        }
        return value == "safe" || value == MY_WORD;
    }

    #[typekin::text(konst = false, valid = valid_owned_text)]
    #[repr(transparent)]
    struct OwnedText(String);

    fn tracked_text(value: &str) -> (OwnedText, AllocationWatch) {
        let mut backing = String::with_capacity(64);
        backing.push_str(value);
        let text = OwnedText::try_make(backing).unwrap();
        assert!(text.capacity() >= 64);
        let watch = AllocationWatch::new(text.as_str().as_ptr());
        return (text, watch);
    }

    struct EmptySetSource(String);
    struct TrustEmptySetSource(String);

    fn empty_set_text(source: EmptySetSource) -> String {
        source.0
    }

    fn trusted_empty_set_text(source: TrustEmptySetSource) -> String {
        source.0
    }

    #[typekin::text(
        konst = false,
        in = [],
        friends = [
            empty_set_text(EmptySetSource) -> Make,
            trusted_empty_set_text(TrustEmptySetSource) -> [Make, Trust],
        ],
    )]
    #[repr(transparent)]
    struct EmptySetText(String);

    #[typekin::text(konst = false, in = [], valid = valid_owned_text)]
    #[repr(transparent)]
    struct CallbackEmptySetText(String);

    #[test]
    fn empty_membership_releases_rejected_owned_constructor_inputs() {
        type Construction = fn(String) -> bool;
        let constructors: [(&str, Construction); 3] = [
            ("checked", |raw| EmptySetText::try_make(raw).is_err()),
            ("callback and membership", |raw| {
                CallbackEmptySetText::try_make(raw).is_err()
            }),
            ("untrusted friend", |raw| {
                assert_panics(|| EmptySetText::of(EmptySetSource(raw)))
            }),
        ];
        for (name, construct) in constructors {
            for value in ["", "safe", "é🦀"] {
                let mut raw = String::with_capacity(64);
                raw.push_str(value);
                let watch = AllocationWatch::new(raw.as_ptr());
                assert!(construct(raw), "{name} must reject {value:?}");
                watch.assert_released();
            }
        }
    }

    #[test]
    fn empty_membership_releases_trusted_buffers_after_rejected_noop_changes() {
        type Mutation = fn(EmptySetText) -> bool;
        let mutations: [Mutation; 2] = [
            |text| text.map(|_| {}).is_err(),
            |text| text.try_push_str("").is_err(),
        ];
        for mutate in mutations {
            let mut raw = String::with_capacity(64);
            raw.push_str("é🦀");
            let watch = AllocationWatch::new(raw.as_ptr());
            let text = EmptySetText::of(TrustEmptySetSource(raw));
            assert_eq!(watch.snapshot().deallocations, 0);
            assert!(mutate(text));
            watch.assert_released();
        }
    }

    struct DropProbe(Rc<Cell<usize>>);

    impl DropProbe {
        fn assert_live(&self) {
            assert_eq!(self.0.get(), 0);
        }
    }

    impl Drop for DropProbe {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    #[test]
    fn successful_mutation_keeps_the_allocation_until_the_owner_is_dropped() {
        let (text, watch) = tracked_text("safe");
        let original_address = text.as_str().as_ptr() as usize;
        let text = text.try_push('é').unwrap();
        assert_eq!(text.as_str(), MY_WORD);
        assert_eq!(text.as_str().as_ptr() as usize, original_address);
        assert_eq!(watch.snapshot().deallocations, 0);
        drop(text);
        watch.assert_released();
    }

    #[test]
    fn every_rejected_mutation_releases_the_owned_backing_allocation() {
        type Mutation = fn(OwnedText) -> bool;
        let mutations: [(&str, Mutation); 11] = [
            ("push", |text| text.try_push('!').is_err()),
            ("push_str", |text| text.try_push_str("!").is_err()),
            ("insert", |text| text.try_insert(1, '!').is_err()),
            ("insert_str", |text| text.try_insert_str(1, "!").is_err()),
            ("replace_range", |text| {
                text.try_replace_range(0..1, "x").is_err()
            }),
            ("truncate", |text| text.try_truncate(3).is_err()),
            ("clear", |text| text.try_clear().is_err()),
            ("retain", |text| {
                text.try_retain(|character| character != 's').is_err()
            }),
            ("remove", |text| text.try_remove(0).is_err()),
            ("pop", |text| text.try_pop().is_err()),
            ("map", |text| text.map(|value| value.clear()).is_err()),
        ];

        for (name, mutate) in mutations {
            let (text, watch) = tracked_text("safe");
            assert!(mutate(text), "{name} must reject invalid output");
            watch.assert_released();
        }
    }

    #[test]
    fn invalid_add_panics_and_releases_the_consumed_buffer() {
        let (text, watch) = tracked_text("safe");
        assert_panics(move || text + "!");
        watch.assert_released();
    }

    #[test]
    fn native_string_index_panics_release_the_consumed_buffer() {
        type Mutation = fn(OwnedText);
        let mutations: [(&str, Mutation); 9] = [
            ("insert inside UTF-8", |text| {
                let _ = text.try_insert(5, 'x');
            }),
            ("insert_str inside UTF-8", |text| {
                let _ = text.try_insert_str(5, "x");
            }),
            ("remove inside UTF-8", |text| {
                let _ = text.try_remove(5);
            }),
            ("replace_range inside UTF-8", |text| {
                let _ = text.try_replace_range(5..6, "x");
            }),
            ("truncate inside UTF-8", |text| {
                let _ = text.try_truncate(5);
            }),
            ("insert past end", |text| {
                let _ = text.try_insert(99, 'x');
            }),
            ("insert_str past end", |text| {
                let _ = text.try_insert_str(99, "x");
            }),
            ("remove past end", |text| {
                let _ = text.try_remove(99);
            }),
            ("replace_range past end", |text| {
                let _ = text.try_replace_range(99..100, "x");
            }),
        ];

        for (_, mutate) in mutations {
            let (text, watch) = tracked_text(MY_WORD);
            assert_panics(move || mutate(text));
            watch.assert_released();
        }
    }

    #[test]
    fn retain_predicate_panic_drops_partially_mutated_buffer_and_owned_capture()
    {
        let drops = Rc::new(Cell::new(0));
        let probe = DropProbe(Rc::clone(&drops));
        let (text, watch) = tracked_text("safe");
        assert_panics(AssertUnwindSafe(move || {
            let mut visited = 0;
            let _ = text.try_retain(move |_| {
                probe.assert_live();
                visited += 1;
                if visited == 3 {
                    panic!("retain predicate panic");
                }
                return visited != 1;
            });
        }));
        assert_eq!(
            drops.get(),
            1,
            "predicate-owned state must be dropped once"
        );
        watch.assert_released();
    }

    #[test]
    fn map_panic_drops_mutated_buffer_and_owned_capture() {
        let drops = Rc::new(Cell::new(0));
        let probe = DropProbe(Rc::clone(&drops));
        let (text, watch) = tracked_text("safe");
        assert_panics(AssertUnwindSafe(move || {
            let _ = text.map(move |value| {
                probe.assert_live();
                value.clear();
                value.push_str("invalid");
                panic!("map callback panic");
            });
        }));
        assert_eq!(drops.get(), 1, "map-owned state must be dropped once");
        watch.assert_released();
    }

    #[test]
    fn validator_panic_drops_the_mutated_buffer() {
        let (text, watch) = tracked_text("safe");
        assert_panics(AssertUnwindSafe(move || {
            let _ = text.try_replace_range(.., "panic");
        }));
        watch.assert_released();
    }

    #[test]
    fn validator_panic_after_map_drops_buffer_and_owned_capture() {
        let drops = Rc::new(Cell::new(0));
        let probe = DropProbe(Rc::clone(&drops));
        let (text, watch) = tracked_text("safe");
        assert_panics(AssertUnwindSafe(move || {
            let _ = text.map(move |value| {
                probe.assert_live();
                value.clear();
                value.push_str("panic");
            });
        }));
        assert_eq!(drops.get(), 1);
        watch.assert_released();
    }

    #[test]
    fn rejected_map_drops_its_owned_capture_and_buffer() {
        let drops = Rc::new(Cell::new(0));
        let probe = DropProbe(Rc::clone(&drops));
        let (text, watch) = tracked_text("safe");
        let result = text.map(move |value| {
            probe.assert_live();
            value.clear();
        });
        assert!(result.is_err());
        assert_eq!(drops.get(), 1);
        watch.assert_released();
    }

    #[test]
    fn rejected_retain_drops_its_owned_capture_and_buffer() {
        let drops = Rc::new(Cell::new(0));
        let probe = DropProbe(Rc::clone(&drops));
        let (text, watch) = tracked_text("safe");
        let result = text.try_retain(move |_| {
            probe.assert_live();
            return false;
        });
        assert!(result.is_err());
        assert_eq!(drops.get(), 1);
        watch.assert_released();
    }

    pub fn assert_panics<T>(
        f: impl FnOnce() -> T + std::panic::UnwindSafe
    ) -> bool {
        std::panic::set_hook(Box::new(move |_| {}));
        let result = std::panic::catch_unwind(f);
        let _ = std::panic::take_hook();
        assert!(result.is_err());
        return true;
    }
}
