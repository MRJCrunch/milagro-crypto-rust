#![allow(non_snake_case)]

pub mod wrappers;

extern crate libc;

use std::fmt;
use std::str::SplitWhitespace;
use big::wrappers::*;
use randapi::wrappers::*;
use ecp::wrappers::*;

impl ECP {
    pub fn set(P: &mut ECP, x: &BIG, y: &BIG) {
        unsafe {
            ECP_set(P, x, y);
        }
    }

    pub fn mul(P: &mut ECP, e: &BIG) {
        unsafe {
            ECP_mul(P, e);
        }
    }

    pub fn add(P: &mut ECP, Q: &ECP) {
        unsafe {
            ECP_add(P, Q);
        }
    }

    pub fn inf(a: &mut ECP) {
        unsafe {
            ECP_inf(a);
        }
    }

    fn rhs(x: &mut BIG) -> BIG {
        let mut r = BIG::default();
        unsafe {
            ECP_rhs(&mut r, x);
        }
        return r;
    }

    pub fn is_infinity(a: &ECP) -> bool {
        // KLUDGE: depends on CURVETYPE milagro define. This is "CURVETYPE: WEIERSTRASS"
        return a.inf != 0;
    }

    pub fn neg(P: &mut ECP) {
        unsafe {
            ECP_neg(P);
        }
    }

    pub fn sub(P: &mut ECP, Q:&ECP) {
        unsafe {
            ECP_sub(P, Q);
        }
    }

    pub fn new_bigs(ix: &BIG,iy: &BIG) -> ECP {
        let mut E=ECP::default();
        E.x = ix.clone();
        E.y = iy.clone();
        BIG::one(&mut E.z);
        let rhs=ECP::rhs(&mut E.x);

        // KLUDGE: depends on CURVETYPE milagro define. This is "CURVETYPE: WEIERSTRASS"

        let mut y2=BIG::new_copy(&E.y);
        BIG::sqrm(&mut y2);
        if y2 == rhs {
            E.inf=0;
        } else {
            ECP::inf(&mut E);
        }
        return E;
    }

    pub fn toOctet(W: &mut octet, P: &ECP) {
        unsafe {
            ECP_toOctet(W, P);
        }
    }

    pub fn fromOctet(W: &octet) -> ECP {
        let mut ret: ECP = ECP::default();
        unsafe {
            ECP_fromOctet(&mut ret, W);
        }
        return ret;
    }

    pub fn to_hex(&self) -> String {
        let mut ret: String = String::with_capacity(4 * BIG_HEX_STRING_LEN);
        ret.push_str(&format!("{} {} {} {}", self.inf, self.x.to_hex(), self.y.to_hex(), self.z.to_hex()));
        return ret;
    }

    pub fn from_hex_iter(iter: &mut SplitWhitespace) -> ECP {
        let mut ret:ECP = ECP::default();
        if let Some(x) = iter.next() {
            ret.inf = i32::from_str_radix(x, 16).unwrap();
            ret.x = BIG::from_hex_iter(iter);
            ret.y = BIG::from_hex_iter(iter);
            ret.z = BIG::from_hex_iter(iter);
        }
        return ret;
    }

    pub fn from_hex(val: String) -> ECP {
        let mut iter = val.split_whitespace();
        return ECP::from_hex_iter(&mut iter);
    }
}

impl PartialEq for ECP {
    fn eq(&self, other: &ECP) -> bool {
        return (self.inf == other.inf) &&
            (self.x == other.x) &&
            (self.y == other.y) &&
            (self.z == other.z);
    }
}

impl Copy for ECP { }

impl Clone for ECP {
    fn clone(&self) -> ECP {
        ECP {
            inf: self.inf,
            x: self.x,
            y: self.y,
            z: self.z
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        let ecp1 = ECP::default();
        let ecp2 = ECP::default();
        let mut ecp3 = ECP::default();
        ecp3.inf = 1;
        assert_eq!(ecp1, ecp2);
        assert_ne!(ecp1, ecp3);
    }

    #[test]
    fn test_new_bigs() {
        let bx = unsafe { CURVE_Gx };
        let by = unsafe { CURVE_Gy };
        let ecp = ECP::new_bigs(&bx, &by);
        println!("new_bigs: ret={}, x={}, y={}", ecp, bx, by);
    }
}
