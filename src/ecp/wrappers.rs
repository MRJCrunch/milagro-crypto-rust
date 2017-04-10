#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;
use self::libc::{c_int, c_void, c_char, uint32_t};

use big::wrappers::*;

// KLUDGE: depends on CURVETYPE milagro define. This is "CURVETYPE: WEIERSTRASS"
// Check amcl_build output!
#[repr(C)]
pub struct ECP {
    inf: c_int,
    x: BIG,
    y: BIG,
    z: BIG
}

impl Default for ECP {
    fn default () -> ECP {
        ECP {
            inf: 0,
            x: BIG_ZERO!(),
            y: BIG_ZERO!(),
            z: BIG_ZERO!()
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
