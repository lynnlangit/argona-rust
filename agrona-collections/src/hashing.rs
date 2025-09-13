use core::hash::{Hash, Hasher};

#[inline(always)]
pub fn fast_int_hash(value: i32) -> u32 {
    let mut x = value as u32;
    x = ((x >> 16) ^ x).wrapping_mul(0x45d9f3b);
    x = ((x >> 16) ^ x).wrapping_mul(0x45d9f3b);
    x = (x >> 16) ^ x;
    x
}

#[inline(always)]
pub fn fast_long_hash(value: i64) -> u32 {
    let mut x = value as u64;
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x = x ^ (x >> 31);
    x as u32
}

#[inline(always)]
pub fn mix_hash(hash: u32) -> u32 {
    let mut h = hash;
    h ^= h >> 16;
    h = h.wrapping_mul(0x85ebca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2ae35);
    h ^= h >> 16;
    h
}

#[inline(always)]
pub fn compound_hash(a: i32, b: i32) -> u32 {
    let mut result = 1u32;
    result = 31u32.wrapping_mul(result).wrapping_add(fast_int_hash(a));
    result = 31u32.wrapping_mul(result).wrapping_add(fast_int_hash(b));
    result
}

pub struct FastHasher {
    state: u64,
}

impl FastHasher {
    pub const fn new() -> Self {
        Self { state: 0 }
    }
}

impl Default for FastHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for FastHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        for chunk in bytes.chunks(8) {
            let mut value = 0u64;
            for (i, &byte) in chunk.iter().enumerate() {
                value |= (byte as u64) << (i * 8);
            }
            self.state = self.state.wrapping_add(fast_long_hash(value as i64) as u64);
        }
    }

    fn write_u8(&mut self, i: u8) {
        self.state = self.state.wrapping_add(i as u64);
    }

    fn write_u16(&mut self, i: u16) {
        self.state = self.state.wrapping_add(i as u64);
    }

    fn write_u32(&mut self, i: u32) {
        self.state = self.state.wrapping_add(fast_int_hash(i as i32) as u64);
    }

    fn write_u64(&mut self, i: u64) {
        self.state = self.state.wrapping_add(fast_long_hash(i as i64) as u64);
    }

    fn write_usize(&mut self, i: usize) {
        self.write_u64(i as u64);
    }

    fn write_i8(&mut self, i: i8) {
        self.write_u8(i as u8);
    }

    fn write_i16(&mut self, i: i16) {
        self.write_u16(i as u16);
    }

    fn write_i32(&mut self, i: i32) {
        self.state = self.state.wrapping_add(fast_int_hash(i) as u64);
    }

    fn write_i64(&mut self, i: i64) {
        self.state = self.state.wrapping_add(fast_long_hash(i) as u64);
    }

    fn write_isize(&mut self, i: isize) {
        self.write_i64(i as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_int_hash() {
        let hash1 = fast_int_hash(42);
        let hash2 = fast_int_hash(42);
        let hash3 = fast_int_hash(43);

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_fast_long_hash() {
        let hash1 = fast_long_hash(1234567890123456789);
        let hash2 = fast_long_hash(1234567890123456789);
        let hash3 = fast_long_hash(1234567890123456788);

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_compound_hash() {
        let hash1 = compound_hash(1, 2);
        let hash2 = compound_hash(1, 2);
        let hash3 = compound_hash(2, 1);

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }
}