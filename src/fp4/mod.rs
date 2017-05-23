pub mod wrappers;

extern crate libc;

use std::fmt;
use std::str::SplitWhitespace;
use big::wrappers::*;
use fp2::wrappers::*;
use fp4::wrappers::*;

impl FP4 {
    pub fn to_hex(&self) -> String {
        let mut ret: String = String::with_capacity(4 * BIG_HEX_STRING_LEN);
        ret.push_str(&format!("{} {}", self.a.to_hex(), self.b.to_hex()));
        return ret;
    }

    pub fn from_hex_iter(iter: &mut SplitWhitespace) -> FP4 {
        let mut ret:FP4 = FP4::default();
        ret.a = FP2::from_hex_iter(iter);
        ret.b = FP2::from_hex_iter(iter);
        return ret;
    }

    pub fn from_hex(val: String) -> FP4 {
        let mut iter = val.split_whitespace();
        return FP4::from_hex_iter(&mut iter);
    }
}

impl PartialEq for FP4 {
    fn eq(&self, other: &FP4) -> bool {
        return (self.a == other.a) &&
            (self.b == other.b);
    }
}

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
