extern crate cryptanalysis;
extern crate hex;

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
    let mut current_secret: u8 = 0;
    let mut current_plaintext: Vec<u8> = Vec::new();

    // can this be done with map and be functional?
    loop {
        match searcher.next() {
            Some(string) => {
                let string_hex = match hex::decode(&string) {
                    Ok(h) => h,
                    Err(e) => panic!(e),
                };

                for key in 0..255 {
                    let mut plaintext_candidate =
                        single_xor_decrypt(Vec::from(string_hex.clone()), key);

                    let char_occ_map = utf8_occurrencies_map(&plaintext_candidate);
                    let fm = eng_frequencies_map();

                    let chi = chi_square(&char_occ_map, &fm);
                    if chi < current_smaller_chi || current_smaller_chi == 0.0 {
                        current_smaller_chi = chi.clone();
                        current_string = string.to_string();
                        current_secret = key.clone();
                        current_plaintext = plaintext_candidate.clone();
                    }
                }
            }
            None => break,
        };
    }

    let decoded_string: String = current_plaintext
        .into_iter()
        .map(|i| i.to_string())
        .collect::<String>();

    println!(
        "\n{} {}: {:?} (encrypted: {:?})",
        current_smaller_chi, current_secret, decoded_string, current_string
    );
}
