use std::io;
use num_bigint::{BigInt, ToBigInt};
use std::ops::{Mul, Div};

mod fraction;

// Key recovering mode
pub fn recover() {
    let mut t = String::new();
    io::stdin()
        .read_line(&mut t)
        .expect("Failed to read T");
    let t: usize = t.trim().parse().expect("Non number value (T)");

    let mut key_parts: Vec<BigInt> = Vec::new();
    let mut x: Vec<i128> = Vec::new();

    for _ in 0..t {
        // Parsing key_parts
        let mut s = String::new();
        io::stdin()
            .read_line(&mut s)
            .expect("Failed to read key");

        let mut s = s.split_whitespace();

        let temp = match s.next() {
            Some(val) => val,
            _ => panic!()
        };

        let temp: i128 = temp.parse().unwrap();
        x.push(temp);

        let key_part = match s.next() {
            Some(val) => val,
            _ => panic!()
        };

        let key_part = key_part.trim().trim_start_matches("0x");
        let key_part = BigInt::parse_bytes(key_part.as_bytes(), 16).expect("Failed to convert");

        key_parts.push(key_part);
    }

    let mut fractions: Vec<fraction::Fraction> = Vec::new();

    for j in 0..t {
        let mut num: i128 = 1;
        let mut den: i128 = 1;

        for m in 0..t {
            if j == m {
                continue
            }

            num *= -x[m];
            den *= x[j] - x[m];
        }

        let mut num_big = num.to_bigint().unwrap();
        let den_big = den.to_bigint().unwrap();

        let key_part = match key_parts.get(j) {
            Some(b) => b,
            None => panic!(),
        };
        num_big = num_big.mul(key_part);

        fractions.push(fraction::Fraction {numerator: num_big, denominator: den_big});
    }

    let mut key_res = fraction::Fraction {numerator: 0.to_bigint().unwrap(),
                                          denominator: 1.to_bigint().unwrap()};

    for f in &fractions {
        key_res = key_res.add(f);
    }

    let key_res = key_res.numerator.div(key_res.denominator);

    println!("\n{:x}", key_res)
}
