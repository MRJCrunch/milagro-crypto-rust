pub mod wrappers;

extern crate libc;
use self::libc::{c_int};

use std::fmt;
use big::wrappers::*;
use ecp::wrappers::*;

impl ECP {
    pub fn set(P: &mut ECP, x: &BIG, y: &BIG) {
        unsafe {
            ECP_set(P, x, y);
        }
    }
}

impl fmt::Display for ECP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ECP: [ {}, {}, {}, {} ]", self.inf, self.x, self.y, self.z)
    }
}

impl fmt::Debug for ECP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ECP: [ {}, {}, {}, {} ]", self.inf, self.x, self.y, self.z)
    }
}
