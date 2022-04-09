use rand::Rng;

fn main() {
    let mut rnd_no = rand::thread_rng();
    println!("Random number\t {}", rnd_no.gen_range(10..100));
}
