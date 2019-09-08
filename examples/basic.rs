use msws::{seed, Rand};

fn main() {
    let seed = seed(0);
    let mut r = Rand::new(seed).expect("invalid seed");
    for i in 0..10 {
        println!("{}: {:#010x}", i, r.rand());
    }
}
