#[macro_use]
pub mod wrappers;

extern crate libc;
use self::libc::{c_int};

use std::cmp::Ordering;
use big::wrappers::*;

impl Ord for BIG {
    fn cmp(&self, other: &BIG) -> Ordering {
        let r = unsafe { BIG_comp(self, other) };
        if r > 0 {
            return Ordering::Greater;
        }
        if r < 0 {
            return Ordering::Less;
        }
        return Ordering::Equal;
    }
}

impl Eq for BIG {}

impl PartialOrd for BIG {
    fn partial_cmp(&self, other: &BIG) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BIG {
    fn eq(&self, other: &BIG) -> bool {
        self.val == other.val
    }
}

impl Copy for BIG { }

impl Clone for BIG {
    fn clone(&self) -> BIG {
        BIG {
            val: self.val
        }
    }
}

impl BIG {
    pub fn nbits(a: &BIG) -> i32 {
        let mut ret;
        unsafe {
            ret = BIG_nbits(a) as i32;
        }
        return ret;
    }

    pub fn copy(d: &mut BIG, s: &BIG) {
        unsafe {
            BIG_copy(d, s);
        }
    }

    pub fn shr(a: &mut BIG, k: i32) {
        unsafe {
            BIG_shr(a, k as c_int);
        }
    }

    pub fn rcopy(b: &mut BIG, a: &BIG) {
        unsafe {
            BIG_copy(b, a);
        }
    }

    pub fn comp(a: &BIG, b: &BIG) -> i32 {
        let mut ret;
        unsafe {
            ret = BIG_comp(a, b) as i32;
        }
        return ret;
    }
}
