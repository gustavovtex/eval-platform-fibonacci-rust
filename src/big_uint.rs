use std::fmt;
use std::ops::Add;

/// One limb holds nine decimal digits, so printing needs no division of the whole number.
const BASE: u32 = 1_000_000_000;

/// A small unsigned big integer: little-endian base-10^9 limbs, no leading zero limbs, zero is empty.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BigUint {
    limbs: Vec<u32>,
}

impl BigUint {
    pub fn zero() -> Self {
        BigUint { limbs: Vec::new() }
    }
}

impl From<u64> for BigUint {
    fn from(mut value: u64) -> Self {
        let mut limbs = Vec::new();
        while value > 0 {
            limbs.push((value % BASE as u64) as u32);
            value /= BASE as u64;
        }
        BigUint { limbs }
    }
}

impl Add for &BigUint {
    type Output = BigUint;

    fn add(self, other: &BigUint) -> BigUint {
        let length = self.limbs.len().max(other.limbs.len());
        let mut limbs = Vec::with_capacity(length + 1);
        let mut carry = 0u32;
        for i in 0..length {
            let sum = self.limbs.get(i).copied().unwrap_or(0) + other.limbs.get(i).copied().unwrap_or(0) + carry;
            if sum >= BASE {
                limbs.push(sum - BASE);
                carry = 1;
            } else {
                limbs.push(sum);
                carry = 0;
            }
        }
        if carry > 0 {
            limbs.push(carry);
        }
        BigUint { limbs }
    }
}

impl fmt::Display for BigUint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.limbs.split_last() {
            None => write!(f, "0"),
            Some((top, rest)) => {
                write!(f, "{top}")?;
                for limb in rest.iter().rev() {
                    write!(f, "{limb:09}")?;
                }
                Ok(())
            }
        }
    }
}
