use std::fmt;

const PUB_DEFAULT: u32 = 65537;

pub struct RSAPair {
    pub_key: u32,
    priv_key: u32,
}

impl RSAPair {
    pub fn gen() -> RSAPair {
        let (p, q) = generate_prime();

        let n = p * q;
        let tot = (p - 1) * (q - 1);

        println!("n: {}, tot: {}", n, tot);

        let pub_key = generate_pub_key(tot, PUB_DEFAULT);
        return RSAPair {
            pub_key: pub_key,
            priv_key: extended_euclidean_algorithm(pub_key, tot),
        };
    }
}

impl fmt::Display for RSAPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pub_key: {}\npriv_key: {}", self.pub_key, self.priv_key)
    }
}

fn generate_prime() -> (u32, u32) {
    (11, 13)
}

fn generate_pub_key(totient: u32, def_pubkey: u32) -> u32 {
    // TODO: check if gdc(tot, PUB_DEFAULT) != 1. if that's the case, pick
    // another pub_key

    //def_pubkey
    7
}

// calculate multiplicative inverse (MIV) so that: `e * MIV = 1 mod tot`, using
// extended euclidean algorithm
fn extended_euclidean_algorithm(e: u32, tot: u32) -> u32 {
    let mut t = 0;
    let mut r = tot;
    let mut newt = 1;
    let mut newr = e;
    while newr != 0 {
        println!("{} ", newr);
        let quotient = r % newr;
        t = newt;
        newt = t - quotient * newt;
        r = newr;
        newr = r - quotient * newr;
    }

    if r > 1 {
        panic!("{} is not invertible as mod {} (r == {})", e, tot, r);
    }

    if t < 0 {
        t = t + tot;
    }

    t
}
