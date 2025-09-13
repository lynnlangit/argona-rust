use crate::buffer::{bounds_check, DirectBuffer, MutableBuffer, BOUNDS_CHECK_ENABLED};
use crate::error::{AgronaError, Result};
use byteorder::{ByteOrder, LittleEndian};
use core::ptr;
use core::slice;

#[repr(C)]
pub struct UnsafeBuffer {
    data: *mut u8,
    capacity: usize,
    owned: bool,
}

unsafe impl Send for UnsafeBuffer {}
unsafe impl Sync for UnsafeBuffer {}

impl UnsafeBuffer {
    pub fn new(capacity: usize) -> Result<Self> {
        if capacity == 0 {
            return Err(AgronaError::InvalidCapacity { capacity });
        }

        let layout = std::alloc::Layout::from_size_align(capacity, 64)
            .map_err(|_| AgronaError::InvalidCapacity { capacity })?;

        let data = unsafe { std::alloc::alloc(layout) };
        if data.is_null() {
            return Err(AgronaError::InvalidCapacity { capacity });
        }

        Ok(Self {
            data,
            capacity,
            owned: true,
        })
    }

    pub fn wrap(data: *mut u8, capacity: usize) -> Self {
        Self {
            data,
            capacity,
            owned: false,
        }
    }

    pub fn wrap_slice(slice: &mut [u8]) -> Self {
        Self {
            data: slice.as_mut_ptr(),
            capacity: slice.len(),
            owned: false,
        }
    }

    pub fn wrap_slice_immutable(slice: &[u8]) -> Self {
        Self {
            data: slice.as_ptr() as *mut u8,
            capacity: slice.len(),
            owned: false,
        }
    }

    #[inline(always)]
    fn check_bounds(&self, index: usize, length: usize) -> Result<()> {
        bounds_check(index, length, self.capacity)
    }

    #[inline(always)]
    unsafe fn get_unchecked<T: Copy>(&self, index: usize) -> T {
        ptr::read_unaligned(self.data.add(index) as *const T)
    }

    #[inline(always)]
    unsafe fn put_unchecked<T: Copy>(&mut self, index: usize, value: T) {
        ptr::write_unaligned(self.data.add(index) as *mut T, value);
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.data
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.data, self.capacity) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.data, self.capacity) }
    }
}

impl Drop for UnsafeBuffer {
    fn drop(&mut self) {
        if self.owned && !self.data.is_null() {
            let layout = std::alloc::Layout::from_size_align(self.capacity, 64).unwrap();
            unsafe { std::alloc::dealloc(self.data, layout) };
        }
    }
}

impl DirectBuffer for UnsafeBuffer {
    fn capacity(&self) -> usize {
        self.capacity
    }

    fn get_u8(&self, index: usize) -> Result<u8> {
        self.check_bounds(index, 1)?;
        Ok(unsafe { self.get_unchecked(index) })
    }

    fn get_i8(&self, index: usize) -> Result<i8> {
        self.check_bounds(index, 1)?;
        Ok(unsafe { self.get_unchecked(index) })
    }

    fn get_u16_with_order<B: ByteOrder>(&self, index: usize, _byte_order: B) -> Result<u16> {
        self.check_bounds(index, 2)?;
        let bytes = unsafe { slice::from_raw_parts(self.data.add(index), 2) };
        Ok(B::read_u16(bytes))
    }

    fn get_i16_with_order<B: ByteOrder>(&self, index: usize, _byte_order: B) -> Result<i16> {
        self.check_bounds(index, 2)?;
        let bytes = unsafe { slice::from_raw_parts(self.data.add(index), 2) };
        Ok(B::read_i16(bytes))
    }

    fn get_u32_with_order<B: ByteOrder>(&self, index: usize, _byte_order: B) -> Result<u32> {
        self.check_bounds(index, 4)?;
        let bytes = unsafe { slice::from_raw_parts(self.data.add(index), 4) };
        Ok(B::read_u32(bytes))
    }

    fn get_i32_with_order<B: ByteOrder>(&self, index: usize, _byte_order: B) -> Result<i32> {
        self.check_bounds(index, 4)?;
        let bytes = unsafe { slice::from_raw_parts(self.data.add(index), 4) };
        Ok(B::read_i32(bytes))
    }

