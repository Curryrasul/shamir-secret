use num_bigint::BigInt;
use std::io;

pub fn parse_split() -> (BigInt, u8, u8) {
    let mut key = String::new();
    io::stdin().read_line(&mut key).expect("Failed to read key");
    let key = key.trim().trim_start_matches("0x");
    let key = BigInt::parse_bytes(key.as_bytes(), 16).expect("Failed to convert");

    let mut parameters = String::new();
    io::stdin()
        .read_line(&mut parameters)
        .expect("Failed to read parameters");

    let values: Vec<u8> = parameters
        .split_whitespace()
        .map(|x| x.parse::<u8>())
        .collect::<Result<Vec<u8>, _>>()
        .expect("Failed to parse N and T");
    assert!(values.len() == 2);

    let n = values[0];
    let t = values[1];

    (key, n, t)
}

pub fn parse_recover() -> (usize, Vec<(i128, BigInt)>) {
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Failed to read T");
    let t: usize = t.trim().parse().expect("Non number value T");

    let mut key_parts: Vec<BigInt> = Vec::new();
    let mut x: Vec<i128> = Vec::new();

    for _ in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read key");

        let args: Vec<&str> = s.split_whitespace().into_iter().collect();

        x.push(args[0].parse().expect("Number expected"));

        let key_part = args[1].trim().trim_start_matches("0x");
        let key_part = BigInt::parse_bytes(key_part.as_bytes(), 16).expect("Failed to convert");

        key_parts.push(key_part);
    }

    let v: Vec<(i128, BigInt)> = x.into_iter().zip(key_parts.into_iter()).collect();

    (t, v)
}
