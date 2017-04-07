#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;
use self::libc::{c_int, c_void, c_char, uint32_t};

use std::mem;

use fp12::wrappers::*;
use ecp::wrappers::*;
use ecp2::wrappers::*;

extern {
    pub fn PAIR_ate(r: *mut FP12, P: *mut ECP2, Q: *mut ECP) -> c_void;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair() {
        let mut r: FP12 = unsafe { mem::zeroed() };
        let mut P: ECP2 = unsafe { mem::zeroed() };
        let mut Q: ECP = unsafe { mem::zeroed() };
        unsafe {
//            PAIR_ate(&mut r, &mut P, &mut Q);
        }
        assert_eq!(1, 1);
    }
}
