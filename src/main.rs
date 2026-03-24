use std::io;
use meval::Expr;
fn main() {
    loop {
        
    
    let mut input = String::new();
    println!("Input Your Calculations");
    io::stdin().read_line(&mut input).unwrap();    
    let no: f64 = input.parse::<Expr>().unwrap().eval().unwrap();

    println!("Your Result is :{:?}",no);
    println!();
    println!();

    
};
}