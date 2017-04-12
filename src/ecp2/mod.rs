pub mod wrappers;

extern crate libc;
use self::libc::{c_int};

use std::fmt;
use fp2::wrappers::*;
use ecp2::wrappers::*;

impl ECP2 {
    pub fn set(P: &mut ECP2, x: &FP2, y: &FP2) {
        unsafe {
            ECP2_set(P, x, y);
        }
    }
}

impl fmt::Display for ECP2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ECP2: [ {}, {}, {}, {} ]", self.inf, self.x, self.y, self.z)
    }
}

impl fmt::Debug for ECP2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ECP2: [ {}, {}, {}, {} ]", self.inf, self.x, self.y, self.z)
    }
}
