extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

fn is_valid_hash(hash: &str) -> bool {
    match hash[..5].parse() {
        Ok(val) => return val == 0,
        Err(e) => return false,
    }
}

fn main() {
    let input = "pqrstuv104897";
    let mut md5 = Md5::new();
    md5.input_str(input);
    println!("{}", is_valid_hash(&md5.result_str()));
}
