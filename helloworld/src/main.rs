use std::io;
use std::io::*;

fn main() {
    let num1 = 2;
    let num2 = 2;

    println!("Hello, world!");
    println!("{num1} + {num2} is {}, -1 thats {} quick mafs.", num1+num2, (num1+num2)-1 );

    print!("\nEnter something: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    println!("You entered: {input}");
}
