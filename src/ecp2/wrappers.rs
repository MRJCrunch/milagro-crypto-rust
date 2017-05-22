#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;
use self::libc::{c_int, c_void};

use big::wrappers::*;
use fp2::wrappers::FP2;

#[repr(C)]
pub struct ECP2 {
    pub inf: c_int,
    pub x: FP2,
    pub y: FP2,
    pub z: FP2
}

impl Default for ECP2 {
    fn default () -> ECP2 {
        ECP2 {
            inf: 0,
            x: Default::default(),
            y: Default::default(),
            z: Default::default()
        }
    }
}

extern {
    pub fn ECP2_set(P: *mut ECP2, x: *const FP2, y: *const FP2) -> c_void;
    pub fn ECP2_output(P: *const ECP2) -> c_void;
    pub fn ECP2_mul(P: *mut ECP2, e: *const BIG) -> c_void;
    pub fn ECP2_add(P: *mut ECP2, Q: *const ECP2) -> c_void;
    pub fn ECP2_sub(P: *mut ECP2, W: *const ECP2) -> c_void;
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