    fn get_u64_with_order<B: ByteOrder>(&self, index: usize, _byte_order: B) -> Result<u64> {
        self.check_bounds(index, 8)?;
        let bytes = unsafe { slice::from_raw_parts(self.data.add(index), 8) };
        Ok(B::read_u64(bytes))
    }

    fn get_i64_with_order<B: ByteOrder>(&self, index: usize, _byte_order: B) -> Result<i64> {
        self.check_bounds(index, 8)?;
        let bytes = unsafe { slice::from_raw_parts(self.data.add(index), 8) };
        Ok(B::read_i64(bytes))
    }

    fn get_f32_with_order<B: ByteOrder>(&self, index: usize, _byte_order: B) -> Result<f32> {
        self.check_bounds(index, 4)?;
        let bytes = unsafe { slice::from_raw_parts(self.data.add(index), 4) };
        Ok(B::read_f32(bytes))
    }

    fn get_f64_with_order<B: ByteOrder>(&self, index: usize, _byte_order: B) -> Result<f64> {
        self.check_bounds(index, 8)?;
        let bytes = unsafe { slice::from_raw_parts(self.data.add(index), 8) };
        Ok(B::read_f64(bytes))
    }

    fn get_bytes(&self, index: usize, dst: &mut [u8]) -> Result<()> {
        self.check_bounds(index, dst.len())?;
        unsafe {
            ptr::copy_nonoverlapping(self.data.add(index), dst.as_mut_ptr(), dst.len());
        }
        Ok(())
    }

    fn parse_natural_i32_ascii(&self, index: usize, length: usize) -> Result<i32> {
        self.check_bounds(index, length)?;
        let slice = unsafe { slice::from_raw_parts(self.data.add(index), length) };

        let mut result = 0i32;
        for &byte in slice {
            if !(b'0'..=b'9').contains(&byte) {
                return Err(AgronaError::AsciiNumberFormat(
                    format!("Invalid digit: {}", byte as char)
                ));
            }
            result = result.checked_mul(10)
                .and_then(|r| r.checked_add((byte - b'0') as i32))
                .ok_or_else(|| AgronaError::AsciiNumberFormat("Number overflow".to_string()))?;
        }
        Ok(result)
    }

    fn parse_natural_i64_ascii(&self, index: usize, length: usize) -> Result<i64> {
        self.check_bounds(index, length)?;
        let slice = unsafe { slice::from_raw_parts(self.data.add(index), length) };

        let mut result = 0i64;
        for &byte in slice {
            if !(b'0'..=b'9').contains(&byte) {
                return Err(AgronaError::AsciiNumberFormat(
                    format!("Invalid digit: {}", byte as char)
                ));
            }
            result = result.checked_mul(10)
                .and_then(|r| r.checked_add((byte - b'0') as i64))
                .ok_or_else(|| AgronaError::AsciiNumberFormat("Number overflow".to_string()))?;
        }
        Ok(result)
    }

    fn parse_i32_ascii(&self, index: usize, length: usize) -> Result<i32> {
        self.check_bounds(index, length)?;
        if length == 0 {
            return Err(AgronaError::AsciiNumberFormat("Empty string".to_string()));
        }

        let slice = unsafe { slice::from_raw_parts(self.data.add(index), length) };
        let (negative, start_idx) = if slice[0] == b'-' {
            (true, 1)
        } else {
            (false, 0)
        };

        if start_idx >= length {
            return Err(AgronaError::AsciiNumberFormat("No digits found".to_string()));
        }

        let mut result = 0i32;
        for &byte in &slice[start_idx..] {
            if !(b'0'..=b'9').contains(&byte) {
                return Err(AgronaError::AsciiNumberFormat(
                    format!("Invalid digit: {}", byte as char)
                ));
            }
            result = result.checked_mul(10)
                .and_then(|r| r.checked_add((byte - b'0') as i32))
                .ok_or_else(|| AgronaError::AsciiNumberFormat("Number overflow".to_string()))?;
        }

        if negative {
            result = result.checked_neg()
                .ok_or_else(|| AgronaError::AsciiNumberFormat("Number overflow".to_string()))?;
        }

        Ok(result)
    }

