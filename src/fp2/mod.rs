#![allow(non_snake_case)]

pub mod wrappers;

use std::fmt;
use std::str::SplitWhitespace;
use big::wrappers::*;
use fp2::wrappers::*;

impl FP2 {
    pub fn from_BIGs(w: &mut FP2, x: &BIG, y: &BIG) {
        unsafe {
            FP2_from_BIGs(w, x, y);
        }
    }

    pub fn to_hex(&self) -> String {
        let mut ret: String = String::with_capacity(2 * BIG_HEX_STRING_LEN);
        ret.push_str(&format!("{} {}", self.a.to_hex(), self.b.to_hex()));
        return ret;
    }

    pub fn from_hex_iter(iter: &mut SplitWhitespace) -> FP2 {
        let mut ret:FP2 = FP2::default();
        ret.a = BIG::from_hex_iter(iter);
        ret.b = BIG::from_hex_iter(iter);
        return ret;
    }

    pub fn from_hex(val: String) -> FP2 {
        let mut iter = val.split_whitespace();
        return FP2::from_hex_iter(&mut iter);
    }
}

impl PartialEq for FP2 {
    fn eq(&self, other: &FP2) -> bool {
        return (self.a == other.a) &&
            (self.b == other.b);
    }
}

impl Copy for FP2 { }

impl Clone for FP2 {
    fn clone(&self) -> FP2 {
        FP2 {
            a: self.a,
            b: self.b
        }
    }
}

impl fmt::Display for FP2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FP2: [ {}, {} ]", self.a, self.b)
    }
}

impl fmt::Debug for FP2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FP2: [ {}, {} ]", self.a, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fp2_hex_io() {
        let mut m = FP2::default();
        let b1 = BIG::new_int(1000);
        let b2 = BIG::new_int(2000000);
        FP2::from_BIGs(&mut m, &b1, &b2);
        let s = m.to_hex();
        let r = FP2::from_hex(s.clone());
        println!("fp2_hex_io=s:{},m:{},r:{}", s, m, r);
        assert_eq!(m, r);
    }
}
