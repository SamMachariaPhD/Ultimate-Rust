fn main() {
    let upper:i32 = 5;
    for i in 0..=upper{
        if i%2 == 0{
            println!("{} is an even number", i);
        }else{
            println!("{} is an odd number", i);
        }
        // Or 
        let even_odd = if i%2==0{"even"} else {"odd"};
        println!("{}|-->{}", i,even_odd);
    }
}