    fn parse_i64_ascii(&self, index: usize, length: usize) -> Result<i64> {
        self.check_bounds(index, length)?;
        if length == 0 {
            return Err(AgronaError::AsciiNumberFormat("Empty string".to_string()));
        }

        let slice = unsafe { slice::from_raw_parts(self.data.add(index), length) };
        let (negative, start_idx) = if slice[0] == b'-' {
            (true, 1)
        } else {
            (false, 0)
        };

        if start_idx >= length {
            return Err(AgronaError::AsciiNumberFormat("No digits found".to_string()));
        }

        let mut result = 0i64;
        for &byte in &slice[start_idx..] {
            if !(b'0'..=b'9').contains(&byte) {
                return Err(AgronaError::AsciiNumberFormat(
                    format!("Invalid digit: {}", byte as char)
                ));
            }
            result = result.checked_mul(10)
                .and_then(|r| r.checked_add((byte - b'0') as i64))
                .ok_or_else(|| AgronaError::AsciiNumberFormat("Number overflow".to_string()))?;
        }

        if negative {
            result = result.checked_neg()
                .ok_or_else(|| AgronaError::AsciiNumberFormat("Number overflow".to_string()))?;
        }

        Ok(result)
    }

    fn get_string_ascii_with_length(&self, index: usize, length: usize) -> Result<String> {
        self.check_bounds(index, length)?;
        let slice = unsafe { slice::from_raw_parts(self.data.add(index), length) };

        for &byte in slice {
            if byte > 127 {
                return Err(AgronaError::AsciiNumberFormat("Non-ASCII character found".to_string()));
            }
        }

        Ok(String::from_utf8_lossy(slice).to_string())
    }

    fn get_string_utf8_with_length(&self, index: usize, length: usize) -> Result<String> {
        self.check_bounds(index, length)?;
        let slice = unsafe { slice::from_raw_parts(self.data.add(index), length) };
        let s = core::str::from_utf8(slice)?;
        Ok(s.to_string())
    }
}

impl MutableBuffer for UnsafeBuffer {
    fn set_memory(&mut self, index: usize, length: usize, value: u8) -> Result<()> {
        self.check_bounds(index, length)?;
        unsafe {
            ptr::write_bytes(self.data.add(index), value, length);
        }
        Ok(())
    }

    fn put_u8(&mut self, index: usize, value: u8) -> Result<()> {
        self.check_bounds(index, 1)?;
        unsafe { self.put_unchecked(index, value) };
        Ok(())
    }

    fn put_i8(&mut self, index: usize, value: i8) -> Result<()> {
        self.check_bounds(index, 1)?;
        unsafe { self.put_unchecked(index, value) };
        Ok(())
    }

    fn put_u16_with_order<B: ByteOrder>(&mut self, index: usize, value: u16, _byte_order: B) -> Result<()> {
        self.check_bounds(index, 2)?;
        let bytes = unsafe { slice::from_raw_parts_mut(self.data.add(index), 2) };
        B::write_u16(bytes, value);
        Ok(())
    }

    fn put_i16_with_order<B: ByteOrder>(&mut self, index: usize, value: i16, _byte_order: B) -> Result<()> {
        self.check_bounds(index, 2)?;
        let bytes = unsafe { slice::from_raw_parts_mut(self.data.add(index), 2) };
        B::write_i16(bytes, value);
        Ok(())
    }

    fn put_u32_with_order<B: ByteOrder>(&mut self, index: usize, value: u32, _byte_order: B) -> Result<()> {
        self.check_bounds(index, 4)?;
        let bytes = unsafe { slice::from_raw_parts_mut(self.data.add(index), 4) };
        B::write_u32(bytes, value);
        Ok(())
    }

    fn put_i32_with_order<B: ByteOrder>(&mut self, index: usize, value: i32, _byte_order: B) -> Result<()> {
        self.check_bounds(index, 4)?;
        let bytes = unsafe { slice::from_raw_parts_mut(self.data.add(index), 4) };
        B::write_i32(bytes, value);
        Ok(())
    }

    fn put_u64_with_order<B: ByteOrder>(&mut self, index: usize, value: u64, _byte_order: B) -> Result<()> {
        self.check_bounds(index, 8)?;
        let bytes = unsafe { slice::from_raw_parts_mut(self.data.add(index), 8) };
        B::write_u64(bytes, value);
        Ok(())
    }

    fn put_i64_with_order<B: ByteOrder>(&mut self, index: usize, value: i64, _byte_order: B) -> Result<()> {
        self.check_bounds(index, 8)?;
        let bytes = unsafe { slice::from_raw_parts_mut(self.data.add(index), 8) };
        B::write_i64(bytes, value);
        Ok(())
    }

