use agrona_core::buffer::{DirectBuffer, MutableBuffer, UnsafeBuffer};
use agrona_core::error::Result;
use byteorder::{ByteOrder, LittleEndian};
use std::sync::atomic::{AtomicU64, Ordering};
use std::ptr;

pub struct AtomicBuffer {
    inner: UnsafeBuffer,
}

impl AtomicBuffer {
    pub fn new(capacity: usize) -> Result<Self> {
        Ok(Self {
            inner: UnsafeBuffer::new(capacity)?,
        })
    }

    pub fn wrap(data: *mut u8, capacity: usize) -> Self {
        Self {
            inner: UnsafeBuffer::wrap(data, capacity),
        }
    }

    pub fn wrap_slice(slice: &mut [u8]) -> Self {
        Self {
            inner: UnsafeBuffer::wrap_slice(slice),
        }
    }

    #[inline]
    pub fn get_volatile_u8(&self, index: usize) -> Result<u8> {
        self.bounds_check(index, 1)?;
        unsafe {
            let ptr = self.inner.as_ptr().add(index);
            Ok(ptr::read_volatile(ptr))
        }
    }

    #[inline]
    pub fn put_volatile_u8(&mut self, index: usize, value: u8) -> Result<()> {
        self.bounds_check(index, 1)?;
        unsafe {
            let ptr = self.inner.as_mut_ptr().add(index);
            ptr::write_volatile(ptr, value);
        }
        Ok(())
    }

    #[inline]
    pub fn get_volatile_u32(&self, index: usize) -> Result<u32> {
        self.get_volatile_u32_with_order(index, LittleEndian)
    }

    #[inline]
    pub fn get_volatile_u32_with_order<B: ByteOrder>(&self, index: usize, _byte_order: B) -> Result<u32> {
        self.bounds_check(index, 4)?;
        unsafe {
            let ptr = self.inner.as_ptr().add(index) as *const u32;
            let value = ptr::read_volatile(ptr);
            Ok(if B::NATIVE_ENDIAN {
                value
            } else {
                value.swap_bytes()
            })
        }
    }

    #[inline]
    pub fn put_volatile_u32(&mut self, index: usize, value: u32) -> Result<()> {
        self.put_volatile_u32_with_order(index, value, LittleEndian)
    }

    #[inline]
    pub fn put_volatile_u32_with_order<B: ByteOrder>(&mut self, index: usize, value: u32, _byte_order: B) -> Result<()> {
        self.bounds_check(index, 4)?;
        unsafe {
            let ptr = self.inner.as_mut_ptr().add(index) as *mut u32;
            let write_value = if B::NATIVE_ENDIAN {
                value
            } else {
                value.swap_bytes()
            };
            ptr::write_volatile(ptr, write_value);
        }
        Ok(())
    }

    #[inline]
    pub fn get_volatile_u64(&self, index: usize) -> Result<u64> {
        self.get_volatile_u64_with_order(index, LittleEndian)
    }

    #[inline]
    pub fn get_volatile_u64_with_order<B: ByteOrder>(&self, index: usize, _byte_order: B) -> Result<u64> {
        self.bounds_check(index, 8)?;
        unsafe {
            let ptr = self.inner.as_ptr().add(index) as *const u64;
            let value = ptr::read_volatile(ptr);
            Ok(if B::NATIVE_ENDIAN {
                value
            } else {
                value.swap_bytes()
            })
        }
    }

    #[inline]
    pub fn put_volatile_u64(&mut self, index: usize, value: u64) -> Result<()> {
        self.put_volatile_u64_with_order(index, value, LittleEndian)
    }

    #[inline]
    pub fn put_volatile_u64_with_order<B: ByteOrder>(&mut self, index: usize, value: u64, _byte_order: B) -> Result<()> {
        self.bounds_check(index, 8)?;
        unsafe {
            let ptr = self.inner.as_mut_ptr().add(index) as *mut u64;
            let write_value = if B::NATIVE_ENDIAN {
                value
            } else {
                value.swap_bytes()
            };
            ptr::write_volatile(ptr, write_value);
        }
        Ok(())
    }

    #[inline]
    pub fn compare_and_set_u32(&mut self, index: usize, expected: u32, update: u32) -> Result<bool> {
        self.bounds_check(index, 4)?;
        unsafe {
            let ptr = self.inner.as_mut_ptr().add(index) as *mut AtomicU64;
            let atomic_ref = &*ptr;
            let packed_expected = (expected as u64) << 32 | expected as u64;
            let packed_update = (update as u64) << 32 | update as u64;
            Ok(atomic_ref.compare_exchange_weak(
                packed_expected,
                packed_update,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ).is_ok())
        }
    }

    #[inline]
    pub fn get_and_add_u32(&mut self, index: usize, delta: u32) -> Result<u32> {
        self.bounds_check(index, 4)?;
        unsafe {
            let ptr = self.inner.as_mut_ptr().add(index) as *mut AtomicU64;
            let atomic_ref = &*ptr;
            let old_packed = atomic_ref.fetch_add(
                ((delta as u64) << 32) | delta as u64,
                Ordering::SeqCst,
            );
            Ok((old_packed >> 32) as u32)
        }
    }

