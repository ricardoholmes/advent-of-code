use std::ops::{Add, Sub, Not, Mul, Div, Rem, Neg, Shl, Shr};
use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct I256 {
    pub data: [u64; 4]
}

impl I256 {
    pub const ZERO: Self = Self {
        data: [0; 4]
    };

    pub fn is_positive(&self) -> bool {
        self.data[0] >> 63 == 0
    }

    pub fn is_negative(&self) -> bool {
        self.data[0] >> 63 == 1
    }

    pub fn abs(&self) -> Self {
        if self.is_negative() {
            -(*self)
        } else {
            *self
        }
    }

    // integer square root via binary search (https://en.wikipedia.org/wiki/Integer_square_root)
    pub fn isqrt(&self) -> Option<Self> {
        if self.is_negative() {
            return None;
        }

        let one = 1u128.into();

        let mut l = Self::ZERO;
        let mut r = *self + one;

        while l != r - one {
            let m = (l + r) >> 1; // div by 2

            if m * m <= *self {
                l = m;
            }
            else {
                r = m;
            }
        }

        Some(l)
    }
}

impl Default for I256 {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Not for I256 {
    type Output = I256;

    fn not(self) -> Self::Output {
        let mut data = self.data;
        for i in data.iter_mut() {
            *i = !(*i);
        }

        Self {
            data
        }
    }
}

impl Shl<u8> for I256 {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        let mut out = [0, 0, 0, 0];
        let mut carry = 0;
        for i in (0..4).rev() {
            let x = ((self.data[i] as u128) << rhs) + (carry as u128);
            out[i] = x as u64;
            carry = (x >> 64) as u64;
        }

        Self {
            data: out
        }
    }
}

impl Shr<u8> for I256 {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        let mut out = [0, 0, 0, 0];
        let mut carry = 0;
        for i in 0..4 {
            let x = (((self.data[i] as u128) << 64) >> rhs) + (carry as u128);
            out[i] = (x >> 64) as u64;
            carry = x as u64;
        }

        Self {
            data: out
        }
    }
}

impl Neg for I256 {
    type Output = I256;

    fn neg(self) -> Self::Output {
        let one = 1u128.into();
        !self + one // 2s complement = (invert x) + 1
    }
}

impl From<u128> for I256 {
    fn from(value: u128) -> Self {
        Self {
            data: [0, 0, (value >> 64) as u64, value as u64]
        }
    }
}

impl From<i128> for I256 {
    fn from(value: i128) -> Self {
        if value == 0 {
            Self::ZERO
        } else if value.is_positive() {
            (value as u128).into()
        } else {
            println!("NEIOEANOIDNS");
            let value = value.abs().into();
            Self::ZERO - value
        }
    }
}

impl Into<i128> for I256 {
    fn into(self) -> i128 {
        let abs_value = self.abs().data;
        if abs_value[0] > 0 || abs_value[1] > 0 || abs_value[2] >> 63 == 1 {
            panic!("Integer overflow when converting to less-precised type.");
        }
        let num = (abs_value[2] as u128) << 64 + (abs_value[3] as u128);

        if self.is_negative() {
            -(num as i128)
        } else {
            num as i128
        }
    }
}

impl PartialOrd for I256 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        let (a, b) = match (self.is_positive(), other.is_positive()) {
            (false, false) => ((-*other).data, (-*self).data),
            (true, true) => (self.data, other.data),
            (a, b) => return a.partial_cmp(&b),
        };

        for i in (0..4).rev() {
            match a[i].cmp(&b[i]) {
                Ordering::Equal => continue,
                ordering => return Some(ordering),
            }
        }

        None
    }
}

impl Add<I256> for I256 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut num = [0; 4];

        let mut carry = 0;
        for i in (0..4).rev() {
            let x = (self.data[i] as u128) + (other.data[i] as u128) + carry;
            carry = x >> 64;
            num[i] = x as u64;
        }

        Self {
            data: num
        }
    }
}

impl Sub<I256> for I256 {
    type Output = Self;

    fn sub(self, rhs: I256) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul<I256> for I256 {
    type Output = I256;

    fn mul(self, rhs: I256) -> Self::Output {
        // let mut total = Self::ZERO;
        
        // // pos * pos = pos and neg * neg = pos
        // let positive = self.is_positive() == rhs.is_positive();

        // let multiplicand = self.abs();
        // let mut multiplier = rhs.abs();
        // let mut n = 1;

        // while multiplier != Self::ZERO {
        //     if multiplier.data[3] & 1 == 1 {
        //         total = total + (multiplicand << n);
        //     }
        //     multiplier = multiplier >> 1;
        //     n += 1;
        // }

        // if positive {
        //     total
        // } else {
        //     -total
        // }

        let mut total = Self::ZERO;
        
        // pos * pos = pos and neg * neg = pos
        let positive = self.is_positive() == rhs.is_positive();

        let one = 1u128.into();
        let multiplicand = self.abs();
        let mut multiplier = rhs.abs();

        while multiplier > Self::ZERO {
            total = total + multiplicand;
            multiplier = multiplier - one;
        }

        if positive {
            total
        } else {
            -total
        }
    }
}

impl Div<I256> for I256 {
    type Output = I256;

    fn div(self, rhs: I256) -> Self::Output {
        let same_sign = self.is_positive() == rhs.is_positive();

        let goal = self.abs();
        let increment = rhs.abs();

        let mut num = Self::ZERO;
        let mut count = Self::ZERO;

        let one: I256 = 1u128.into();

        while num < goal {
            num = num + increment;
            if num <= goal {
                count = count + one;
            }
        }

        if same_sign {
            count
        } else {
            -count
        }
    }
}

impl Rem<I256> for I256 {
    type Output = I256;

    fn rem(self, rhs: I256) -> Self::Output {
        let div = self / rhs;

        self - (div * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u128_0() {
        let x: I256 = 0u128.into();
        assert_eq!(x, I256 { data: [0;4] });
    }

    #[test]
    fn from_u128_1() {
        let x: I256 = 1u128.into();
        assert_eq!(x, I256 { data: [0,0,0,1] });
    }

    #[test]
    fn from_u128_max() {
        let x: I256 = u128::MAX.into();
        assert_eq!(x, I256 { data: [0,0,u64::MAX,u64::MAX] });
    }

    #[test]
    fn from_i128_10() {
        let x: I256 = 10i128.into();
        assert_eq!(x, I256 { data: [0,0,0,10] });
    }

    #[test]
    fn from_i128_neg_10() {
        let x: I256 = (-10 as i128).into();
        assert_eq!(x, I256 { data: [u64::MAX, u64::MAX, u64::MAX, u64::MAX - 10 + 1] });
    }

    #[test]
    fn from_i128_max() {
        let x: I256 = i128::MAX.into();
        assert_eq!(x, I256 { data: [0,0,u64::MAX >> 1,u64::MAX] });
    }

    #[test]
    fn from_i128_min() {
        let x: I256 = i128::MIN.into();
        assert_eq!(x, I256 { data: [u64::MAX,u64::MAX,1<<63,0] });
    }
}