    fn put_f32_with_order<B: ByteOrder>(&mut self, index: usize, value: f32, _byte_order: B) -> Result<()> {
        self.check_bounds(index, 4)?;
        let bytes = unsafe { slice::from_raw_parts_mut(self.data.add(index), 4) };
        B::write_f32(bytes, value);
        Ok(())
    }

    fn put_f64_with_order<B: ByteOrder>(&mut self, index: usize, value: f64, _byte_order: B) -> Result<()> {
        self.check_bounds(index, 8)?;
        let bytes = unsafe { slice::from_raw_parts_mut(self.data.add(index), 8) };
        B::write_f64(bytes, value);
        Ok(())
    }

    fn put_bytes(&mut self, index: usize, src: &[u8]) -> Result<()> {
        self.check_bounds(index, src.len())?;
        unsafe {
            ptr::copy_nonoverlapping(src.as_ptr(), self.data.add(index), src.len());
        }
        Ok(())
    }

    fn put_i32_ascii(&mut self, index: usize, value: i32) -> Result<usize> {
        let mut temp_buffer = [0u8; 12];
        let mut temp_index = temp_buffer.len();
        let mut remaining = value.abs() as u64;
        let negative = value < 0;

        loop {
            temp_index -= 1;
            temp_buffer[temp_index] = b'0' + (remaining % 10) as u8;
            remaining /= 10;
            if remaining == 0 {
                break;
            }
        }

        if negative {
            temp_index -= 1;
            temp_buffer[temp_index] = b'-';
        }

        let length = temp_buffer.len() - temp_index;
        self.put_bytes(index, &temp_buffer[temp_index..])?;
        Ok(length)
    }

    fn put_natural_i32_ascii(&mut self, index: usize, value: i32) -> Result<usize> {
        if value < 0 {
            return Err(AgronaError::AsciiNumberFormat("Negative value for natural number".to_string()));
        }
        self.put_i32_ascii(index, value)
    }

    fn put_natural_padded_i32_ascii(&mut self, index: usize, length: usize, value: i32) -> Result<()> {
        if value < 0 {
            return Err(AgronaError::AsciiNumberFormat("Negative value for natural number".to_string()));
        }

        let mut temp_buffer = vec![b'0'; length];
        let mut remaining = value as u64;
        let mut temp_index = length;

        loop {
            if temp_index == 0 {
                return Err(AgronaError::AsciiNumberFormat("Number too large for specified length".to_string()));
            }
            temp_index -= 1;
            temp_buffer[temp_index] = b'0' + (remaining % 10) as u8;
            remaining /= 10;
            if remaining == 0 {
                break;
            }
        }

        self.put_bytes(index, &temp_buffer)?;
        Ok(())
    }

    fn put_natural_i32_ascii_from_end(&mut self, value: i32, end_exclusive: usize) -> Result<usize> {
        if value < 0 {
            return Err(AgronaError::AsciiNumberFormat("Negative value for natural number".to_string()));
        }

        let mut remaining = value as u64;
        let mut current_index = end_exclusive;
        let start_index = current_index;

        loop {
            if current_index == 0 {
                return Err(AgronaError::IndexOutOfBounds {
                    index: 0,
                    length: 1,
                    capacity: self.capacity,
                });
            }
            current_index -= 1;
            self.put_u8(current_index, b'0' + (remaining % 10) as u8)?;
            remaining /= 10;
            if remaining == 0 {
                break;
            }
        }

        Ok(current_index)
    }

    fn put_natural_i64_ascii(&mut self, index: usize, value: i64) -> Result<usize> {
        if value < 0 {
            return Err(AgronaError::AsciiNumberFormat("Negative value for natural number".to_string()));
        }

        let mut temp_buffer = [0u8; 21];
        let mut temp_index = temp_buffer.len();
        let mut remaining = value as u64;

        loop {
            temp_index -= 1;
            temp_buffer[temp_index] = b'0' + (remaining % 10) as u8;
            remaining /= 10;
            if remaining == 0 {
                break;
            }
        }

        let length = temp_buffer.len() - temp_index;
        self.put_bytes(index, &temp_buffer[temp_index..])?;
        Ok(length)
    }

