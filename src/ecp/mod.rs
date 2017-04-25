pub mod wrappers;

extern crate libc;
use self::libc::{c_int};

use std::fmt;
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

    pub fn add(P: &mut ECP, Q: &ECP, W: &ECP) {
        unsafe {
            ECP_add(P, Q, W);
        }
    }

    pub fn inf(a: &mut ECP) {
        a.inf=1;
        a.x = BIG::default();
        BIG::one(&mut a.y);
        BIG::one(&mut a.z);
    }

    fn rhs(x: &mut BIG) -> BIG {
        BIG::norm(x);
        let mut r=x.clone();
        BIG::sqrm(&mut r);

        // KLUDGE: depends on CURVETYPE milagro define. This is "CURVETYPE: WEIERSTRASS"
        let b = CURVE_B;
        r = BIG::mulm(&r, &x);
        if CURVE_A == -3 {
            let mut cx=x.clone();
            cx = BIG::imul(&cx, 3);
            BIG::neg(&mut cx);
            BIG::norm(&mut cx);
            r = BIG::add(&r, &cx);
        }
        r = BIG::add(&r, &b);
        BIG::reduce(&mut r);
        return r;
    }

    pub fn new_bigs(ix: &BIG,iy: &BIG) -> ECP {
        let mut E=ECP::default();
        E.x = ix.clone();
        E.y = iy.clone();
        BIG::one(&mut E.z);
        let mut rhs=ECP::rhs(&mut E.x);

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
