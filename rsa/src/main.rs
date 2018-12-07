#![allow(unused)]
mod keys;

use keys::RSAPair;

fn main() {
    let pp_keys = RSAPair::gen();
    println!("{}", pp_keys);
}
