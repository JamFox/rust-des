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
                encrypt(plaintext, cipher_key);
            }
            2 => {
                let ciphertext = get_input("Enter 64-bit ciphertext: ");
                let cipher_key = get_input("Enter 64-bit cipher key: ");
                decrypt(ciphertext, cipher_key);
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

fn encrypt(plaintext: String, cipher_key: String) {
    // Placeholder for encryption function
    println!("Encrypting: {} with key: {}", plaintext, cipher_key);
    let plaintext_byte_string: &[u8] = plaintext.as_bytes();
    println!("Plaintext Byte String: {:?}", plaintext_byte_string);
    let cipher_key_byte_string: &[u8] = cipher_key.as_bytes();
    println!("Key Byte String: {:?}", cipher_key_byte_string);
}

fn decrypt(ciphertext: String, cipher_key: String) {
    // Placeholder for decryption function
    println!("Decrypting: {} with key: {}", ciphertext, cipher_key);
    let ciphertext_byte_string: &[u8] = ciphertext.as_bytes();
    println!("Ciphertext Byte String: {:?}", ciphertext_byte_string);
    let cipher_key_byte_string: &[u8] = cipher_key.as_bytes();
    println!("Key Byte String: {:?}", cipher_key_byte_string);
}

