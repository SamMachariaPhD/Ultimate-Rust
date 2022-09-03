
fn main() {
    // let mut ar = [2,3,4,5,6];
    // for n in 0..ar.len(){
    //     ar[n] *= 2;
    // }
    // println!("{:?}", ar);
    let mut arr = [1,2,3,5,9,4,2,8,5,6];
    //println!("Single {:?}\nDouble {:?}\nEfficient{:?}", arr, double(arr), double2(&mut arr));
    println!("Single {:?}", arr);
    double2(&mut arr);
    println!("Efficient {:?}", arr);
}

// fn double(mut a:[i32;10]) -> [i32;10]{
//     for i in 0..a.len() {
//         a[i] *= 2;
//     }
//     a
// }

// passing argument by referencing 

fn double2(a: &mut [i32;10]){ // to do a.len() here, a would have to be a const 
    for i in 0..a.len(){
        //(*a)[i] *= 2; // actually not required 
        a[i] *= 2;
    }
}