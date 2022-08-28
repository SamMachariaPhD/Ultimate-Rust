
fn main() {
    let arr = [1,2,3,5,9,4,2,8,5,6];
    println!("Single {:?}\nDouble {:?}", arr, double(arr));
}

fn double(mut a:[i32;10]) -> [i32;10]{
    for i in 0..10 {
        a[i] *= 2;
    }
    a
}