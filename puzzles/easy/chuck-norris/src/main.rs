use std::io;

use primal::Sieve;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let message = input_line.trim_right().to_string();

    // Write an action using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("answer");
}
