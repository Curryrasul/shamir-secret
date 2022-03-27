use num_bigint::{BigInt, RandBigInt, ToBigInt};
use rand::rngs::OsRng;
use std::convert::TryInto;

// Key splitting mode
pub fn split(key: BigInt, n: u8, t: u8) -> Vec<BigInt> {
    // Random coeffs vector
    let mut coefs: Vec<BigInt> = Vec::new();

    let mut rng = OsRng;
    for _ in 0..(t - 1) {
        coefs.push(rng.gen_bigint(256));
    }

    let mut res: Vec<BigInt> = Vec::new();

    for i in 1..=n {
        let mut secret = key.clone();

        for (j, coef) in coefs.iter().enumerate() {
            let mut temp_big = coef.clone();
            let mut i_big = i.to_bigint().unwrap();

            let j_u32: u32 = (j + 1).try_into().unwrap();

            i_big = i_big.pow(j_u32);
            temp_big *= i_big;

            secret += temp_big;
        }

        res.push(secret);
    }

    res
}
