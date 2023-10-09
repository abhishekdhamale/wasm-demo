
use sha2::{Digest, Sha256};
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut hasher = Sha256::new();
    hasher.update(input.clone());
    let hash_result = hasher.finalize();
    println!("{:?} > SHA-256 {:x}", &input[..input.len()-1], hash_result);
}