#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;

use std::mem;
use std::ptr;

use libc::{size_t, c_uchar, c_int, c_ulonglong, c_char, c_void,
           uint32_t, uint64_t, uint8_t, int32_t, int64_t, uint16_t};

extern {
    pub fn amcl_version() -> c_void;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_amcl_version() {
        unsafe {
            amcl_version();
        }
    }
}
