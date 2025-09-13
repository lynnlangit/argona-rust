#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::missing_safety_doc)]

pub mod buffer;
pub mod bit_util;
pub mod error;

pub use buffer::*;
pub use bit_util::*;
pub use error::*;

use core::mem;

pub const CACHE_LINE_SIZE: usize = 64;