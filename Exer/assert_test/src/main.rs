// `assert_eq!` is a macro that asserts two things are equal.
// it is great for testing. 
// if those things are equal, no output.
// if they are not equal, `panic`.
// check also `Assert!(a == b)`

fn main() {
    println!("`assert_eq!` test:");
    let ans = 73;
    assert_eq!(ans,73);
    //assert_eq!(ans,73, "testing a {} and {} equivalence assertion", ans, 73);
}