    fn put_i64_ascii(&mut self, index: usize, value: i64) -> Result<usize> {
        let mut temp_buffer = [0u8; 21];
        let mut temp_index = temp_buffer.len();
        let mut remaining = value.abs() as u64;
        let negative = value < 0;

        loop {
            temp_index -= 1;
            temp_buffer[temp_index] = b'0' + (remaining % 10) as u8;
            remaining /= 10;
            if remaining == 0 {
                break;
            }
        }

        if negative {
            temp_index -= 1;
            temp_buffer[temp_index] = b'-';
        }

        let length = temp_buffer.len() - temp_index;
        self.put_bytes(index, &temp_buffer[temp_index..])?;
        Ok(length)
    }

    fn put_string_ascii_without_length_range(
        &mut self,
        index: usize,
        value: &str,
        value_offset: usize,
        length: usize,
    ) -> Result<usize> {
        if value_offset + length > value.len() {
            return Err(AgronaError::IndexOutOfBounds {
                index: value_offset,
                length,
                capacity: value.len(),
            });
        }

        let slice = &value.as_bytes()[value_offset..value_offset + length];

        for &byte in slice {
            if byte > 127 {
                return Err(AgronaError::AsciiNumberFormat("Non-ASCII character found".to_string()));
            }
        }

        self.put_bytes(index, slice)?;
        Ok(length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::{BigEndian, LittleEndian};

    #[test]
    fn test_new_buffer() {
        let buffer = UnsafeBuffer::new(1024).unwrap();
        assert_eq!(buffer.capacity(), 1024);
    }

    #[test]
    fn test_wrap_slice() {
        let mut data = vec![0u8; 64];
        let buffer = UnsafeBuffer::wrap_slice(&mut data);
        assert_eq!(buffer.capacity(), 64);
    }

    #[test]
    fn test_basic_operations() {
        let mut buffer = UnsafeBuffer::new(64).unwrap();

        buffer.put_u32(0, 0x12345678).unwrap();
        assert_eq!(buffer.get_u32(0).unwrap(), 0x12345678);

        buffer.put_i64(8, -12345678901234i64).unwrap();
        assert_eq!(buffer.get_i64(8).unwrap(), -12345678901234i64);

        buffer.put_f64(16, 3.141592653589793).unwrap();
        assert!((buffer.get_f64(16).unwrap() - 3.141592653589793).abs() < f64::EPSILON);
    }

    #[test]
    fn test_ascii_parsing() {
        let mut buffer = UnsafeBuffer::new(64).unwrap();

        buffer.put_bytes(0, b"12345").unwrap();
        assert_eq!(buffer.parse_natural_i32_ascii(0, 5).unwrap(), 12345);

        buffer.put_bytes(10, b"-67890").unwrap();
        assert_eq!(buffer.parse_i32_ascii(10, 6).unwrap(), -67890);
    }

    #[test]
    fn test_string_operations() {
        let mut buffer = UnsafeBuffer::new(64).unwrap();

        let test_string = "Hello, World!";
        let bytes_written = buffer.put_string_ascii(0, test_string).unwrap();
        let retrieved = buffer.get_string_ascii(0).unwrap();

        assert_eq!(retrieved, test_string);
        assert_eq!(bytes_written, test_string.len() + 4);
    }

    #[test]
    fn test_bounds_checking() {
        let buffer = UnsafeBuffer::new(64).unwrap();

        assert!(buffer.get_u32(61).is_err());
        assert!(buffer.get_u64(57).is_err());
    }

    #[test]
    fn test_byte_order() {
        let mut buffer = UnsafeBuffer::new(64).unwrap();

        buffer.put_u32_with_order(0, 0x12345678, BigEndian).unwrap();
        buffer.put_u32_with_order(4, 0x12345678, LittleEndian).unwrap();

        let big_endian_result = buffer.get_u32_with_order(0, BigEndian).unwrap();
        let little_endian_result = buffer.get_u32_with_order(4, LittleEndian).unwrap();

        assert_eq!(big_endian_result, 0x12345678);
        assert_eq!(little_endian_result, 0x12345678);

        let big_as_little = buffer.get_u32_with_order(0, LittleEndian).unwrap();
        let little_as_big = buffer.get_u32_with_order(4, BigEndian).unwrap();

        assert_ne!(big_as_little, 0x12345678);
        assert_ne!(little_as_big, 0x12345678);
    }
}