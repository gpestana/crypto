#![allow(unused)]
extern crate sha2;

use std::str;
use sha2::{Sha256, Digest};
use sha2::digest::generic_array::GenericArray;
use sha2::digest::generic_array::typenum::U32;

const BLOCK_SIZE: usize = 8; // block size is 64 bits (8bytes)
const OPAD: u8 = 0x5c;
const IPAD: u8 = 0x36;

fn hash(s: &String, m: &String) -> Vec<u8>  {
    // define k: hash(m) iff len(m) > block_size | k
    let mut k: [u8; 64] = [0; 64];
    if m.len() > BLOCK_SIZE {
        let mut hasher = Sha256::default();
        hasher.input(&m);
        // TODO: how to idiomatically copy the generic array into the array?
        // maybe use Vec<u8> instead of array?
        for (i, res) in hasher.result().into_iter().enumerate() {
           k[i] = res;
        }
    };
    
    // also transforms into Vec<u8>
    let oh = add_pad(OPAD, k);
    let ih_tmp = add_pad(IPAD, k);
    
    let ma: Vec<u8> = m.as_bytes().iter().map(|x| *x).collect();
    let ih: Vec<u8> = [ih_tmp.to_vec(), ma]
        .iter()
        .flat_map(|a| a.iter())
        .cloned()
        .collect();

    // hash oh and ih
    let doh = digest256(oh);
    let dih = digest256(ih);
    
    // hmac = H(oh || ih))
    let mut f_res: Vec<u8> = vec![0; 32];
    // TODO: use map instead of for loop
    for (i, r) in doh.clone().into_iter().enumerate() {
        f_res[i] = doh[i] ^ dih[i]; 
    }

    f_res
}

fn digest256(m: Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::default();
    hasher.input(m);
    hasher.result().to_vec()
}

fn add_pad(pad: u8, k: [u8; 64]) -> Vec<u8> {
    let mut hashpad: [u8; 64] = [pad; 64];
    let ok = k.clone();
    for (i, r) in ok.into_iter().enumerate() {
        match ok[i] {
            0 => hashpad[i] = ok[i] ^ pad,
            _ => hashpad[i] = *r,
        }
    }
    hashpad.to_vec()
}

fn to_string(vec: Vec<u8>) -> String {
    String::from_utf8_lossy(&vec).to_string()
}

fn main() {
    let k = String::from("key");
    let m = String::from("The quick brown fox jumps over the lazy dog");

    let h = hash(&k, &m);

    println!("HMAC-SHA256 implementation ({}, {})", k, m);
    println!("{:?}", h);;
    println!("{:?}", to_string(h));
}


#[cfg(test)]
mod tests {
    fn to_string(vec: Vec<u8>) -> String {
        String::from_utf8_lossy(&vec).to_string()
    }

    #[test]
    fn case_empty() {
        use hash;

        let expect_hmac = "b613679a0814d9ec772f95d778c35fc5ff1697c493715653c6c712144292c5ad".to_string();
        let res_hmac = hash(&"".to_string(), &"".to_string());
        assert_eq!(expect_hmac, to_string(res_hmac));
    }

    #[test]
    fn case_message() {
        use hash;

        let expect_hmac = "f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8".to_string();
        let res_hmac = hash(&"key".to_string(), &"The quick brown fox jumps over the lazy dog".to_string());
        assert_eq!(expect_hmac, to_string(res_hmac));
    }
}
