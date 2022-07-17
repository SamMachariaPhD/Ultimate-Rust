// fn main() {
//     println!("Hello, world!");
// }

#[test]
fn works(){
    assert!(true); // what about `!true`?
} 

use std::process::Command;

#[test]
fn runs(){
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok());
}