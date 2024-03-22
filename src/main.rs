use std::io;
use std::io::Write;

mod permutations;
use permutations::{
    permute_u32, permute_u64, reverse_permute_u32, reverse_permute_u64, E, FP, IP, PC1, PC2, PF,
};
mod sboxes;
//use sboxes::{};

fn main() {
    println!("Welcome to the Rust Encryption/Decryption program!");

    loop {
        println!();
        println!("Choose an option:");
        println!("1. Encrypt");
        println!("2. Decrypt");
        println!("3. Print Debug stuff");
        println!("Press any other number to exit.");
        println!();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                let plaintext = get_input("Enter plaintext as number: ");
                let cipher_key = get_input("Enter cipher key as number: ");
                encrypt(plaintext, cipher_key);
            }
            2 => {
                let ciphertext = get_input("Enter ciphertext as number: ");
                let cipher_key = get_input("Enter cipher key as number: ");
                decrypt(ciphertext, cipher_key);
            }
            3 => {
                print_debug();
            }
            _ => {
                println!("Exiting program!");
                break;
            }
        }
    }
}

fn print_debug() {
    /*
     * Print the permutations for debugging purposes
     */
    println!("Initial Permutation: {:?}", IP);
    println!("Final Permutation: {:?}", FP);
    println!("Expansion: {:?}", E);
    println!("Permutation in F: {:?}", PF);
    println!("Permuted Choice One (PC1): {:?}", PC1);
    println!("Permuted Choice Two (PC2): {:?}", PC2);
    println!();

    // Initialize the data array with consecutive integers starting from 1
    let mut data: [u64; 64] = [0; 64];
    for (index, element) in data.iter_mut().enumerate() {
        *element = (index + 1) as u64;
    }

    //let permuted_data: [u64; 64] = permute(&IP, &data);

    //reverse_permute(&IP, &permuted_data);
}

fn get_input(prompt: &str) -> u64 {
    loop {
        let mut input = String::new();
        print!("{}", prompt);
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        // Attempt to parse the input as a hexadecimal u64
        match u64::from_str_radix(input, 16) {
            Ok(parsed) => {
                println!("Your value: {}", parsed);
                return input.parse().unwrap();
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid hexadecimal number.");
            }
        }
    }
}

fn split_u64_to_u32_halves(input: u64) -> (u32, u32) {
    // shifts the bits of the input to the right by 32 positions, isolating the upper 32 bits
    let left_half = (input >> 32) as u32;
    println!("Upper half: {:032b}", left_half);
    // simply casts the u64 input to a u32, isolating the lower 32 bits
    let right_half = input as u32;
    println!("Lower half: {:032b}", right_half);
    (left_half, right_half)
}

fn key_generation(key: u64) -> [u64; 16] {
    let mut round_keys: [u64; 16] = [0; 16]; // Initialize an array to store round keys.

    // Step 1: Permuted Choice One (PC1)
    let mut permuted_choice_one: u64 = permute_u64(&PC1, key);

    // Step 2: Split into left and right halves
    let (mut left, mut right) = split_u64_to_u32_halves(permuted_choice_one);
    // TODO: convert to halves to 28 bits each?

    // Step 3: 16 rounds of key generation
    for i in 0..16 {
        println!("Keygen Round {} ", i + 1);
        // Step 3.1 Left shift for the round and mask

        // Step 3.2 Permuted Choice Two (PC2)
        let combined: u64 = 1; // placeholder for testing
        let mut round_key: u64 = permute_u64(&PC2, combined);

        // Step3.3 Store the round key

    }

    round_keys // Return the array of round keys.
}

fn encrypt(plaintext: u64, cipher_key: u64) {
    // Placeholder for encryption function
    println!("--- String represantation ---");
    println!("Encrypting: {} with key: {}", plaintext, cipher_key);

    // Step 1: Initial Permutation
    let permuted: u64 = permute_u64(&IP, plaintext);

    // Step 2: Split into left and right halves
    let (mut left, mut right) = split_u64_to_u32_halves(permuted);

    // Step 3: Key generation
    let round_keys: [u64; 16] = key_generation(cipher_key);

    // Step 4: 16 rounds
    for i in 0..16 {
        println!("Round {} ", i + 1);
        let prev_right: u32 = right;
        // Step 4.1: Expansion

        // Step 4.2: XOR with round key

        // Step 4.3: S-box substitution
        let sbox_result: u32 = 1; // placeholder for testing because sboxes not implemented yet

        // Step 4.4: Permutation
        let permuted_s_box: u32 = permute_u32(&PF, sbox_result);

        // Step 4.5: XOR with left half and update halves
        right = left ^ permuted_s_box;
        left = prev_right;
    }

    // Step 5: Final Permutation
    let final_permuted: u64 = permute_u64(&FP, (left as u64) << 32 | right as u64);
    println!("Final permuted: {:064b}", final_permuted);
}

fn decrypt(ciphertext: u64, cipher_key: u64) {
    println!("Not implemented");
}
