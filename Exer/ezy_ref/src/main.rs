

fn main() {
    let n = 3;
    let x = &&&5;
    let ref_n = &n;

    println!("Number n = {}\nRef n (&n,*ref_n) = {}\nRef n (&n,ref_n) = {}", n, *ref_n, ref_n);
    /*
    output:
    Number n = 3
    Ref n (&n,*ref_n) = 3
    Ref n (&n,ref_n) = 3
    */
    println!("***x = {}\n**x = {}\n*x = {}\nx = {}", ***x, **x, *x, x);
    /*
    output:
    ***x = 5    the object referenced by "x" is obtained using the right most "*"
    **x = 5     the object is a reference of reference
    *x = 5      the object is a reference and it is taken using "*"
    x = 5       the object is a number 5
    */
}
