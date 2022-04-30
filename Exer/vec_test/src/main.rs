fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(3);
    v.push(21);
    v.push(7); v.push(4); v.push(6); v.push(17);
    v.push(2);
    v.pop();
    let v2 = vec![3,4,5,2,5,6];
    // let v = v.pop();
    println!("Vector\t{:?}", v);
    println!("Vector idx\t{}", v[v.len()-3]);
    println!("Vector\t{:?}", v2);
}
