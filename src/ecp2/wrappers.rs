#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;
use self::libc::{c_int, c_void, c_char, uint32_t};

use fp2::wrappers::FP2;

#[repr(C)]
pub struct ECP2 {
    inf: c_int,
    x: FP2,
    y: FP2,
    z: FP2
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

/*
extern {
    pub fn qwe(qwe: asd) -> c_void;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qwe() {
        assert_eq!(1, 1);
    }
}
*/
