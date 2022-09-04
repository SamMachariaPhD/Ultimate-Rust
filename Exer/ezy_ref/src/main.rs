

fn main() {
    let n = 3;
    let ref_n = &n;

    println!("Number n = {}\nRef n (&n,*ref_n) = {}\nRef n (&n,ref_n) = {}", n, *ref_n, ref_n);
}
