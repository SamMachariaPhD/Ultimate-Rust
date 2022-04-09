use rand::Rng;

fn main() {
    let mut rnd_no = rand::thread_rng();
    println!("Random int\t {}", rnd_no.gen_range(10..100));
    println!("Random flt\t {}", rnd_no.gen_range(10.0..100.0));
}
