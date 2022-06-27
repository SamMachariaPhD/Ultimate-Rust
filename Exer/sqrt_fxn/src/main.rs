
static NO:f64 = 5.0;

fn sq(sth: f64) -> f64{
    sth*sth
}

fn main() {
    println!("SQroot of {:.4} = {:.4}", NO, sq(NO));
    let sh = sq(NO);
    println!("SQroot = {}", sh);
}
