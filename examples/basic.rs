use msws::Rand;

fn main() {
    let seed = 0xb5ad4eceda1ce2a9;
    let mut r = Rand::new(seed).expect("invalid seed");
    for i in 0..10 {
        println!("{}: {}", i, r.rand());
    }
}
