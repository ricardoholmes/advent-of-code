use std::ops::{Mul, Div, Rem};

#[allow(dead_code)]
pub fn lcm<T>(a: T, b: T) -> T where T: Copy + Mul<Output=T> + PartialEq + Div<Output=T> + Rem<Output=T> + Default {
    a * b / gcd(a, b)
}

#[allow(dead_code)]
pub fn gcd<T>(a: T, b: T) -> T where T: Copy + PartialEq + Rem<Output=T> + Default {
    if b == T::default() {
        a
    }
    else {
        gcd(b, a % b)
    }
}
