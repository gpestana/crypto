#![allow(non_snake_case)]
use std::collections::HashMap;
use std::u16;

const BIT_MASK: u16 = 63; // used to clear 2 most significant bits when AND'ed with other u16

fn main() {
    let hex_string = String::from(
        "49276d206b696c6c696e6720796f757220627261696e206c696b65206\
         120706f69736f6e6f7573206d757368726f6f6d",
    );
    let b64_string =
        String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");

    // because rust treats str and String as UTF-8 encoded, we need to construct
    // an array with hexadecimal values where it is possible to apply bitwise
    // operations over the correct hexadecimal representation
    let mut hex: [u16; 96] = [0; 96];
    for (i, chunk) in hex_string.chars().enumerate() {
        hex[i] = u16::from_str_radix(&chunk.to_string(), 16).expect("nok");
    }

    let mut b64: [u8; 64] = [0; 64];
    let mut j = 0;
    for (i, _e) in hex.iter().enumerate() {
        if i % 3 == 0 {
            let (lb, rb) = hex_to_b64(hex[i], hex[i + 1], hex[i + 2]);
            b64[j] = lb;
            b64[j + 1] = rb;
            j = j + 2;
        }
    }

    let done = b64_to_string(b64);
    assert_eq!(done, b64_string);
}

// converts hexadecimal to base 64: takes three bytes hexadecimal encoded and
// returns two base64 encoded bytes. returns each base64 word as u8
fn hex_to_b64(L: u16, M: u16, R: u16) -> (u8, u8) {
    let lb = ((L << 4) + M) >> 2;
    let rb = ((M << 4) + R) & BIT_MASK;
    (lb as u8, rb as u8)
}

// converst base 64 to string based on the encoding mapping
fn b64_to_string(b: [u8; 64]) -> String {
    // conversion map
    let map: HashMap<u8, String> = [
        (0, "A".to_string()),
        (1, "B".to_string()),
        (2, "C".to_string()),
        (3, "D".to_string()),
        (4, "E".to_string()),
        (5, "F".to_string()),
        (6, "G".to_string()),
        (7, "H".to_string()),
        (8, "I".to_string()),
        (9, "J".to_string()),
        (10, "K".to_string()),
        (11, "L".to_string()),
        (12, "M".to_string()),
        (13, "N".to_string()),
        (14, "O".to_string()),
        (15, "P".to_string()),
        (16, "Q".to_string()),
        (17, "R".to_string()),
        (18, "S".to_string()),
        (19, "T".to_string()),
        (20, "U".to_string()),
        (21, "V".to_string()),
        (22, "W".to_string()),
        (23, "X".to_string()),
        (24, "Y".to_string()),
        (25, "Z".to_string()),
        (26, "a".to_string()),
        (27, "b".to_string()),
        (28, "c".to_string()),
        (29, "d".to_string()),
        (30, "e".to_string()),
        (31, "f".to_string()),
        (32, "g".to_string()),
        (33, "h".to_string()),
        (34, "i".to_string()),
        (35, "j".to_string()),
        (36, "k".to_string()),
        (37, "l".to_string()),
        (38, "m".to_string()),
        (39, "n".to_string()),
        (40, "o".to_string()),
        (41, "p".to_string()),
        (42, "q".to_string()),
        (43, "r".to_string()),
        (44, "s".to_string()),
        (45, "t".to_string()),
        (46, "u".to_string()),
        (47, "v".to_string()),
        (48, "w".to_string()),
        (49, "x".to_string()),
        (50, "y".to_string()),
        (51, "z".to_string()),
        (52, "0".to_string()),
        (53, "1".to_string()),
        (54, "2".to_string()),
        (55, "3".to_string()),
        (56, "4".to_string()),
        (57, "5".to_string()),
        (58, "6".to_string()),
        (59, "7".to_string()),
        (60, "8".to_string()),
        (61, "9".to_string()),
        (62, "+".to_string()),
        (63, "/".to_string()),
    ]
        .iter()
        .cloned()
        .collect();

    let mut s = String::new();
    for k in b.iter() {
        s = s + &map[k]
    }
    s
}
