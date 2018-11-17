#[allow(dead_code)]
extern crate hex;

fn main() {
    let hex_word = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let b64_word = String::from("SSdtIGtpbGxpbmcgeW92ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")
        .into_bytes();

    let res = hex::decode(&hex_word).expect("err decoding");

    println!("{:?}", res);
    println!("{:?}", hex_word);

    assert_eq!(b64_word.len(), res.len());
    assert_eq!(b64_word, res);
}
