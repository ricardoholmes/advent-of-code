use std::ops::{Mul, Div, Rem};

pub fn lcm<T>(a: T, b: T) -> T where T: Copy + Mul<Output=T> + PartialEq + Div<Output=T> + Rem<Output=T> + Default {
    a * b / gcd(a, b)
}

pub fn gcd<T>(a: T, b: T) -> T where T: Copy + PartialEq + Rem<Output=T> + Default {
    if b == T::default() {
        a
    }
    else {
        gcd(b, a % b)
    }
}
