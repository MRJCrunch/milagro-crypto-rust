#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;
use self::libc::{c_int, c_void, c_char, uint32_t};

use big::wrappers::*;

#[repr(C)]
pub struct FP2 {
    a: BIG,
    b: BIG
}

impl Default for FP2 {
    fn default () -> FP2 {
        FP2 {
            a: BIG_ZERO!(),
            b: BIG_ZERO!()
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
