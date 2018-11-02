extern crate sha2;
extern crate hmac;
extern crate hex;

use sha2::{Sha256, Digest};

const BLOCK_SIZE: usize = 64;
const OPAD: u8 = 0x5c;
const IPAD: u8 = 0x36;

fn hash(s: &String, m: &String) -> Vec<u8>  {
    // convert message and secret to vec
    let m_vec: Vec<u8> = m.as_bytes().iter().map(|x| *x).collect();
    let s_vec: Vec<u8> = s.as_bytes().iter().map(|x| *x).collect();

    // k' = k iif k.len() < 64, otherwise k'= H(k)
    let mut k: Vec<u8> = vec![0; 64];
    if s_vec.len() > BLOCK_SIZE {
        let mut hasher = Sha256::default();
        hasher.input(&s_vec);
        k = hasher.result().to_vec();
    } else {
        // add s bytes to 0-padded k
        for (i, e) in s_vec.into_iter().enumerate() {
            k[i] = e;
        }
    }
    
    // H(opad ^ k') && H(ipad ^ k')
    let oh = add_pad(OPAD, &k);
    let ih = add_pad(IPAD, &k);
    
    // H(hi || m)
    let ih_m = digest256(concat(ih, m_vec));

    // H(oh || ih_m))
    let f_res = digest256(concat(oh, ih_m));

    f_res
}

// immutable concat of Vec (TODO: implement with MACRO)
fn concat(v1: Vec<u8>, v2: Vec<u8>) -> Vec<u8> {
    let mut v1_cp = v1.clone();
    let mut v2_cp = v2.clone();
    v1_cp.append(&mut v2_cp);
    v1_cp 
}

fn digest256(m: Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::default();
    hasher.input(m);
    let r = hasher.result().to_vec();
    r
}

fn add_pad(pad: u8, k: &Vec<u8>) -> Vec<u8> {
    let mut hashpad: Vec<u8> = vec![0; BLOCK_SIZE];
    for (i, _r) in k.clone().into_iter().enumerate() {
        hashpad[i] = k[i] ^ pad;
    }
    hashpad
}

fn main() {
    let k = String::from("key");
    let m = String::from("large_message");

    let h = hash(&k, &m);

    println!("HMAC_SHA256: 0x{}", hex::encode(h));
}


#[cfg(test)]
mod tests {
    use sha2::{Sha256};
    use hmac::{Hmac, Mac};

    type HmacSha256 = Hmac<Sha256>; // implementation of HMAC-256 for comparison

    fn real_hash(m: &String, k: &String) -> Vec<u8> {
        let mut mac = HmacSha256::new_varkey(k.as_bytes())
            .expect("HMAC can take key of any size");
        mac.input(m.as_bytes());
        mac.result().code().to_vec()
    }

    #[test]
    fn case_empty() {
        use hash;

        let k = String::from("");
        let m = String::from("");
        let res_hmac = hash(&k, &m);
        let expected_hmac = real_hash(&k, &m);
        assert_eq!(expected_hmac, res_hmac);
    }

    #[test]
    fn case_message() {
        use hash;

        let k = String::from("key");
        let m = String::from("The quick brown fox jumps over the lazy dog");
        let res_hmac = hash(&k, &m);
        let expected_hmac = real_hash(&k, &m);
        assert_eq!(expected_hmac, res_hmac);
    }
}
