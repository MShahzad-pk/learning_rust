#[allow(unused_imports)]

use library;
fn main() {
    let x = library::take_string(String::from("X"));
   

    let y = library::take_u8(String::from("Y"));

    println!("");
    println!("Value of x is {}",x);
    println!("Value of y is {}",y);
    
}