    #[inline]
    pub fn get_and_add_u64(&mut self, index: usize, delta: u64) -> Result<u64> {
        self.bounds_check(index, 8)?;
        unsafe {
            let ptr = self.inner.as_mut_ptr().add(index) as *mut AtomicU64;
            let atomic_ref = &*ptr;
            Ok(atomic_ref.fetch_add(delta, Ordering::SeqCst))
        }
    }

    #[inline]
    pub fn put_ordered_u32(&mut self, index: usize, value: u32) -> Result<()> {
        self.put_ordered_u32_with_order(index, value, LittleEndian)
    }

    #[inline]
    pub fn put_ordered_u32_with_order<B: ByteOrder>(&mut self, index: usize, value: u32, _byte_order: B) -> Result<()> {
        self.bounds_check(index, 4)?;
        unsafe {
            let ptr = self.inner.as_mut_ptr().add(index) as *mut AtomicU64;
            let atomic_ref = &*ptr;
            let write_value = if B::NATIVE_ENDIAN {
                value as u64
            } else {
                value.swap_bytes() as u64
            };
            atomic_ref.store(write_value, Ordering::Release);
        }
        Ok(())
    }

    #[inline]
    pub fn put_ordered_u64(&mut self, index: usize, value: u64) -> Result<()> {
        self.put_ordered_u64_with_order(index, value, LittleEndian)
    }

    #[inline]
    pub fn put_ordered_u64_with_order<B: ByteOrder>(&mut self, index: usize, value: u64, _byte_order: B) -> Result<()> {
        self.bounds_check(index, 8)?;
        unsafe {
            let ptr = self.inner.as_mut_ptr().add(index) as *mut AtomicU64;
            let atomic_ref = &*ptr;
            let write_value = if B::NATIVE_ENDIAN {
                value
            } else {
                value.swap_bytes()
            };
            atomic_ref.store(write_value, Ordering::Release);
        }
        Ok(())
    }

    #[inline]
    pub fn add_ordered_u64(&mut self, index: usize, increment: u64) -> Result<()> {
        self.bounds_check(index, 8)?;
        unsafe {
            let ptr = self.inner.as_mut_ptr().add(index) as *mut AtomicU64;
            let atomic_ref = &*ptr;
            atomic_ref.fetch_add(increment, Ordering::Release);
        }
        Ok(())
    }
}

unsafe impl Send for AtomicBuffer {}
unsafe impl Sync for AtomicBuffer {}

impl DirectBuffer for AtomicBuffer {
    fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    fn get_u8(&self, index: usize) -> Result<u8> {
        self.inner.get_u8(index)
    }

    fn get_i8(&self, index: usize) -> Result<i8> {
        self.inner.get_i8(index)
    }

    fn get_u16_with_order<B: ByteOrder>(&self, index: usize, byte_order: B) -> Result<u16> {
        self.inner.get_u16_with_order(index, byte_order)
    }

    fn get_i16_with_order<B: ByteOrder>(&self, index: usize, byte_order: B) -> Result<i16> {
        self.inner.get_i16_with_order(index, byte_order)
    }

    fn get_u32_with_order<B: ByteOrder>(&self, index: usize, byte_order: B) -> Result<u32> {
        self.inner.get_u32_with_order(index, byte_order)
    }

    fn get_i32_with_order<B: ByteOrder>(&self, index: usize, byte_order: B) -> Result<i32> {
        self.inner.get_i32_with_order(index, byte_order)
    }

    fn get_u64_with_order<B: ByteOrder>(&self, index: usize, byte_order: B) -> Result<u64> {
        self.inner.get_u64_with_order(index, byte_order)
    }

    fn get_i64_with_order<B: ByteOrder>(&self, index: usize, byte_order: B) -> Result<i64> {
        self.inner.get_i64_with_order(index, byte_order)
    }

    fn get_f32_with_order<B: ByteOrder>(&self, index: usize, byte_order: B) -> Result<f32> {
        self.inner.get_f32_with_order(index, byte_order)
    }

    fn get_f64_with_order<B: ByteOrder>(&self, index: usize, byte_order: B) -> Result<f64> {
        self.inner.get_f64_with_order(index, byte_order)
    }

    fn get_bytes(&self, index: usize, dst: &mut [u8]) -> Result<()> {
        self.inner.get_bytes(index, dst)
    }

    fn parse_natural_i32_ascii(&self, index: usize, length: usize) -> Result<i32> {
        self.inner.parse_natural_i32_ascii(index, length)
    }

    fn parse_natural_i64_ascii(&self, index: usize, length: usize) -> Result<i64> {
        self.inner.parse_natural_i64_ascii(index, length)
    }

