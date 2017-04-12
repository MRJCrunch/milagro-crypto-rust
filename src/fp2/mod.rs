pub mod wrappers;

extern crate libc;
use self::libc::{c_int};

use big::wrappers::*;
use fp2::wrappers::*;

impl FP2 {
    pub fn from_BIGs(w: &mut FP2, x: &BIG, y: &BIG) {
        unsafe {
            FP2_from_BIGs(w, x, y);
        }
    }
}
