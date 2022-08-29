
fn main() {
    // let mut ar = [2,3,4,5,6];
    // for n in 0..ar.len(){
    //     ar[n] *= 2;
    // }
    // println!("{:?}", ar);
    let arr = [1,2,3,5,9,4,2,8,5,6];
    println!("Single {:?}\nDouble {:?}", arr, double(arr));
}

fn double(mut a:[i32;10]) -> [i32;10]{
    for i in 0..a.len() {
        a[i] *= 2;
    }
    a
}