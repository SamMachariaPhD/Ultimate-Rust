use std::io; 

static NO:f64 = 5.0;

fn sq(sth: f64) -> f64{
    sth*sth // `return sth*sth;` also works but rarely used
}

fn main() {
    let mut n_ = String::new();
    
    println!("Enter a no: ");
    io::stdin().read_line(&mut n_).expect("Not a valid string.");
    let no:f64 = n_.trim().parse().expect("Not a valid number.");

    println!("SQroot of {:.4} = {:.4}", no, sq(no));
    
    let sh = sq(NO);
    println!("`static` SQroot = {}", sh);
}
