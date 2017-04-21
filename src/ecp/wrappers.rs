#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;
use self::libc::{c_int, c_void, c_char, uint32_t};

use big::wrappers::*;
use randapi::wrappers::*;

// KLUDGE: depends on CURVETYPE milagro define. This is "CURVETYPE: WEIERSTRASS"
// Check amcl_build output!
#[repr(C)]
pub struct ECP {
    pub inf: c_int,
    pub x: BIG,
    pub y: BIG,
    pub z: BIG
}

impl Default for ECP {
    fn default () -> ECP {
        ECP {
            inf: 0,
            x: BIG::default(),
            y: BIG::default(),
            z: BIG::default()
        }
    }
}

extern {
    pub fn ECP_set(P: *mut ECP, x: *const BIG, y: *const BIG) -> c_void;
    pub fn ECP_output(P: *const ECP) -> c_void;

    // TODO:
    //new_bigs

    pub fn ECP_mul(P: *mut ECP, e: *const BIG) -> c_void;
    pub fn ECP_add(P: *mut ECP, Q: *const ECP, W: *const ECP) -> c_void;
    pub fn ECP_toOctet(W: *mut octet, P: *const ECP) -> c_void;
    pub fn ECP_fromOctet(P: *mut ECP, W: *const octet) -> c_int;
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
