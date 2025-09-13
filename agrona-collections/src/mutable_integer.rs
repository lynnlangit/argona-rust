use core::fmt;
use core::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MutableInteger {
    value: i32,
}

impl MutableInteger {
    pub const fn new(value: i32) -> Self {
        Self { value }
    }

    pub const fn get(&self) -> i32 {
        self.value
    }

    pub fn set(&mut self, value: i32) {
        self.value = value;
    }

    pub fn increment(&mut self) -> i32 {
        self.value += 1;
        self.value
    }

    pub fn decrement(&mut self) -> i32 {
        self.value -= 1;
        self.value
    }

    pub fn add_and_get(&mut self, delta: i32) -> i32 {
        self.value += delta;
        self.value
    }

    pub fn get_and_add(&mut self, delta: i32) -> i32 {
        let old_value = self.value;
        self.value += delta;
        old_value
    }

    pub fn get_and_increment(&mut self) -> i32 {
        let old_value = self.value;
        self.value += 1;
        old_value
    }

    pub fn get_and_decrement(&mut self) -> i32 {
        let old_value = self.value;
        self.value -= 1;
        old_value
    }

    pub fn compare_and_set(&mut self, expected: i32, new_value: i32) -> bool {
        if self.value == expected {
            self.value = new_value;
            true
        } else {
            false
        }
    }

    pub fn get_and_set(&mut self, new_value: i32) -> i32 {
        let old_value = self.value;
        self.value = new_value;
        old_value
    }
}

impl Default for MutableInteger {
    fn default() -> Self {
        Self::new(0)
    }
}

impl From<i32> for MutableInteger {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

impl From<MutableInteger> for i32 {
    fn from(mi: MutableInteger) -> Self {
        mi.value
    }
}

impl fmt::Display for MutableInteger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Add<i32> for MutableInteger {
    type Output = i32;

    fn add(self, rhs: i32) -> Self::Output {
        self.value + rhs
    }
}

impl AddAssign<i32> for MutableInteger {
    fn add_assign(&mut self, rhs: i32) {
        self.value += rhs;
    }
}

impl Sub<i32> for MutableInteger {
    type Output = i32;

    fn sub(self, rhs: i32) -> Self::Output {
        self.value - rhs
    }
}

impl SubAssign<i32> for MutableInteger {
    fn sub_assign(&mut self, rhs: i32) {
        self.value -= rhs;
    }
}

impl Mul<i32> for MutableInteger {
    type Output = i32;

    fn mul(self, rhs: i32) -> Self::Output {
        self.value * rhs
    }
}

impl MulAssign<i32> for MutableInteger {
    fn mul_assign(&mut self, rhs: i32) {
        self.value *= rhs;
    }
}

impl Div<i32> for MutableInteger {
    type Output = i32;

    fn div(self, rhs: i32) -> Self::Output {
        self.value / rhs
    }
}

impl DivAssign<i32> for MutableInteger {
    fn div_assign(&mut self, rhs: i32) {
        self.value /= rhs;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MutableLong {
    value: i64,
}

impl MutableLong {
    pub const fn new(value: i64) -> Self {
        Self { value }
    }

    pub const fn get(&self) -> i64 {
        self.value
    }

    pub fn set(&mut self, value: i64) {
        self.value = value;
    }

    pub fn increment(&mut self) -> i64 {
        self.value += 1;
        self.value
    }

    pub fn decrement(&mut self) -> i64 {
        self.value -= 1;
        self.value
    }

    pub fn add_and_get(&mut self, delta: i64) -> i64 {
        self.value += delta;
        self.value
    }

    pub fn get_and_add(&mut self, delta: i64) -> i64 {
        let old_value = self.value;
        self.value += delta;
        old_value
    }

    pub fn get_and_increment(&mut self) -> i64 {
        let old_value = self.value;
        self.value += 1;
        old_value
    }

    pub fn get_and_decrement(&mut self) -> i64 {
        let old_value = self.value;
        self.value -= 1;
        old_value
    }

    pub fn compare_and_set(&mut self, expected: i64, new_value: i64) -> bool {
        if self.value == expected {
            self.value = new_value;
            true
        } else {
            false
        }
    }

    pub fn get_and_set(&mut self, new_value: i64) -> i64 {
        let old_value = self.value;
        self.value = new_value;
        old_value
    }
}

impl Default for MutableLong {
    fn default() -> Self {
        Self::new(0)
    }
}

impl From<i64> for MutableLong {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl From<MutableLong> for i64 {
    fn from(ml: MutableLong) -> Self {
        ml.value
    }
}

impl fmt::Display for MutableLong {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutable_integer() {
        let mut mi = MutableInteger::new(42);
        assert_eq!(mi.get(), 42);

        mi.set(100);
        assert_eq!(mi.get(), 100);

        assert_eq!(mi.increment(), 101);
        assert_eq!(mi.get(), 101);

        assert_eq!(mi.get_and_increment(), 101);
        assert_eq!(mi.get(), 102);

        assert_eq!(mi.add_and_get(10), 112);
        assert_eq!(mi.get(), 112);

        assert!(mi.compare_and_set(112, 200));
        assert_eq!(mi.get(), 200);

        assert!(!mi.compare_and_set(100, 300));
        assert_eq!(mi.get(), 200);
    }

    #[test]
    fn test_mutable_long() {
        let mut ml = MutableLong::new(1234567890123456789);
        assert_eq!(ml.get(), 1234567890123456789);

        ml.set(-9876543210987654321);
        assert_eq!(ml.get(), -9876543210987654321);

        assert_eq!(ml.increment(), -9876543210987654320);
        assert_eq!(ml.get(), -9876543210987654320);
    }

    #[test]
    fn test_arithmetic_operations() {
        let mut mi = MutableInteger::new(10);

        mi += 5;
        assert_eq!(mi.get(), 15);

        mi -= 3;
        assert_eq!(mi.get(), 12);

        mi *= 2;
        assert_eq!(mi.get(), 24);

        mi /= 4;
        assert_eq!(mi.get(), 6);
    }
}