use std::io;
use std::io::Write;

mod permutations;
use permutations::{IP,FP,E,PF};

fn main() {
    println!("Welcome to the Rust Encryption/Decryption program!");

    loop {
        println!();
        println!("Choose an option:");
        println!("1. Encrypt");
        println!("2. Decrypt");
        println!("3. Print Permutations");
        println!("Press any other number to exit.");
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
                print_permutations();
            }
            _ => {
                println!("Exiting program!");
                break;
            }
        }
    }
}

fn print_permutations() {
    /* 
     * Print the permutations for debugging purposes
    */
    println!("Initial Permutation: {:?}", IP);
    println!("Final Permutation: {:?}", FP);
    println!("Expansion: {:?}", E);
    println!("Permutation in F: {:?}", PF);
    println!();

    // Initialize the data array with consecutive integers starting from 1
    let mut data: [usize; 64] = [0; 64];
    for (index, element) in data.iter_mut().enumerate() {
        *element = index + 1;
    }

    let permuted_data: [usize; 64] = permute(&IP, &data);

    reverse_permute(&IP, &permuted_data);
}

fn get_input(prompt: &str) -> String {
    loop {
        print!("{}", prompt);
        let _ = io::stdout().flush();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let trimmed_input = input.trim();
        
        // Each char is 4 bytes, we want the input to be 64 bits long, so input should be 2 characters long
        if trimmed_input.len() < 2 {
            println!("Input should be at least 2 characters long. Please try again.");
        } else {
            return trimmed_input[..2].to_string(); // Take first 16 characters if input is longer
        }
    }
}

fn permute(permutation_table: &[usize; 64], data: &[usize; 64]) -> [usize; 64] {
    let mut permuted_data: [usize; 64] = [0; 64];
    for i in 0..64 {
        // Data element i goes to the position specified in permutation table
        // For example if permutation_table[i] = 58, then data[i] goes to position 58 - 1 = 57 in permuted_data
        permuted_data[permutation_table[i] - 1] = data[i];
    }

    // Print the original and permuted data
    println!("Original Data: {:?}", data);
    println!("Permuted Data: {:?}", permuted_data);
    println!();

    permuted_data
}

fn reverse_permute(permutation_table: &[usize; 64], permuted_data: &[usize; 64]) -> [usize; 64] {
    let mut data: [usize; 64] = [0; 64];
    for i in 0..64 {
        // Permuted data element i goes back to the position specified in permutation table
        // For example if permutation_table[i] = 58, then permuted_data[i] goes to position 58 - 1 = 57 in data
        data[i] = permuted_data[permutation_table[i] - 1];
    }

    // Print the original and reverse-permuted data
    println!("Permuted Data: {:?}", permuted_data);
    println!("Reversed Permuted Data: {:?}", data);
    println!();

    data
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
