#[macro_use]
pub mod wrappers;

extern crate libc;
use self::libc::{c_int};

use std::cmp::Ordering;
use std::fmt;
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

    pub fn add(c: &mut BIG, a: &BIG, b: &BIG) {
        unsafe {
            BIG_add(c, a, b);
        }
    }

    pub fn rmod(b: &mut BIG, c: &BIG) {
        unsafe {
            BIG_mod(b, c);
        }
    }

    pub fn modmul(r: &mut BIG, a: &mut BIG, b: &mut BIG, m: &BIG) {
        unsafe {
            BIG_modmul(r, a, b, m);
        }
    }

    pub fn modneg(r: &mut BIG, a: &mut BIG, m: &BIG) {
        unsafe {
            BIG_modneg(r, a, m);
        }
    }

    pub fn toBytes(b: &mut [u8], a: &BIG) {
        unsafe {
            BIG_toBytes(&mut b[0], a);
        }
    }

    pub fn fromBytes(b: &[u8]) -> BIG {
        let mut ret: BIG = BIG::default();
        unsafe {
            BIG_fromBytes(&mut ret, b.as_ptr());
        }
        return ret;
    }
}

impl fmt::Display for BIG {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BIG: [{}]", big_to_hex(self))
    }
}

impl fmt::Debug for BIG {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BIG: [{}]", big_to_hex(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes() {
        let mut bytes: [u8; MODBYTES] = [0; MODBYTES];
        let mut outbytes: [u8; MODBYTES] = [0; MODBYTES];
        bytes[0] = 0xFF;
        let a: BIG = BIG::fromBytes(&bytes[..]);
        BIG::toBytes(&mut outbytes[..], &a);
        assert_eq!(bytes, outbytes);
    }
}
