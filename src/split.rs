use std::io;
use rand::rngs::OsRng;
use num_bigint::{BigInt, RandBigInt, ToBigInt};
use std::ops::{Add, Mul};
use std::convert::TryInto;

// Key splitting mode 
pub fn split() {
    // Parsing 256 bit number in Hex
    let mut key = String::new();
    io::stdin()
        .read_line(&mut key)
        .expect("Failed to read key");
    let key = key.trim().trim_start_matches("0x");
    let key = BigInt::parse_bytes(key.as_bytes(), 16).expect("Failed to convert");

    // Parsing N - parts of a secret && T - how many parts need to open a secret
    let mut parameters = String::new();
    io::stdin()
        .read_line(&mut parameters)
        .expect("Failed to read parameters");

    let values = parameters
        .split_whitespace()
        .map(|x| x.parse::<u8>())
        .collect::<Result<Vec<u8>, _>>()
        .expect("Failed to parse N and T");
    assert!(values.len() == 2);

    let n = values[0];
    let t = values[1];

    // Random coeffs vector
    let mut coefs: Vec<BigInt> = Vec::new();

    let mut rng = OsRng;
    for _ in 0..(t - 1) {
        coefs.push(rng.gen_bigint(256));
    }

    println!();

    for i in 1..=n {
        let mut secret = key.clone();
        for j in 0..coefs.len() {
            let mut temp_big = coefs[j].clone();
            let mut i_big = i.to_bigint().unwrap();
            let j_u32: u32 = (j + 1).try_into().unwrap();
            
            i_big = i_big.pow(j_u32);
            temp_big = temp_big.mul(i_big);

            secret = secret.add(temp_big);
        }
        println!("{:x}", secret);
    }
}
