// Fraction structure with one method: Add

use num_bigint::BigInt;
use std::ops::{Add, Mul};

#[derive(Debug)]
pub struct Fraction {
    pub numerator: BigInt,
    pub denominator: BigInt,
}


impl Fraction {
    pub fn add(mut self, other: &Fraction) -> Fraction {  
        let mut first = self.numerator.mul(other.denominator.clone());
        let second = other.numerator.clone().mul(self.denominator.clone());

        let den = self.denominator.mul(other.denominator.clone());

        first = first.add(second);

        self = Fraction {numerator: first, denominator: den};
        self
    }
}
