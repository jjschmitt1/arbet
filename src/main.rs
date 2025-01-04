use std::io::{self, Write};
mod arb;

fn main() {
    println!("Welcome to arbet, your personal arbitrage betting finder!");

    // buffer for taking input
    let mut buf = String::new();

    // print prompt for first line
    print!("Line 1: ");
    io::stdout().flush().unwrap();

    // get input for first line
    io::stdin().read_line(&mut buf).expect("Error reading input");

    // cast to int
    let line_one: i32 = buf.trim().parse::<i32>().unwrap();

    // clear the buffer
    buf.clear();

    // prompt for second line
    print!("Line 2: ");
    io::stdout().flush().unwrap();

    // read second line
    io::stdin().read_line(&mut buf).expect("Error reading input");

    // cast second line
    let line_two: i32 = buf.trim().parse::<i32>().unwrap();

    let _ = arb::calc_arb(&line_one, &line_two);

}
