#[derive(Debug)]
pub struct ConstructedThingy(pub f64);

fn main() {
    let it = ConstructedThingy(1.1f64);
    println!("subject: {:?}", it);
}
