use base16ct;
use md5::{Digest, Md5};
fn main() {
    let my_number = find_additional_hash_numbers("iwrupvqb");
    println!("{my_number}")
}
fn find_additional_hash_numbers(secret_key: &str) -> u32 {
    let mut result: u32 = 0;
    for number_to_find in 0..10000000 {
        let try_hash = number_to_find.to_string();
        let try_hash = secret_key.to_owned() + &try_hash;
        let mut hasher = Md5::new();
        hasher.update(try_hash.as_bytes());
        // Note that calling `finalize()` consumes hasher
        let hash = hasher.finalize();
        let mut buf = [0u8; 32];
        let res: &str = base16ct::lower::encode_str(&hash, &mut buf).unwrap();
        if res.starts_with("00000") {
            result = number_to_find;
            break;
        }
    }
    result
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn secret_key_abcdef() {
        let additional_numbers = find_additional_hash_numbers("abcdef");
        assert_eq!(additional_numbers, 609043);
    }

    #[test]
    fn secret_key_pqrstuv() {
        let additional_numbers = find_additional_hash_numbers("pqrstuv");
        assert_eq!(additional_numbers, 1048970);
    }
}
