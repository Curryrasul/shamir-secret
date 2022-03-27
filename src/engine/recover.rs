use num_bigint::{BigInt, ToBigInt};

use crate::engine::Fraction;

pub fn recover(t: usize, v: Vec<(i128, BigInt)>) -> BigInt {
    let mut fractions: Vec<Fraction> = Vec::new();

    for j in 0..t {
        let mut num: i128 = 1;
        let mut den: i128 = 1;

        for m in 0..t {
            if j == m {
                continue;
            }

            num *= -v[m].0;
            den *= v[j].0 - v[m].0;
        }

        let mut num_big = num.to_bigint().unwrap();
        let den_big = den.to_bigint().unwrap();

        let key_part = v[j].1.clone();
        num_big *= key_part;

        fractions.push(Fraction::new(num_big, den_big));
    }

    let mut key_res = Fraction::default();

    for f in fractions {
        key_res += f;
    }

    key_res.get_result()
}
