use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to the Rust Encryption/Decryption program!");

    loop {
        println!();
        println!("Choose an option:");
        println!("1. Encrypt");
        println!("2. Decrypt");
        println!("3. Exit");
        println!();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                let plaintext = get_input("Enter 64-bit plaintext: ");
                let cipher_key = get_input("Enter 64-bit cipher key: ");
                encrypt(plaintext.as_bytes(), cipher_key.as_bytes());
            }
            2 => {
                let ciphertext = get_input("Enter 64-bit ciphertext: ");
                let cipher_key = get_input("Enter 64-bit cipher key: ");
                decrypt(ciphertext.as_bytes(), cipher_key.as_bytes());
            }
            3 => {
                println!("Exiting program!");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a valid option (1, 2, or 3).");
            }
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    // let _ to discard output value and flush
    let _ = std::io::stdout().flush();
    // store input in mutable var
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    // rust return last line if semicolon is not at the end
    // you could also add return at the front and semicolon at the end
    input.trim().to_string()
}

fn encrypt(plaintext: &[u8], cipher_key: &[u8]) {
    // Placeholder for encryption function
    println!("Encrypting:");
    println!("Plaintext String: {}", String::from_utf8_lossy(plaintext));
    println!("Plaintext Byte String: {:?}", plaintext);
    println!("Key String: {}", String::from_utf8_lossy(cipher_key));
    println!("Key Byte String: {:?}", cipher_key);
}

fn decrypt(ciphertext: &[u8], cipher_key: &[u8]) {
    // Placeholder for decryption function
    println!("Decrypting:");
    println!("Ciphertext String: {}", String::from_utf8_lossy(ciphertext));
    println!("Ciphertext Byte String: {:?}", ciphertext);
    println!("Key String: {}", String::from_utf8_lossy(cipher_key));
    println!("Key Byte String: {:?}", cipher_key);
}

