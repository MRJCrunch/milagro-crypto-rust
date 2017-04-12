pub mod wrappers;

extern crate libc;
use self::libc::{c_int};

use std::fmt;
use fp4::wrappers::*;

impl fmt::Display for FP4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FP4: [ {}, {} ]", self.a, self.b)
    }
}

impl fmt::Debug for FP4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FP4: [ {}, {} ]", self.a, self.b)
    }
}
