fn main() {
    println!("f32\t{}", quad_root(26f32));
    println!("f64\t{}", quad_root(26f64));
}

trait SquareRoot{
    fn sqroot(self) -> Self;
}

impl SquareRoot for f32{
    fn sqroot(self) -> Self{
        self.sqrt()
    }
}

impl SquareRoot for f64{
    fn sqroot(self) -> Self{
        self.sqrt()
    }
}

fn quad_root<Num>(x: Num) -> Num 
where Num: SquareRoot {
    x.sqroot().sqroot()
}