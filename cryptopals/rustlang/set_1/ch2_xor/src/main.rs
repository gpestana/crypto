// this implementation uses external crate hex. check implementation of string to
// hex decoding implementation
extern crate hex;

fn main() {
    let hex1_str = String::from("1c0111001f010100061a024b53535009181c");
    let hex2_str = String::from("686974207468652062756c6c277320657965");
    let expected_result = String::from("746865206b696420646f6e277420706c6179");

    let h1 = match hex::decode(&hex1_str) {
        Ok(s) => s,
        Err(e) => panic!("panic hex decode: {} ", e),
    };

    let h2 = match hex::decode(&hex2_str) {
        Ok(s) => s,
        Err(e) => panic!("panic hex decode: {} ", e),
    };

    let result = xor(&h1, &h2);
    let result_str = hex::encode(result.as_slice());
    assert_eq!(result_str.as_bytes(), expected_result.as_bytes());
}

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    if a.len() != b.len() {
        panic!("a must be same length as b");
    }

    let mut v = vec![0; a.len()];
    for (i, _x) in a.iter().enumerate() {
        v[i] = a[i] ^ b[i];
    }
    v
}
