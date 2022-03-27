use num_bigint::{BigInt, ToBigInt};
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone)]
pub(super) struct Fraction {
    numerator: BigInt,
    denominator: BigInt,
}

impl Default for Fraction {
    fn default() -> Self {
        Self {
            numerator: 0.to_bigint().unwrap(),
            denominator: 1.to_bigint().unwrap(),
        }
    }
}

impl Add for Fraction {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, other: Self) -> Self::Output {
        let mut numerator = self.numerator.clone() * other.denominator.clone();
        let second = other.numerator.clone() * self.denominator.clone();

        let denominator = self.denominator * other.denominator;

        numerator += second;

        Self {
            numerator,
            denominator,
        }
    }
}

impl AddAssign for Fraction {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Fraction {
    pub(super) fn new(numerator: BigInt, denominator: BigInt) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    pub(super) fn get_result(self) -> BigInt {
        self.numerator / self.denominator
    }
}
