fn main() {
    let mut sum:f64 = 0.0;
    let upper:i32 = 5;
    //let precision:usize = 5; // unsigned integer 
    for i in 0..upper{ // in Rust, step is not implemented for f64
        sum += i as f64 +0.1;
        println!("sum = {:.7}", sum); //{:.1$}", sum, precision 
    }
}
