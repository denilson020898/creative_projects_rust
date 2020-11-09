use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();
    println!("{}", rng.gen_range(0, 20));
    println!("{}", rng.gen::<f64>());
    println!("{}", if rng.gen() { "Heads" } else { "Tails" });
}
