extern crate ch3_xor_cipher;

use ch3_xor_cipher::{decrypt, find_key_str};
use std::fs;
use std::str::from_utf8;
use std::vec::Vec;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_challenge() {
        let f = fs::read_to_string("ciphers.txt").expect("Err opening file");
        let mut searcher = f.split("\n");

        let mut current_min_chi = 0.0;
        let mut current_hex_string = Vec::new();
        let mut current_secret: u8 = 0;

        loop {
            match searcher.next() {
                Some(string) => {
                    let hex_str = hex::decode(&string).unwrap();
                    let (chi_min, secret) = find_key_str(hex_str.clone());

                    println!("{:?}", hex_str);
                    if chi_min < current_min_chi || current_min_chi == 0.0 {
                        let plaintext = decrypt(current_hex_string.clone(), current_secret);
                        println!("{:?}", from_utf8(&plaintext));

                        current_min_chi = chi_min;
                        current_hex_string = hex_str;
                        current_secret = secret;
                    }
                }

                None => break,
            }
        }
    }
}
