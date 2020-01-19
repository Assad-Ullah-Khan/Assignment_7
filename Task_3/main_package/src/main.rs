use std::io;
use lib_package;

fn main() {
    println!("Please enter a number: ");
    
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Error");

    let input_1:u32=input.trim().parse().expect("Error");

    lib_package::squaring::sub_squaring::square(input_1);
}