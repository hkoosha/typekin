pub mod subject {
    #[derive(Debug)]
    pub struct ConstructedThingy(pub f64);
}

use subject::ConstructedThingy as Subject;

fn main() {
    let it = Subject(1.1f64);
    println!("subject: {:?}", it);
}
