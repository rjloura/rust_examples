use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter something: ");
    io::stdin().read_line(&mut input).expect("user input");

    // Trim off the '\n'
    let input_trimmed = input.trim();

    dbg!(input_trimmed);
}
