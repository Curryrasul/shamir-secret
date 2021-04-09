use clap::{Arg, App};
use std::collections::hash_map::Entry::Occupied;
use std::io;
use num_bigint::{BigUint, RandBigInt, ToBigUint};
use rand::rngs::OsRng;
use std::ops::{Add, Mul};
use std::convert::TryInto;

fn main() {
    // Flags
    // cargo run --release -- --help to see info
    let mut matches = App::new("Shamir's Secret Sharing")
        .arg(Arg::with_name("split")
             .long("split")
             .takes_value(false)
             .help("Start program in split mode"))
        .arg(Arg::with_name("recover")
             .long("recover")
             .takes_value(false)
             .help("Start program in recover mode"))
        .get_matches();

    if matches.args.len() > 1 {
        panic!("Too much args");
    }

    if let Occupied(_) = matches.args.entry("split") {
        split();
        return
    }

    if let Occupied(_) = matches.args.entry("recover") {
        recover();
        return 
    }

    panic!("Not enough args!");
}


// Key splitting mode 
fn split() {
    // Parsing 256 bit number in Hex
    let mut key = String::new();
    io::stdin()
        .read_line(&mut key)
        .expect("Failed to read key");
    let key = key.trim().trim_start_matches("0x");
    let key = BigUint::parse_bytes(key.as_bytes(), 16).expect("Failed to convert");

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

    let mut coefs: Vec<BigUint> = Vec::new();

    let mut rng = OsRng;
    for _ in 0..(t - 1) {
        coefs.push(rng.gen_biguint(32));
    }

    // println!("{:?}", coefs);

    for i in 1..=n {
        let mut secret = key.clone();
        print!("{} ", i);
        for j in 0..coefs.len() {
            let mut temp_big = coefs[j].clone();
            let mut i_big = i.to_biguint().unwrap();
            let j_u32: u32 = (j + 1).try_into().unwrap();
            
            i_big = i_big.pow(j_u32);
            temp_big = temp_big.mul(i_big);

            secret = secret.add(temp_big);
        }
        println!("0x{:x}", secret);
    }
}

// Key recovering mode
fn recover() {
    let mut t = String::new();
    io::stdin()
        .read_line(&mut t)
        .expect("Failed to read T");
    let t: usize = t.trim().parse().expect("Non number value (T)");

    let mut key_parts: Vec<BigUint> = Vec::new();

    for _ in 0..t {
        // Parsing key_parts
        let mut key_part = String::new();
        io::stdin()
            .read_line(&mut key_part)
            .expect("Failed to read key");
        let key_part = key_part.trim().trim_start_matches("0x");
        let key_part = BigUint::parse_bytes(key_part.as_bytes(), 16).expect("Failed to convert");

        key_parts.push(key_part);
    }

    let mut secret = 0.to_biguint().unwrap();

    
}
