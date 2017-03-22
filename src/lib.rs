#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;

use std::mem;
use libc::{c_int, c_char, c_void, uint32_t, int64_t};

// TODO: autogenerate this part!
const NK:usize = 21;      // See amcl.h
const NLEN:usize = 5;     // use amcl_build command to get this
pub type chunk = int64_t; // use amcl_build command to get this
// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

#[repr(C)]
pub struct csprng {
     ira: [uint32_t; NK],
     rndptr: c_int,
     borrow: uint32_t,
     pool_ptr: c_int,
     pool: [c_char; 32]
}

pub type BIG = [chunk; NLEN];
pub type octet = c_char;

extern {
    pub fn amcl_version() -> c_void;

    pub fn CREATE_CSPRNG(R: &mut csprng, S: &mut octet) -> c_void;
    pub fn KILL_CSPRNG(R: &mut csprng) -> c_void;

    pub fn FF_random(x: &mut BIG, R: &mut csprng, n: c_int) -> c_void;
    pub fn FF_mul(x: &mut BIG, y: &mut BIG, z: &mut BIG, n: c_int) -> c_void;
    pub fn FF_add(x: &mut BIG, y: &mut BIG, z: &mut BIG, n: c_int) -> c_void;
    pub fn FF_sub(x: &mut BIG, y: &mut BIG, z: &mut BIG, n: c_int) -> c_void;
    pub fn FF_mod(x: &mut BIG, m: &mut BIG, n: c_int) -> c_void;
    pub fn FF_sqr(x: &mut BIG, y: &mut BIG, n: c_int) -> c_void;
    pub fn FF_pow(r: &mut BIG, x: &mut BIG, e: &mut BIG, m: &mut BIG, n: c_int) -> c_void;
    pub fn FF_prime(x: &mut BIG, R: &mut csprng, n: c_int) -> c_int;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amcl_version() {
        unsafe {
            amcl_version();
        }
    }

    #[test]
    fn test_rng() {
        unsafe {
            let mut rng: csprng = mem::zeroed();
            let mut o: octet = 0;
            CREATE_CSPRNG(&mut rng, &mut o);
            KILL_CSPRNG(&mut rng);
        }
    }
}
