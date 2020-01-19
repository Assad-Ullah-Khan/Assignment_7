mod squaring{
    pub fn square(input_1:u32){
        println!("Square of the {} is: {}",input_1,input_1*input_1 );
    }
}

use std::io;
fn main() {
    println!("Please enter a number: ");
    
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Error");

    let input_1:u32=input.trim().parse().expect("Error");

    squaring::square(input_1);
}