#[allow(dead_code)]
#[macro_use]
extern crate num;

use num::pow::pow;
use std::collections::HashMap;

extern crate hex;

fn main() {
    let mut eng_freq: HashMap<u8, f64> = HashMap::new();

    eng_freq = hashmap![97 => 0.08167, 98: => 0.01492, 99 => 0.02782, 100 => 0.04253, 101 => 0.12702, => 102: 0.02228, 103 => 0.02015, 104 => 0.06094, 105 => 0.06966, 106 => 0.00153, 107 => 0.00772, 108 => 0.04025, 109 => 0.02406, 110 => 0.06749, 111=> 0.07507, 112 => 0.01929, 113 => 0.00095, 114 => 0.05987, 115 => 0.06327, 116 => .09056, 117 => 0.02758, 118 => 0.00978, 119 => 0.02360, 120 => 0.00150, 121=> 0.01974, 122 => 0.00074];

    let encoded_str =
        String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let hex = match hex::decode(&encoded_str) {
        Ok(h) => h,
        Err(e) => panic!("{}", e),
    };

    // calculate char occurrences
    let mut occs = HashMap::new();
    for b in hex.iter() {
        match occs.clone().get(&b) {
            Some(c) => occs.insert(b, c + 1),
            None => occs.insert(b, 1),
        };
    }

    println!("{:?} {}", encoded_str, encoded_str.len());
    println!("{:?} {}", hex, hex.len());
    println!("{:?}", occs);

    // tests all possible one-byte secret
    for i in 97..123 {
        let mut pt = vec![0; hex.len()];
        for (j, b) in hex.iter().enumerate() {
            pt[j] = *b ^ i as u8;
        }
        let pt_occ = calculate_occ(&pt);
        let chi = chi_square(&pt_occ, &eng_freq);
        println!("{:?}", chi);
    }
}

fn chi_square(c: &HashMap<u8, f64>, e: &HashMap<u8, f64>) -> f64 {
    if c.clone().keys().len() != e.clone().keys().len() {
        panic!("chi_square: both arrays should have same length");
    }

    let mut v: f64 = 0.0;
    for k in c.clone().keys() {
        v = v + ((pow(c[k] - e[k], 2) / e[k]) as f64);
    }
    return v;
}

fn calculate_occ(p: &[u8]) -> HashMap<u8, f64> {
    let mut occs: HashMap<u8, f64> = HashMap::new();
    occs
}

#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
        let mut map = ::std::collections::HashMap::new();
        $( map.insert($key, $val); )*
        map
    }}
}
