#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;
use self::libc::{c_int, c_void, c_char, uint32_t};

use big::wrappers::BIG;
use randapi::wrappers::octet;
use fp4::wrappers::FP4;

#[repr(C)]
pub struct FP12 {
    pub a: FP4,
    pub b: FP4,
    pub c: FP4
}

impl Default for FP12 {
    fn default () -> FP12 {
        FP12 {
            a: Default::default(),
            b: Default::default(),
            c: Default::default()
        }
    }
}

extern {
    pub fn FP12_output(w: *const FP12) -> c_void;
    pub fn FP12_mul(w: *mut FP12, y: *const FP12) -> c_void;
    pub fn FP12_pow(r: *mut FP12, a: *const FP12, b: *const BIG) -> c_void;
    pub fn FP12_inv(w: *mut FP12, x: *const FP12) -> c_void;
    pub fn FP12_toOctet(W: *mut octet, g: *const FP12) -> c_void;
    pub fn FP12_fromOctet(g: *mut FP12, W: *const octet) -> c_void;
}


/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qwe() {
        assert_eq!(1, 1);
    }
}
*/
