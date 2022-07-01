use std::io;

fn factorial(n:u64) -> u64{ //64 bit unsigned integer type 
    if n == 0{
        1
    }else{
        n*factorial(n-1)
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter an integer: ");
    io::stdin().read_line(&mut input).expect("Not a valid unsigned integer");
    let no:u64 = input.trim().parse().expect("Not a valid unsigned integer");

    println!("{} factorial is {}", no, factorial(no));
}
