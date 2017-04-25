pub mod wrappers;

extern crate libc;

use std::fmt;
use fp4::wrappers::*;

impl Copy for FP4 { }

impl Clone for FP4 {
    fn clone(&self) -> FP4 {
        FP4 {
            a: self.a,
            b: self.b
        }
    }
}

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
