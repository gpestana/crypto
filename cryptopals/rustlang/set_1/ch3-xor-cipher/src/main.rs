#[allow(dead_code)]
extern crate hex;

use std::collections::HashMap;
use std::f64;
use std::str::from_utf8;
use std::vec::Vec;

const ASCII_A: u8 = 65;
const ASCII_Z: u8 = 90;

fn main() {}

pub fn find_key_str(hex_str: Vec<u8>) -> (f64, u8) {
    let mut chi_min: f64 = 0.0;
    let mut secret: u8 = 0;

    for i in ASCII_A..ASCII_Z {
        let plaintext_candidate = decrypt(hex_str.clone(), i);
        let plaintext_occ = calculate_occ(&plaintext_candidate);
        let chi = chi_square(&plaintext_occ, &eng_freq());

        // select chi_square minimum
        if chi_min == 0.0 || chi_min > chi {
            chi_min = chi;
            secret = i;
        };
    }
    (chi_min, secret)
}

fn decrypt(cipher: Vec<u8>, secret: u8) -> Vec<u8> {
    cipher.iter().map(|v| v ^ secret).collect()
}

fn chi_square(observed: &HashMap<u8, f64>, expected: &HashMap<u8, f64>) -> f64 {
    if observed.keys().len() != expected.keys().len() {
        panic!("chi_square: both arrays should have same length")
    }

    let mut chi: f64 = 0.0;
    for key in observed.keys() {
        let obs = observed.get(key).unwrap();
        let exp = expected.get(key).unwrap();

        if *exp != 0.0 as f64 {
            chi = chi + (f64::powi(obs - exp, 2) / exp);
        }
    }
    chi
}

fn calculate_occ(p: &[u8]) -> HashMap<u8, f64> {
    let mut occs: HashMap<u8, f64> = new_zero_hashmap();
    for b in p.iter() {
        let sum = match occs.get(b) {
            Some(s) => (s + 1.0) / p.len() as f64,
            None => 1.0 / p.len() as f64,
        };
        occs.insert(*b, sum);
    }
    occs
}

fn new_zero_hashmap() -> HashMap<u8, f64> {
    let mut hm: HashMap<u8, f64> = HashMap::new();
    for i in 0..255 {
        hm.insert(i, 0.0);
    }
    hm
}

fn eng_freq() -> HashMap<u8, f64> {
    let mut f: HashMap<u8, f64> = new_zero_hashmap();
    f.insert(97, 0.08167); //a
    f.insert(98, 0.01492);
    f.insert(99, 0.02782);
    f.insert(100, 0.04253);
    f.insert(101, 0.12702); //e
    f.insert(102, 0.02228);
    f.insert(103, 0.02015);
    f.insert(104, 0.06094); //h
    f.insert(105, 0.06966);
    f.insert(106, 0.00153);
    f.insert(107, 0.03872); //k
    f.insert(108, 0.04025);
    f.insert(109, 0.02406);
    f.insert(110, 0.06749);
    f.insert(111, 0.07507); //o
    f.insert(112, 0.01929);
    f.insert(113, 0.00095);
    f.insert(114, 0.05987); //r
    f.insert(115, 0.06327);
    f.insert(116, 0.09056);
    f.insert(117, 0.02758);
    f.insert(118, 0.00978);
    f.insert(119, 0.05360); //w
    f.insert(120, 0.00150);
    f.insert(121, 0.03974);
    f.insert(122, 0.00074);
    f
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    #[test]
    fn chi_square_test() {
        // expected
        let mut e: HashMap<u8, f64> = HashMap::new();
        e.insert(0, 10.0);
        e.insert(1, 10.0);
        e.insert(2, 10.0);
        e.insert(3, 10.0);
        e.insert(4, 10.0);

        // observation
        let mut o: HashMap<u8, f64> = HashMap::new();
        o.insert(0, 4.0);
        o.insert(1, 6.0);
        o.insert(2, 14.0);
        o.insert(3, 10.0);
        o.insert(4, 16.0);

        let chi = chi_square(&o, &e);
        assert_eq!(chi, 10.4);
    }

    #[test]
    fn test_challenge() {
        let solution = "Cooking MC\'s like a pound of bacon".to_string();

        let encoded_str =
            String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        let hex_str = hex::decode(&encoded_str).unwrap();

        let (_, secret) = find_key_str(hex_str.clone());

        let plaintext = decrypt(hex_str, secret);
        assert_eq!(from_utf8(&plaintext).unwrap(), solution);
    }
}
