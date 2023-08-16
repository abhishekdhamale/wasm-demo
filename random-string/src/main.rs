use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn main() {
    let random_text: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(10)
    .map(char::from)
    .collect();
    println!("{}", random_text);
}
