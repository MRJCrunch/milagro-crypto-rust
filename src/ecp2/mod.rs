pub mod wrappers;

extern crate libc;
use self::libc::{c_int};

use fp2::wrappers::*;
use ecp2::wrappers::*;

impl ECP2 {
    pub fn set(P: &mut ECP2, x: &FP2, y: &FP2) {
        unsafe {
            ECP2_set(P, x, y);
        }
    }
}
