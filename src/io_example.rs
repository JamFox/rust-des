use std::io;
use std::io::Write;

fn read_number() -> Result<i32, io::Error> {
    let mut input = String::new();

    // Read input from the user
    io::stdin().read_line(&mut input)?;

    // Parse the input as an integer
    let number: i32 = input.trim().parse()?;

    Ok(number)
}

fn main() {
    print!("Please enter your name:");
    // let _ to discard output value
    // flush stout because stdout is line buffered 
    // so without newline buffer is not flushed
    let _ = std::io::stdout().flush();
    // store input in mutable var
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    println!("Hello, {}!", input.trim());
}
