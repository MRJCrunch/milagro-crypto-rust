#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;
use self::libc::{c_int, c_char, c_void, uint8_t, uint32_t};

pub const NK:usize = 21; // See amcl.h

#[repr(C)]
pub struct csprng {
     ira: [uint32_t; NK],
     rndptr: c_int,
     borrow: uint32_t,
     pool_ptr: c_int,
     pool: [c_char; 32]
}

macro_rules! CSPRNG_INIT {
    () => {
        csprng {
            ira: [0;NK],
            rndptr: 0,
            borrow: 0,
            pool_ptr: 0,
            pool: [0; 32]
        };
    };
}

impl csprng {
    pub fn new() -> csprng {
        CSPRNG_INIT!()
    }
}

#[repr(C)]
pub struct octet<'l> {
    len: c_int,
    max: c_int,
    val: &'l uint8_t
}

impl<'l> octet<'l> {
    pub fn new(val: &[u8], size: usize) -> octet {
        octet {
            len: size as i32,
            max: size as i32,
            val: &val[0]
        }
    }
}

extern {
    pub fn CREATE_CSPRNG(R: *mut csprng, S: *mut octet) -> c_void;
    pub fn KILL_CSPRNG(R: *mut csprng) -> c_void;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng() {
        unsafe {
            let mut rng: csprng = CSPRNG_INIT!();
            let val: [uint8_t; 8] = [0; 8];
            let mut o: octet = octet::new(&val[0..], 8);
            CREATE_CSPRNG(&mut rng, &mut o);
            KILL_CSPRNG(&mut rng);
        }
        // no assert, segfault means test failed
    }
}
