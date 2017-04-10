#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;
use self::libc::{c_int, c_void, uint8_t, uint32_t, int64_t};

use randapi::wrappers::{csprng, octet};
use big::wrappers::{BIG, NLEN, MODBYTES, big_to_hex};

#[macro_export]
macro_rules! FF_ZERO {
    ( $x:expr ) => {
        [ BIG_ZERO!(); $x ];
    };
}

extern {
    pub fn FF_random(x: *mut BIG, R: &mut csprng, n: c_int) -> c_void;
    pub fn FF_randomnum(x: *mut BIG, p: *const BIG, R: &mut csprng, n: c_int) -> c_void;
    pub fn FF_mul(x: *mut BIG, y: *const BIG, z: *const BIG, n: c_int) -> c_void;
    pub fn FF_add(x: *mut BIG, y: *const BIG, z: *const BIG, n: c_int) -> c_void;
    pub fn FF_sub(x: *mut BIG, y: *const BIG, z: *const BIG, n: c_int) -> c_void;
    pub fn FF_mod(x: *mut BIG, m: *const BIG, n: c_int) -> c_void;
    pub fn FF_sqr(x: *mut BIG, y: *const BIG, n: c_int) -> c_void;
    pub fn FF_pow(r: *mut BIG, x: *const BIG, e: *const BIG, m: *const BIG, n: c_int) -> c_void;
    pub fn FF_invmodp(r: *mut BIG, a: *const BIG, p: *const BIG, n: c_int) -> c_void;
    pub fn FF_prime(x: *const BIG, R: &mut csprng, n: c_int) -> c_int;
    pub fn FF_comp(x: *const BIG, y: *const BIG, n: c_int) -> c_int;

    pub fn FF_inc(x: *mut BIG, m: c_int, n: c_int) -> c_void;
    pub fn FF_norm(x: *mut BIG, n: c_int) -> c_void;
    pub fn FF_output(x: *const BIG, n: c_int) -> c_void;
    pub fn FF_fromOctet(x: *mut BIG, S: &mut octet, n: c_int) -> c_void;
    pub fn FF_toOctet(S: &mut octet, x: *const BIG, n: c_int) -> c_void;
}

pub fn ff_to_hex(x: &mut [BIG], n: c_int) -> String {
    let len = n * (2 * MODBYTES as i32 + 1) - 1;
    let mut ret:String = String::with_capacity(len as usize);
    unsafe {
        FF_norm(&mut x[0], n);
        for i in (0..n).rev() {
            ret.push_str(big_to_hex(&mut x[i as usize]).as_str());
            if i > 0 {
                ret.push(' ');
            }
        }
    }
    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let mut x: [BIG; 1] = FF_ZERO!(1);
        let mut y: [BIG; 1] = FF_ZERO!(1);
        let mut z: [BIG; 2] = FF_ZERO!(2);
        let val: [uint8_t; 32] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                   0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                   0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                   0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02 ];
        let mut o = octet::new(&val[0..], 32);
        unsafe {
            FF_fromOctet(&mut x[0], &mut o, 1);
            FF_fromOctet(&mut y[0], &mut o, 1);
            FF_fromOctet(&mut z[0], &mut o, 1);

            FF_inc(&mut x[0], 1, 1);
            FF_inc(&mut y[0], 1, 1);
            FF_inc(&mut z[0], 1, 1);

            // 3 * 3 + 3 - 3 == 9
            FF_mul(&mut z[0], &mut x[0], &mut y[0], 1);
            println!("3 * 3 = {}", ff_to_hex(&mut z, 1));

            FF_add(&mut x[0], &mut z[0], &mut y[0], 1);
            println!("3 * 3 + 3 = {}", ff_to_hex(&mut x, 1));

            FF_sub(&mut z[0], &mut x[0], &mut y[0], 1);
            println!("3 * 3 + 3 - 3 = {}", ff_to_hex(&mut z, 1));
        }
        assert_eq!(z[0][0], 9);
    }

    #[test]
    fn test_strout() {
        let mut x: [BIG; 1] = FF_ZERO!(1);
        let val: [uint8_t; 32] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                   0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                   0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                   0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x03 ];
        let mut o = octet::new(&val[0..], 32);
        unsafe {
            FF_fromOctet(&mut x[0], &mut o, 1);
        }
        let str = ff_to_hex(&mut x, 1);
        println!("strout = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000103");
    }

    #[test]
    fn test_more_ops() {
        let mut x: [BIG; 2] = FF_ZERO!(2);
        let mut y: [BIG; 2] = FF_ZERO!(2);
        let mut z: [BIG; 4] = FF_ZERO!(4);
        let val: [uint8_t; 32] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                   0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                   0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                   0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03 ];
        let mut o = octet::new(&val[0..], 32);
        unsafe {
            FF_fromOctet(&mut x[0], &mut o, 1);
            FF_fromOctet(&mut y[0], &mut o, 1);
            FF_fromOctet(&mut z[0], &mut o, 1);

            println!("X = {}", ff_to_hex(&mut x, 2));
            println!("Y = {}", ff_to_hex(&mut y, 2));
            println!("Z = {}", ff_to_hex(&mut z, 4));

            for _ in 0..171 {
                FF_mul(&mut z[0], &mut x[0], &mut y[0], 2);
                x[0] = z[0];
                x[1] = z[1];
            }
        }
        let str = ff_to_hex(&mut z, 4);
        println!("test_more_ops = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000000000 \
                         00000000000000000000000000000000000000000000000000000000000187AF \
                         5D211F2B422CB2A6AFB5E1D3A3B9C65D56BEC8E51AC8D04087A7E0E67AC84C71");
    }
}
