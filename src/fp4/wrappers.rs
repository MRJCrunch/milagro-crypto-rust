#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;
use self::libc::{c_int, c_void, c_char, uint32_t};

use fp2::wrappers::FP2;

#[repr(C)]
pub struct FP4 {
    a: FP2,
    b: FP2
}

impl Default for FP4 {
    fn default () -> FP4 {
        FP4 {
            a: Default::default(),
            b: Default::default()
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
