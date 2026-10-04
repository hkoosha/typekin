pub mod subject {
    #[derive(Debug)]
    pub struct ConstructedThingy(pub f64);
}

type Subject = subject::ConstructedThingy;

fn main() {
    let it = Subject(1.1f64);
    println!("subject: {}", it);
}
