use crate::error::{AgronaError, Result};
use byteorder::{ByteOrder, LittleEndian};
use core::cmp::Ordering;

pub trait DirectBuffer: Send + Sync {
    fn capacity(&self) -> usize;

    fn check_limit(&self, limit: usize) -> Result<()> {
        if limit > self.capacity() {
            return Err(AgronaError::IndexOutOfBounds {
                index: 0,
                length: limit,
                capacity: self.capacity(),
            });
        }
        Ok(())
    }

    fn bounds_check(&self, index: usize, length: usize) -> Result<()> {
        let capacity = self.capacity();
        if index + length > capacity {
            return Err(AgronaError::IndexOutOfBounds {
                index,
                length,
                capacity,
            });
        }
        Ok(())
    }

    fn get_u8(&self, index: usize) -> Result<u8>;
    fn get_i8(&self, index: usize) -> Result<i8>;

    fn get_u16(&self, index: usize) -> Result<u16> {
        self.get_u16_with_order(index, LittleEndian)
    }

    fn get_u16_with_order<B: ByteOrder>(&self, index: usize, _byte_order: B) -> Result<u16>;

    fn get_i16(&self, index: usize) -> Result<i16> {
        self.get_i16_with_order(index, LittleEndian)
    }

    fn get_i16_with_order<B: ByteOrder>(&self, index: usize, _byte_order: B) -> Result<i16>;

    fn get_u32(&self, index: usize) -> Result<u32> {
        self.get_u32_with_order(index, LittleEndian)
    }

    fn get_u32_with_order<B: ByteOrder>(&self, index: usize, _byte_order: B) -> Result<u32>;

    fn get_i32(&self, index: usize) -> Result<i32> {
        self.get_i32_with_order(index, LittleEndian)
    }

    fn get_i32_with_order<B: ByteOrder>(&self, index: usize, _byte_order: B) -> Result<i32>;

    fn get_u64(&self, index: usize) -> Result<u64> {
        self.get_u64_with_order(index, LittleEndian)
    }

    fn get_u64_with_order<B: ByteOrder>(&self, index: usize, _byte_order: B) -> Result<u64>;

    fn get_i64(&self, index: usize) -> Result<i64> {
        self.get_i64_with_order(index, LittleEndian)
    }

    fn get_i64_with_order<B: ByteOrder>(&self, index: usize, _byte_order: B) -> Result<i64>;

    fn get_f32(&self, index: usize) -> Result<f32> {
        self.get_f32_with_order(index, LittleEndian)
    }

    fn get_f32_with_order<B: ByteOrder>(&self, index: usize, _byte_order: B) -> Result<f32>;

    fn get_f64(&self, index: usize) -> Result<f64> {
        self.get_f64_with_order(index, LittleEndian)
    }

    fn get_f64_with_order<B: ByteOrder>(&self, index: usize, _byte_order: B) -> Result<f64>;

    fn get_bytes(&self, index: usize, dst: &mut [u8]) -> Result<()>;

    fn get_bytes_into(&self, index: usize, dst: &mut [u8], offset: usize, length: usize) -> Result<()> {
        if offset + length > dst.len() {
            return Err(AgronaError::IndexOutOfBounds {
                index: offset,
                length,
                capacity: dst.len(),
            });
        }
        let mut temp_dst = vec![0u8; length];
        self.get_bytes(index, &mut temp_dst)?;
        dst[offset..offset + length].copy_from_slice(&temp_dst);
        Ok(())
    }

    fn parse_natural_i32_ascii(&self, index: usize, length: usize) -> Result<i32>;
    fn parse_natural_i64_ascii(&self, index: usize, length: usize) -> Result<i64>;
    fn parse_i32_ascii(&self, index: usize, length: usize) -> Result<i32>;
    fn parse_i64_ascii(&self, index: usize, length: usize) -> Result<i64>;

    fn get_string_ascii(&self, index: usize) -> Result<String> {
        let length = self.get_u32(index)? as usize;
        self.get_string_ascii_with_length(index + 4, length)
    }

    fn get_string_ascii_with_length(&self, index: usize, length: usize) -> Result<String>;

    fn get_string_utf8(&self, index: usize) -> Result<String> {
        let length = self.get_u32(index)? as usize;
        self.get_string_utf8_with_length(index + 4, length)
    }

    fn get_string_utf8_with_length(&self, index: usize, length: usize) -> Result<String>;
}

impl<T: DirectBuffer + ?Sized> PartialEq for T {
    fn eq(&self, other: &Self) -> bool {
        if self.capacity() != other.capacity() {
            return false;
        }

        for i in 0..self.capacity() {
            if self.get_u8(i).unwrap_or(0) != other.get_u8(i).unwrap_or(0) {
                return false;
            }
        }
        true
    }
}

impl<T: DirectBuffer + ?Sized> Eq for T {}

impl<T: DirectBuffer + ?Sized> PartialOrd for T {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: DirectBuffer + ?Sized> Ord for T {
    fn cmp(&self, other: &Self) -> Ordering {
        let min_len = self.capacity().min(other.capacity());

        for i in 0..min_len {
            let self_byte = self.get_u8(i).unwrap_or(0);
            let other_byte = other.get_u8(i).unwrap_or(0);
            match self_byte.cmp(&other_byte) {
                Ordering::Equal => continue,
                other => return other,
            }
        }

        self.capacity().cmp(&other.capacity())
    }
}