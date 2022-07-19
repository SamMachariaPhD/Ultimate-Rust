// fn main() {
//     println!("Hello, world!");
// }

#[test]
fn works(){
    assert!(true); // what about `!true`?
} 
/*
use std::process::Command;

#[test]
fn runs(){
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok());
}

#[test]
fn runs2(){
    let mut cmd2 = Command::new("pwd");
    let res2 = cmd2.output();
    assert!(res2.is_ok());
} 
*/
use assert_cmd::Command; 

#[test]
fn runs3(){
    let mut cmd3 = Command::cargo_bin("tests").unwrap(); 
    cmd3.assert().success(); 
} 

