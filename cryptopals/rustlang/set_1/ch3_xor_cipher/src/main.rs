#[allow(dead_code)]
use std::collections::HashMap;
use std::f64;
use std::str::from_utf8;

extern crate hex;

fn main() {
    let encoded_str =
        String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let hex = match hex::decode(&encoded_str) {
        Ok(h) => h,
        Err(e) => panic!("{}", e),
    };

    // tests all possible one-byte secret
    let mut chi_min: f64 = 0.0;
    let mut secret: u8 = 0;
    for i in 97..123 {
        let mut pt = vec![0; hex.len()];
        for (j, b) in hex.iter().enumerate() {
            pt[j] = *b ^ i as u8;
        }
        let pt_occ = calculate_occ(&pt);
        let chi = chi_square(&pt_occ, &eng_freq());

        if chi_min == 0.0 || chi_min > chi {
            chi_min = chi;
            secret = i;
        };

        println!("{} {}", i, chi);
    }
    println!("\nSecret: {} (chi square: {})", secret, chi_min);

    // decrypt using the found key
    let mut plaintext = hex.clone();
    for (i, b) in plaintext.clone().iter().enumerate() {
        plaintext[i] = b ^ secret;
    }
    println!("{:?}", from_utf8(plaintext.as_slice()).unwrap());
}

fn chi_square(c: &HashMap<u8, f64>, e: &HashMap<u8, f64>) -> f64 {
    if c.clone().keys().len() != e.clone().keys().len() {
        panic!(
            "chi_square: both arrays should have same length, {} != {}",
            c.clone().keys().len(),
            e.clone().keys().len()
        );
    }

    let mut v: f64 = 0.0;
    for k in c.clone().keys() {
        v = v + (f64::powi(c[k] - e[k], 2) / e[k]);
    }
    return v;
}

fn calculate_occ(p: &[u8]) -> HashMap<u8, f64> {
    let mut occs: HashMap<u8, f64> = HashMap::new();
    // init occs
    for i in 97..123 {
        occs.insert(i as u8, 0.0);
    }

    // calculate occurrences
    for b in p.iter() {
        let sum = match occs.get(b) {
            Some(s) => s + 1.0,
            None => 1.0,
        };
        if *b >= 97 && *b < 123 {
            occs.insert(*b, sum);
        }
    }

    occs
}

fn eng_freq() -> HashMap<u8, f64> {
    let mut f: HashMap<u8, f64> = HashMap::new();
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
