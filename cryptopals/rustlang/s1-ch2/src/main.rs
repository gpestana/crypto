fn main() {
    let hex1 = String::from("1c0111001f010100061a024b53535009181c");
    let hex2 = String::from("686974207468652062756c6c277320657965");
    let expected_result = String::from("746865206b696420646f6e277420706c6179");

    let result = xor(&hex1, &hex2);
    assert_eq!(result, expected_result);
}

fn xor(a: &str, b: &str) -> String {
    a.to_string()
}
