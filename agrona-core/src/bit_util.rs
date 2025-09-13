use core::mem;

pub const SIZE_OF_BYTE: usize = mem::size_of::<u8>();
pub const SIZE_OF_BOOL: usize = mem::size_of::<bool>();
pub const SIZE_OF_CHAR: usize = mem::size_of::<char>();
pub const SIZE_OF_I16: usize = mem::size_of::<i16>();
pub const SIZE_OF_U16: usize = mem::size_of::<u16>();
pub const SIZE_OF_I32: usize = mem::size_of::<i32>();
pub const SIZE_OF_U32: usize = mem::size_of::<u32>();
pub const SIZE_OF_F32: usize = mem::size_of::<f32>();
pub const SIZE_OF_I64: usize = mem::size_of::<i64>();
pub const SIZE_OF_U64: usize = mem::size_of::<u64>();
pub const SIZE_OF_F64: usize = mem::size_of::<f64>();
pub const SIZE_OF_USIZE: usize = mem::size_of::<usize>();

pub const CACHE_LINE_LENGTH: usize = 64;

#[inline(always)]
pub const fn is_power_of_two(value: u64) -> bool {
    value > 0 && (value & (value - 1)) == 0
}

#[inline(always)]
pub const fn next_power_of_two(mut value: u32) -> u32 {
    value -= 1;
    value |= value >> 1;
    value |= value >> 2;
    value |= value >> 4;
    value |= value >> 8;
    value |= value >> 16;
    value + 1
}

#[inline(always)]
pub const fn align(size: usize, alignment: usize) -> usize {
    (size + alignment - 1) & !(alignment - 1)
}

#[inline(always)]
pub fn is_aligned(ptr: *const u8, alignment: usize) -> bool {
    (ptr as usize) & (alignment - 1) == 0
}

#[inline(always)]
pub fn fast_hex_digit(value: u8) -> u8 {
    static HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";
    HEX_DIGITS[(value & 0x0F) as usize]
}

#[inline(always)]
pub fn from_hex_digit(digit: u8) -> Result<u8, ()> {
    match digit {
        b'0'..=b'9' => Ok(digit - b'0'),
        b'a'..=b'f' => Ok(digit - b'a' + 10),
        b'A'..=b'F' => Ok(digit - b'A' + 10),
        _ => Err(()),
    }
}

#[inline(always)]
pub fn number_of_leading_zeros_u32(value: u32) -> u32 {
    value.leading_zeros()
}

#[inline(always)]
pub fn number_of_leading_zeros_u64(value: u64) -> u32 {
    value.leading_zeros()
}

#[inline(always)]
pub fn number_of_trailing_zeros_u32(value: u32) -> u32 {
    value.trailing_zeros()
}

#[inline(always)]
pub fn number_of_trailing_zeros_u64(value: u64) -> u32 {
    value.trailing_zeros()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_two() {
        assert!(is_power_of_two(1));
        assert!(is_power_of_two(2));
        assert!(is_power_of_two(4));
        assert!(is_power_of_two(8));
        assert!(is_power_of_two(1024));
        assert!(!is_power_of_two(0));
        assert!(!is_power_of_two(3));
        assert!(!is_power_of_two(5));
        assert!(!is_power_of_two(1023));
    }

    #[test]
    fn test_next_power_of_two() {
        assert_eq!(next_power_of_two(1), 1);
        assert_eq!(next_power_of_two(2), 2);
        assert_eq!(next_power_of_two(3), 4);
        assert_eq!(next_power_of_two(5), 8);
        assert_eq!(next_power_of_two(1023), 1024);
        assert_eq!(next_power_of_two(1024), 1024);
    }

    #[test]
    fn test_align() {
        assert_eq!(align(1, 4), 4);
        assert_eq!(align(4, 4), 4);
        assert_eq!(align(5, 4), 8);
        assert_eq!(align(7, 8), 8);
        assert_eq!(align(9, 8), 16);
    }
}