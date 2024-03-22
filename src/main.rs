use std::io;
use std::io::Write;

mod permutations;
use permutations::{permute_u32, permute_u64, expand, FP, IP, PC1, PC2, PF};
mod sboxes;
//use sboxes::{substitute};

fn main() {
    println!("Welcome to the Rust DES cryptography program!");

    println!();
    println!("--- Menu ---");
    println!("1. Encrypt");
    println!("2. Decrypt");
    println!("3. Print Debug stuff");
    println!("Press any other number to exit.");
    println!();

    let choice = get_input("Your choice: ");

    match choice {
        1 => {
            let plaintext = get_input("Enter plaintext as number: ");
            let cipher_key = get_input("Enter cipher key as number: ");
            println!("--- Encryption ---");
            des(plaintext, cipher_key, false);
        }
        2 => {
            let ciphertext = get_input("Enter ciphertext as number: ");
            let cipher_key = get_input("Enter cipher key as number: ");
            println!("--- Decryption ---");
            des(ciphertext, cipher_key, true);
        }
        3 => {
            print_debug();
        }
        _ => {
            println!("Exiting program!");
        }
    }
}

fn print_debug() {
    /*
     * Print the permutations for debugging purposes
     */
    println!("Initial Permutation: {:?}", IP);
}

fn get_input(prompt: &str) -> u64 {
    loop {
        println!("{}", prompt);
        io::stdout().flush().unwrap(); // Flush stdout to display the prompt before read_line

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        match choice.trim().parse::<u64>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
            }
        };
    }
}

fn split_u64_to_u32_halves(input: u64) -> (u32, u32) {
    // shifts the bits of the input to the right by 32 positions, isolating the upper 32 bits
    let left_half = (input >> 32) as u32;
    println!("Left split : {:032b}", left_half);
    // simply casts the u64 input to a u32, isolating the lower 32 bits
    let right_half = input as u32;
    println!("Right split: {:032b}", right_half);
    (left_half, right_half)
}

fn key_generation(key: u64) -> [u64; 16] {
    println!("--- Key Generation ---");
    let mut round_keys: [u64; 16] = [0; 16]; // Initialize an array to store round keys.

    // Step 1: Permuted Choice One (PC1)
    let permuted_choice_one: u64 = permute_u64(&PC1, key);

    // Step 2: Split into left and right halves
    let mut left = (permuted_choice_one >> 28) as u32;
    let mut right = (permuted_choice_one & 0x0FFFFFFF) as u32;
    println!("Left key   : {:028b}", left);
    println!("Right key  : {:028b}", right);

    // Step 3: 16 rounds of key generation
    const NUMBER_OF_SHIFTS: [usize; 16] = [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1];
    for i in 0..16 {
        println!("- Keygen Round: {} ", i + 1);
        // Step 3.1: Left shift for the round
        right = right.rotate_left(NUMBER_OF_SHIFTS[i] as u32);
        left = left.rotate_left(NUMBER_OF_SHIFTS[i] as u32);

        // Step 3.2: Permuted Choice Two (PC2)
        let combined: u64 = (left as u64) << 28 | right as u64;
        let round_key: u64 = permute_u64(&PC2, combined);

        // Step 3.3: Store the round key
        round_keys[i] = round_key;
        println!("Round key   : {:048b}", round_key);
    }
    round_keys
}

fn des(plaintext: u64, cipher_key: u64, reverse: bool) {
    println!("DES: {} with key: {}", plaintext, cipher_key);

    // Step 1: Initial Permutation
    let permuted: u64 = permute_u64(&IP, plaintext);

    // Step 2: Split into left and right halves
    let (mut left, mut right) = split_u64_to_u32_halves(permuted);

    // Step 3: Key generation
    let mut round_keys: [u64; 16] = key_generation(cipher_key);

    // Reverse the round keys if decrypting
    if reverse {
        round_keys.reverse();
    }

    // Step 4: 16 rounds
    println!("--- 16 Rounds ---");
    for i in 0..16 {
        println!("- Round {} ", i + 1);
        let prev_right: u32 = right;
        // Step 4.1: Expansion
        let expanded_right: u64 = expand(right);

        // Step 4.2: XOR with round key
        let xor_result: u64 = expanded_right ^ round_keys[i];
        println!("XOR result: {:048b}", xor_result);

        // Step 4.3: S-box substitution
        let sbox_result: u32 = 1; // placeholder, but TODO: substitute(xor_result);

        // Step 4.4: Permutation
        let permuted_s_box: u32 = permute_u32(&PF, sbox_result);

        // Step 4.5: XOR with left half and update halves
        right = left ^ permuted_s_box;
        left = prev_right;
    }

    // Step 5: Final Permutation
    println!("--- Final Permutation ---");
    let final_permuted: u64 = permute_u64(&FP, (left as u64) << 32 | right as u64);
    println!("Final permute: {:064b}", final_permuted);
    println!("Result: {}", final_permuted);
}
