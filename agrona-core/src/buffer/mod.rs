pub mod direct_buffer;
pub mod mutable_buffer;
pub mod unsafe_buffer;

pub use direct_buffer::*;
pub use mutable_buffer::*;
pub use unsafe_buffer::*;

use crate::error::{AgronaError, Result};

pub const STR_HEADER_LEN: usize = 4;

#[cfg(feature = "no_bounds_check")]
const BOUNDS_CHECK_ENABLED: bool = false;
#[cfg(not(feature = "no_bounds_check"))]
const BOUNDS_CHECK_ENABLED: bool = true;

#[inline(always)]
fn bounds_check(index: usize, length: usize, capacity: usize) -> Result<()> {
    if BOUNDS_CHECK_ENABLED && index + length > capacity {
        return Err(AgronaError::IndexOutOfBounds {
            index,
            length,
            capacity,
        });
    }
    Ok(())
}