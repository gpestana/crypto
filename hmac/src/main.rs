#![allow(unused)]
extern crate sha2;

use sha2::{Sha256, Digest};
use sha2::digest::generic_array::GenericArray;
use sha2::digest::generic_array::typenum::U32;

const BLOCK_SIZE: usize = 8; // block size is 64 bits (8bytes)
const OPAD: u8 = 0x5c;
const IPAD: u8 = 0x36;

fn hash(s: String, m: String) -> [u8; 64]  {
    // define k: hash(m) iff len(m) > block_size | k
    let mut k: [u8; 64] = [0; 64];
    if m.len() > BLOCK_SIZE {
        let mut hasher = Sha256::default();
        hasher.input(&m);
        // TODO: how to idiomatically copy the generic array into the array?
        for (i, res) in hasher.result().into_iter().enumerate() {
           k[i] = res;
        }
    };
    
    // TODO: refactor H(k ^ xpad)

    // oh = H(k ^ opad)
    let mut oh: [u8; 64] = [OPAD; 64];
    let ok = k.clone();
    for (i, r) in ok.into_iter().enumerate() {
        match ok[i] {
            0 => oh[i] = ok[i] ^ OPAD,
            _ => oh[i] = *r,
        }
    }

    // TODO: hashes oh
    
    // ih = H(k ^ ipad) || m
    let mut ih_tmp: [u8; 64] = [IPAD; 64];
    let ik = k.clone();
    for (i, r) in ik.into_iter().enumerate() {
        match ik[i] {
            0 => ih_tmp[i] = ik[i] ^ IPAD,
            _ => ih_tmp[i] = *r,
        }
    }
    
    // TODO: hashes ih 

    // concatenates with message
    // TODO: how to fit string message into [u8; 64] array?

    let conc_list = [ih_tmp, m.as_bytes()[..]];
    conc_list.iter().flat_map(|s| s.iter()).collect();

    
    // hmac = H(oh || ih))

    k
}

fn main() {
    let k = String::from("key");
    let m = String::from("large_message");

    // tests
    let h = hash(k, m);
    // ---

    println!("HMAC-SHA256 implementation ({}, {})", OPAD, IPAD);
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
