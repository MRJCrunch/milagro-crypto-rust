pub mod wrappers;

extern crate libc;
use self::libc::{c_int};

use big::wrappers::*;
use ecp::wrappers::*;

impl ECP {
    pub fn set(P: &mut ECP, x: &BIG, y: &BIG) {
        unsafe {
            ECP_set(P, x, y);
        }
    }
}
