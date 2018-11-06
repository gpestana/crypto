// generates private and public key
//
// algorithm:
// 1) generates 2 large (1024 bits) random prime numbers, p and q, using the
//  miller-rabin algorithm
// 2) calculates modulus n = pq;
// 3) calculates totient of n, phi(n), using Euler's method
// 4) extract PublicKey
//  - calculate encryption exponent, e, so that gcd(e, phi(n)) = 1 (aka e is
//  coprime with phi(n)), which usually may be 65537. 
//  - the public key is phi(n) and e
// 5) extract PrivateKey
//  - 
pub fn generate() {
    let p = 11; // TODO: generate large prime 
    let q = 12; // TODO: generate large prime 

    let n = p * q;
    let tot = (p - 1 * (q - 1));
}

// generates random prime number using the miller-rabin primality test
fn generate_prime(nbits: int) {}
