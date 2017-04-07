#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;
use self::libc::{c_int, c_void, c_char, uint32_t};

use fp4::wrappers::FP4;

#[repr(C)]
pub struct FP12 {
    a: FP4,
    b: FP4,
    c: FP4
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
