fn main() {
    let arr = [1,2,3,5,9,4,2,8,5,6];
    println!("{:?}", double(arr));
}

fn double(mut a:[i32;10]) {
    for i in 0..10 {
        a[i] *= 2;
    }
}