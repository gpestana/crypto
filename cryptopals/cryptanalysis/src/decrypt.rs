pub fn single_xor_decrypt(cipher: std::vec::Vec<u8>, secret: u8) -> std::vec::Vec<u8> {
    let mut plaintext = cipher.clone();
    for (i, b) in plaintext.clone().iter().enumerate() {
        plaintext[i] = b ^ secret;
    }
    plaintext
}
