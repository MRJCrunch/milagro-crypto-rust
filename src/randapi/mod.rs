#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub mod wrappers;

extern crate libc;
use self::libc::{c_int};

use randapi::wrappers::*;

pub struct Random {
    pub rng: csprng
}

impl Random {
    pub fn new(seed: &Vec<u8>) -> Random {
        let mut ret: csprng = csprng::new();
        let mut o = octet::new(&seed.as_slice(), seed.len());
        unsafe {
            CREATE_CSPRNG(&mut ret, &mut o);
        }
        Random {
            rng: ret
        }
    }

    pub fn getbyte(&mut self) -> u8 {
        let r: c_int;
        unsafe {
            r = RAND_byte(&mut self.rng);
        }
        return r as u8;
    }
}

impl Drop for Random {
    fn drop(&mut self) {
        unsafe {
            KILL_CSPRNG(&mut self.rng);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        let seed: Vec<u8> = vec![ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                  0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00,
                                  0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                  0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02 ];
        Random::new(&seed);
        // no assert, segfault means test failed
    }
}
