#![allow(unused)]
extern crate sha2;

use sha2::{Sha256, Digest};

const BLOCK_SIZE: usize = 8; // block size is 64 bits (8bytes)
const OPAD: u8 = 0x5c;
const IPAD: u8 = 0x36;

fn hash(s: String, m: String) -> str {
    // define k: hash(m) iff len(m) > block_size | k
    let k = {
        if m.len() > BLOCK_SIZE {
            // return hash
            let mut hasher = Sha256::default();
            hasher.input(&m)
        } 
        m[..]
    };
    k
}

fn main() {
    let k = String::from("key");
    let m = String::from("large_message");

    // tests
    let ms = m.clone();
    let mb = ms.as_bytes();
    println!("{}, {:?}", ms.len(), mb);
    // ---

    println!("HMAC-SHA256 implementation ({}, {}, {})", OPAD, IPAD, hash(k, m));
}


#[cfg(test)]
mod tests {
    #[test]
    fn case_empty() {
        use hash;

        let expect_hmac = "b613679a0814d9ec772f95d778c35fc5ff1697c493715653c6c712144292c5ad".to_string();
        let res_hmac = hash("".to_string(), "".to_string());
        assert_eq!(expect_hmac, res_hmac);
    }

    #[test]
    fn case_message() {
        use hash;

        let expect_hmac = "f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8".to_string();
        let res_hmac = hash("key".to_string(), "The quick brown fox jumps over the lazy dog".to_string());
        assert_eq!(expect_hmac, res_hmac);
    }

}
