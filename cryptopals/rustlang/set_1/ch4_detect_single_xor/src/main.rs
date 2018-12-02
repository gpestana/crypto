extern crate cryptanalysis;

use cryptanalysis::decrypt::single_xor_decrypt;
use cryptanalysis::frequency_analysis::{chi_square, eng_frequencies_map, utf8_occurrencies_map};
use std::fs;
use std::str::from_utf8;
use std::vec::Vec;

fn main() {
    let f = fs::read_to_string("ciphers.txt").expect("Err opening file");
    let mut searcher = f.split("\n");

    let mut current_smaller_chi = 0.0;
    let mut current_string = String::new();
    let mut current_secret: u8 = 10;
    let mut current_plaintext: Vec<u8> = Vec::new();

    // can this be done with map and be functional?
    loop {
        match searcher.next() {
            Some(string) => {
                println!("decoding and processing {}", string);
                for key in 56..90 {
                    let string = string.to_string();
                    let mut plaintext_candidate =
                        single_xor_decrypt(Vec::from(string.clone()), key);
                    let char_occ_map = utf8_occurrencies_map(&plaintext_candidate);

                    let chi = chi_square(&char_occ_map, &eng_frequencies_map());
                    if chi < current_smaller_chi || current_smaller_chi == 0.0 {
                        current_smaller_chi = chi;
                        current_string = string;
                        current_secret = key;
                        current_plaintext = plaintext_candidate;
                    }
                }
            }
            None => break,
        };
    }

    println!(
        "\n{} {}: {} (encrypted: {})",
        current_smaller_chi,
        current_secret,
        from_utf8(current_plaintext.as_slice()).unwrap(),
        current_string
    );
}