    fn parse_i32_ascii(&self, index: usize, length: usize) -> Result<i32> {
        self.inner.parse_i32_ascii(index, length)
    }

    fn parse_i64_ascii(&self, index: usize, length: usize) -> Result<i64> {
        self.inner.parse_i64_ascii(index, length)
    }

    fn get_string_ascii_with_length(&self, index: usize, length: usize) -> Result<String> {
        self.inner.get_string_ascii_with_length(index, length)
    }

    fn get_string_utf8_with_length(&self, index: usize, length: usize) -> Result<String> {
        self.inner.get_string_utf8_with_length(index, length)
    }
}

impl MutableBuffer for AtomicBuffer {
    fn set_memory(&mut self, index: usize, length: usize, value: u8) -> Result<()> {
        self.inner.set_memory(index, length, value)
    }

    fn put_u8(&mut self, index: usize, value: u8) -> Result<()> {
        self.inner.put_u8(index, value)
    }

    fn put_i8(&mut self, index: usize, value: i8) -> Result<()> {
        self.inner.put_i8(index, value)
    }

    fn put_u16_with_order<B: ByteOrder>(&mut self, index: usize, value: u16, byte_order: B) -> Result<()> {
        self.inner.put_u16_with_order(index, value, byte_order)
    }

    fn put_i16_with_order<B: ByteOrder>(&mut self, index: usize, value: i16, byte_order: B) -> Result<()> {
        self.inner.put_i16_with_order(index, value, byte_order)
    }

    fn put_u32_with_order<B: ByteOrder>(&mut self, index: usize, value: u32, byte_order: B) -> Result<()> {
        self.inner.put_u32_with_order(index, value, byte_order)
    }

    fn put_i32_with_order<B: ByteOrder>(&mut self, index: usize, value: i32, byte_order: B) -> Result<()> {
        self.inner.put_i32_with_order(index, value, byte_order)
    }

    fn put_u64_with_order<B: ByteOrder>(&mut self, index: usize, value: u64, byte_order: B) -> Result<()> {
        self.inner.put_u64_with_order(index, value, byte_order)
    }

    fn put_i64_with_order<B: ByteOrder>(&mut self, index: usize, value: i64, byte_order: B) -> Result<()> {
        self.inner.put_i64_with_order(index, value, byte_order)
    }

    fn put_f32_with_order<B: ByteOrder>(&mut self, index: usize, value: f32, byte_order: B) -> Result<()> {
        self.inner.put_f32_with_order(index, value, byte_order)
    }

    fn put_f64_with_order<B: ByteOrder>(&mut self, index: usize, value: f64, byte_order: B) -> Result<()> {
        self.inner.put_f64_with_order(index, value, byte_order)
    }

    fn put_bytes(&mut self, index: usize, src: &[u8]) -> Result<()> {
        self.inner.put_bytes(index, src)
    }

    fn put_i32_ascii(&mut self, index: usize, value: i32) -> Result<usize> {
        self.inner.put_i32_ascii(index, value)
    }

    fn put_natural_i32_ascii(&mut self, index: usize, value: i32) -> Result<usize> {
        self.inner.put_natural_i32_ascii(index, value)
    }

    fn put_natural_padded_i32_ascii(&mut self, index: usize, length: usize, value: i32) -> Result<()> {
        self.inner.put_natural_padded_i32_ascii(index, length, value)
    }

    fn put_natural_i32_ascii_from_end(&mut self, value: i32, end_exclusive: usize) -> Result<usize> {
        self.inner.put_natural_i32_ascii_from_end(value, end_exclusive)
    }

    fn put_natural_i64_ascii(&mut self, index: usize, value: i64) -> Result<usize> {
        self.inner.put_natural_i64_ascii(index, value)
    }

    fn put_i64_ascii(&mut self, index: usize, value: i64) -> Result<usize> {
        self.inner.put_i64_ascii(index, value)
    }

    fn put_string_ascii_without_length_range(
        &mut self,
        index: usize,
        value: &str,
        value_offset: usize,
        length: usize,
    ) -> Result<usize> {
        self.inner.put_string_ascii_without_length_range(index, value, value_offset, length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_operations() {
        let mut buffer = AtomicBuffer::new(64).unwrap();

        buffer.put_volatile_u32(0, 42).unwrap();
        assert_eq!(buffer.get_volatile_u32(0).unwrap(), 42);

        buffer.put_volatile_u64(8, 1234567890123456789).unwrap();
        assert_eq!(buffer.get_volatile_u64(8).unwrap(), 1234567890123456789);
    }

    #[test]
    fn test_ordered_operations() {
        let mut buffer = AtomicBuffer::new(64).unwrap();

        buffer.put_ordered_u32(0, 100).unwrap();
        assert_eq!(buffer.get_volatile_u32(0).unwrap(), 100);

        buffer.add_ordered_u64(8, 50).unwrap();
        buffer.add_ordered_u64(8, 25).unwrap();
        assert_eq!(buffer.get_volatile_u64(8).unwrap(), 75);
    }
}