use crate::buffer::DirectBuffer;
use crate::error::{AgronaError, Result};
use byteorder::{ByteOrder, LittleEndian};

pub trait MutableBuffer: DirectBuffer {
    fn is_expandable(&self) -> bool {
        false
    }

    fn set_memory(&mut self, index: usize, length: usize, value: u8) -> Result<()>;

    fn put_u8(&mut self, index: usize, value: u8) -> Result<()>;
    fn put_i8(&mut self, index: usize, value: i8) -> Result<()>;

    fn put_u16(&mut self, index: usize, value: u16) -> Result<()> {
        self.put_u16_with_order(index, value, LittleEndian)
    }

    fn put_u16_with_order<B: ByteOrder>(&mut self, index: usize, value: u16, _byte_order: B) -> Result<()>;

    fn put_i16(&mut self, index: usize, value: i16) -> Result<()> {
        self.put_i16_with_order(index, value, LittleEndian)
    }

    fn put_i16_with_order<B: ByteOrder>(&mut self, index: usize, value: i16, _byte_order: B) -> Result<()>;

    fn put_u32(&mut self, index: usize, value: u32) -> Result<()> {
        self.put_u32_with_order(index, value, LittleEndian)
    }

    fn put_u32_with_order<B: ByteOrder>(&mut self, index: usize, value: u32, _byte_order: B) -> Result<()>;

    fn put_i32(&mut self, index: usize, value: i32) -> Result<()> {
        self.put_i32_with_order(index, value, LittleEndian)
    }

    fn put_i32_with_order<B: ByteOrder>(&mut self, index: usize, value: i32, _byte_order: B) -> Result<()>;

    fn put_u64(&mut self, index: usize, value: u64) -> Result<()> {
        self.put_u64_with_order(index, value, LittleEndian)
    }

    fn put_u64_with_order<B: ByteOrder>(&mut self, index: usize, value: u64, _byte_order: B) -> Result<()>;

    fn put_i64(&mut self, index: usize, value: i64) -> Result<()> {
        self.put_i64_with_order(index, value, LittleEndian)
    }

    fn put_i64_with_order<B: ByteOrder>(&mut self, index: usize, value: i64, _byte_order: B) -> Result<()>;

    fn put_f32(&mut self, index: usize, value: f32) -> Result<()> {
        self.put_f32_with_order(index, value, LittleEndian)
    }

    fn put_f32_with_order<B: ByteOrder>(&mut self, index: usize, value: f32, _byte_order: B) -> Result<()>;

    fn put_f64(&mut self, index: usize, value: f64) -> Result<()> {
        self.put_f64_with_order(index, value, LittleEndian)
    }

    fn put_f64_with_order<B: ByteOrder>(&mut self, index: usize, value: f64, _byte_order: B) -> Result<()>;

    fn put_bytes(&mut self, index: usize, src: &[u8]) -> Result<()>;

    fn put_bytes_from(&mut self, index: usize, src: &[u8], offset: usize, length: usize) -> Result<()> {
        if offset + length > src.len() {
            return Err(AgronaError::IndexOutOfBounds {
                index: offset,
                length,
                capacity: src.len(),
            });
        }
        self.put_bytes(index, &src[offset..offset + length])
    }

    fn put_i32_ascii(&mut self, index: usize, value: i32) -> Result<usize>;
    fn put_natural_i32_ascii(&mut self, index: usize, value: i32) -> Result<usize>;
    fn put_natural_padded_i32_ascii(&mut self, index: usize, length: usize, value: i32) -> Result<()>;
    fn put_natural_i32_ascii_from_end(&mut self, value: i32, end_exclusive: usize) -> Result<usize>;
    fn put_natural_i64_ascii(&mut self, index: usize, value: i64) -> Result<usize>;
    fn put_i64_ascii(&mut self, index: usize, value: i64) -> Result<usize>;

    fn put_string_ascii(&mut self, index: usize, value: &str) -> Result<usize> {
        let length = value.len();
        self.put_u32(index, length as u32)?;
        self.put_string_ascii_without_length(index + 4, value)?;
        Ok(4 + length)
    }

    fn put_string_ascii_without_length(&mut self, index: usize, value: &str) -> Result<usize> {
        self.put_string_ascii_without_length_range(index, value, 0, value.len())
    }

    fn put_string_ascii_without_length_range(
        &mut self,
        index: usize,
        value: &str,
        value_offset: usize,
        length: usize,
    ) -> Result<usize>;

    fn put_string_utf8(&mut self, index: usize, value: &str) -> Result<usize> {
        let bytes = value.as_bytes();
        let length = bytes.len();
        self.put_u32(index, length as u32)?;
        self.put_bytes(index + 4, bytes)?;
        Ok(4 + length)
    }

    fn put_string_utf8_without_length(&mut self, index: usize, value: &str) -> Result<usize> {
        let bytes = value.as_bytes();
        self.put_bytes(index, bytes)?;
        Ok(bytes.len())
    }
}