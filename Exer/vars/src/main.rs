const STARTING_MISSILES: i32 = 10;
const READY_AMOUNT: i32 = 3;

fn main() {
    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left!", (missiles - ready));
}
