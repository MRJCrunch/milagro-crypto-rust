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
