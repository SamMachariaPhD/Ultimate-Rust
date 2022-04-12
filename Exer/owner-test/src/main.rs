fn main() {
    let s1 = String::from("bunny");
    let s2 = s1;
    //println!("s1 is\t{}", s1); // error - value of s1 was moved to s2
    println!("s2 is\t{}", s2);
}
