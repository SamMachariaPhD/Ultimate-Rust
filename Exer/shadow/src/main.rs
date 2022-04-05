fn main() {
    let bunnies = 4;
    {
        let bunnies = 8.2;
        println!("First bunny:\t{}", bunnies);
    }
    println!("Second bunny:\t{}", bunnies);

    let mut buny = 5; println!("Third bunny:\t{}", buny);
    // let buny = buny;
    // println!("Fourth bunny:\t{}", buny);
    buny = 3; println!("Fifth bunny:\t{}", buny);
}
