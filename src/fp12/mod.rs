pub mod wrappers;

extern crate libc;
use self::libc::{c_int};

use std::fmt;
use fp12::wrappers::*;

impl fmt::Display for FP12 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FP12: [ {}, {}, {} ]", self.a, self.b, self.c)
    }
}

impl fmt::Debug for FP12 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FP12: [ {}, {}, {} ]", self.a, self.b, self.c)
    }
}
