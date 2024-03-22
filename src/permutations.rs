// Permutations
// According to standard: https://en.wikipedia.org/wiki/DES_supplementary_material

// Initial permutation
pub const IP: [u64; 64] = [
    58, 50, 42, 34, 26, 18, 10, 2, 60, 52, 44, 36, 28, 20, 12, 4, 62, 54, 46, 38, 30, 22, 14, 6,
    64, 56, 48, 40, 32, 24, 16, 8, 57, 49, 41, 33, 25, 17, 9, 1, 59, 51, 43, 35, 27, 19, 11, 3, 61,
    53, 45, 37, 29, 21, 13, 5, 63, 55, 47, 39, 31, 23, 15, 7,
];

// Final permutation
// Inverse of the Initial Permutation
pub const FP: [u64; 64] = [
    40, 8, 48, 16, 56, 24, 64, 32, 39, 7, 47, 15, 55, 23, 63, 31, 38, 6, 46, 14, 54, 22, 62, 30,
    37, 5, 45, 13, 53, 21, 61, 29, 36, 4, 44, 12, 52, 20, 60, 28, 35, 3, 43, 11, 51, 19, 59, 27,
    34, 2, 42, 10, 50, 18, 58, 26, 33, 1, 41, 9, 49, 17, 57, 25,
];

// Expansion
pub const E: [u64; 48] = [
    32, 1, 2, 3, 4, 5, 4, 5, 6, 7, 8, 9, 8, 9, 10, 11, 12, 13, 12, 13, 14, 15, 16, 17, 16, 17, 18,
    19, 20, 21, 20, 21, 22, 23, 24, 25, 24, 25, 26, 27, 28, 29, 28, 29, 30, 31, 32, 1,
];

// Permutation in F
pub const PF: [u64; 32] = [
    16, 7, 20, 21, 29, 12, 28, 17, 1, 15, 23, 26, 5, 18, 31, 10, 2, 8, 24, 14, 32, 27, 3, 9, 19,
    13, 30, 6, 22, 11, 4, 25,
];

// Permuted Choice One (PC1)
// B64 to B56
pub const PC1: [u64; 56] = [
    57, 49, 41, 33, 25, 17, 9, 1, 58, 50, 42, 34, 26, 18, 10, 2, 59, 51, 43, 35, 27, 19, 11, 3, 60,
    52, 44, 36, 63, 55, 47, 39, 31, 23, 15, 7, 62, 54, 46, 38, 30, 22, 14, 6, 61, 53, 45, 37, 29,
    21, 13, 5, 28, 20, 12, 4,
];

// Permuted Choice Two (PC2)
// B56 to B48
pub const PC2: [u64; 48] = [
    14, 17, 11, 24, 1, 5, 3, 28, 15, 6, 21, 10, 23, 19, 12, 4, 26, 8, 16, 7, 27, 20, 13, 2, 41, 52,
    31, 37, 47, 55, 30, 40, 51, 45, 33, 48, 44, 49, 39, 56, 34, 53, 46, 42, 50, 36, 29, 32,
];

pub fn permute_u64(permutation_table: &[u64], data: u64) -> u64 {
    let mut permuted_data: u64 = 0;
    for i in 0..permutation_table.len() {
        // Get the i-th bit from the original data
        let data_bit = (data >> i) & 1;
        // Set the i-th bit in the permuted data according to the permutation table
        permuted_data |= data_bit << (permutation_table[i] - 1);
    }
    println!("Original Data: {:064b}", data);
    println!("Permuted Data: {:064b}", permuted_data);
    permuted_data
}

pub fn permute_u32(permutation_table: &[u64], data: u32) -> u32 {
    let mut permuted_data: u32 = 0;
    for i in 0..permutation_table.len() {
        // Get the i-th bit from the original data
        let data_bit = (data >> i) & 1;
        // Set the i-th bit in the permuted data according to the permutation table
        permuted_data |= data_bit << (permutation_table[i] - 1) as u32;
    }
    println!("Original Data: {:032b}", data);
    println!("Permuted Data: {:032b}", permuted_data);
    permuted_data
}

pub fn reverse_permute_u64(permutation_table: &[u64], permuted_data: u64) -> u64 {
    let mut data: u64 = 0;
    for i in 0..permutation_table.len() {
        // Get the i-th bit from the permuted data according to the permutation table
        let data_bit = (permuted_data >> (permutation_table[i] - 1)) & 1;
        // Set the i-th bit in the original data
        data |= data_bit << i;
    }
    println!("Original Permuted Data: {:064b}", permuted_data);
    println!("Reverse Permuted Data: {:064b}", data);
    data
}

pub fn reverse_permute_u32(permutation_table: &[u64], permuted_data: u32) -> u32 {
    let mut data: u32 = 0;
    for i in 0..permutation_table.len() {
        // Get the i-th bit from the permuted data according to the permutation table
        let data_bit = (permuted_data >> (permutation_table[i] - 1) as u32) & 1;
        // Set the i-th bit in the original data
        data |= data_bit << i;
    }
    println!("Original Permuted Data: {:032b}", permuted_data);
    println!("Reverse Permuted Data: {:032b}", data);
    data
}
