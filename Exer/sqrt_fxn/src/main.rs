use std::io; 

static NO:f32 = 5.0;

fn sq(sth: f32) -> f32{
    sth*sth // `return sth*sth;` also works but rarely used
}

// absolute value of a floating-point number
fn abs(x: f32) -> f32 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

// ensure the number always falls in the given range
fn clamp(x: f32, x1: f32, x2: f32) -> f32 {
    if x < x1 {
        x1
    } else if x > x2 {
        x2
    } else {
        x
    }
}

fn main() {
    let mut n_ = String::new();
    
    println!("Enter a no: ");
    io::stdin().read_line(&mut n_).expect("Not a valid string.");
    let no:f32 = n_.trim().parse().expect("Not a valid number.");

    println!("SQroot of {:.4} = {:.4}", no, sq(no));
    println!("The abs val of {} is {}.", no, abs(no));
    
    let _sh = sq(NO); // `_sth` for unused/unimplemented variable. `_` alone has less information about the variable name.
    //println!("`static` SQroot = {}", sh);
}
