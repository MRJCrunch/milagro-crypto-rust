#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;

use libc::{c_int, c_char, c_void, uint8_t, uint32_t, int64_t};

// TODO: autogenerate this part!
const NLEN:usize = 5;     // use amcl_build command to get this
pub type chunk = int64_t; // use amcl_build command to get this
// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

const NK:usize = 21; // See amcl.h

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

pub type BIG = [ chunk; NLEN ];

macro_rules! BIG_ZERO {
    () => {
        [ 0; NLEN ];
    };
}

macro_rules! FF_ZERO {
    ( $x:expr ) => {
        [ BIG_ZERO!(); $x ];
    };
}

#[repr(C)]
pub struct octet<'l> {
    len: c_int,
    max: c_int,
    val: &'l uint8_t
}

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

    pub fn FF_norm(x: &mut BIG, n: c_int) -> c_void;
    pub fn FF_output(x: &mut BIG, n: c_int) -> c_void;
    pub fn FF_fromOctet(x: &mut BIG, S: &mut octet, n: c_int) -> c_void;
    pub fn FF_toOctet(S: &mut octet, x: &mut BIG, n: c_int) -> c_void;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amcl_version() {
        unsafe {
            amcl_version();
        }
        // no assert, segfault means test failed
    }

    #[test]
    fn test_rng() {
        unsafe {
            let mut rng: csprng = CSPRNG_INIT!();
            let val: [uint8_t; 8] = [0; 8];
            let mut o: octet = octet {
                len: 8,
                max: 8,
                val: &val[0]
            };
            CREATE_CSPRNG(&mut rng, &mut o);
            KILL_CSPRNG(&mut rng);
        }
        // no assert, segfault means test failed
    }

    #[test]
    fn test_ops() {
        let mut x: [BIG; 1] = FF_ZERO!(1);
        let mut y: [BIG; 1] = FF_ZERO!(1);
        let mut z: [BIG; 2] = FF_ZERO!(2);
        let val: [uint8_t; 32] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                   0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                   0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                   0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03 ];
        let mut o = octet {
            len: 8,
            max: 8,
            val: &val[0]
        };
        unsafe {

            FF_fromOctet(&mut x[0], &mut o, 1);
            FF_fromOctet(&mut y[0], &mut o, 1);
            FF_fromOctet(&mut z[0], &mut o, 2);

            FF_mul(&mut z[0], &mut x[0], &mut y[0], 1);
            FF_output(&mut z[0], 1);

            FF_add(&mut x[0], &mut z[0], &mut y[0], 1);
            FF_output(&mut x[0], 1);

            FF_sub(&mut z[0], &mut x[0], &mut y[0], 1);
            FF_output(&mut z[0], 1);

        }
        assert_eq!(z[0][0], 9);
    }
}
