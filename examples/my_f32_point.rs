/// A position: differences are plain `f32`, and positions cannot be added.
#[typekin::point(
    konst = none,
    produce = Self::into_offset(_) -> f32,
)]
pub struct Position(f32);

impl Position {
    fn into_offset(self) -> f32 {
        return self.raw();
    }
}

fn main() {
    let start = Position::make(1.5);
    let end = Position::make(4.0);

    let offset: f32 = end - start;
    assert_eq!(offset, 2.5);
    assert!(start < end);

    println!("{offset}");
}
