
use sha2::{Digest, Sha512};
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut hasher = Sha512::new();
    hasher.update(input.clone());
    let hash_result = hasher.finalize();
    println!("{:?} > SHA-512 {:x}", &input[..input.len()-1], hash_result);
}