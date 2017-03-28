#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;
use self::libc::{c_int, c_char, c_void, uint8_t, uint32_t, int64_t};

// TODO: autogenerate this part!
pub const NLEN:usize = 5;      // use amcl_build command to get this
pub type chunk = int64_t;  // use amcl_build command to get this
pub const MODBYTES:usize = 32; // use amcl_build command to get this
// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

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

pub type BIG = [ chunk; NLEN ];

macro_rules! BIG_ZERO {
    () => {
        [ 0; NLEN ];
    };
}

#[macro_export]
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
    pub fn amcl_version() -> c_void;

    pub fn CREATE_CSPRNG(R: &mut csprng, S: &mut octet) -> c_void;
    pub fn KILL_CSPRNG(R: &mut csprng) -> c_void;

    pub fn FF_random(x: &mut BIG, R: &mut csprng, n: c_int) -> c_void;
    pub fn FF_randomnum(x: &mut BIG, p: &BIG, R: &mut csprng, n: c_int) -> c_void;
    pub fn FF_mul(x: &mut BIG, y: &BIG, z: &BIG, n: c_int) -> c_void;
    pub fn FF_add(x: &mut BIG, y: &BIG, z: &BIG, n: c_int) -> c_void;
    pub fn FF_sub(x: &mut BIG, y: &BIG, z: &BIG, n: c_int) -> c_void;
    pub fn FF_mod(x: &mut BIG, m: &BIG, n: c_int) -> c_void;
    pub fn FF_sqr(x: &mut BIG, y: &BIG, n: c_int) -> c_void;
    pub fn FF_pow(r: &mut BIG, x: &BIG, e: &BIG, m: &BIG, n: c_int) -> c_void;
    pub fn FF_prime(x: &mut BIG, R: &mut csprng, n: c_int) -> c_int;

    pub fn FF_norm(x: &mut BIG, n: c_int) -> c_void;
    pub fn FF_output(x: &BIG, n: c_int) -> c_void;
    pub fn FF_fromOctet(x: &mut BIG, S: &mut octet, n: c_int) -> c_void;
    pub fn FF_toOctet(S: &mut octet, x: &BIG, n: c_int) -> c_void;

    pub fn BIG_nbits(a: &BIG) -> c_int;
    pub fn BIG_copy(d: &mut BIG, s: &BIG) -> c_void;
    pub fn BIG_shr(a: &mut BIG, k: c_int) -> c_void;
}

pub fn big_to_string(a: &BIG) -> String {
    let mut ret: String = String::new();
    let mut b: BIG = BIG_ZERO!();
    let mut len: usize;

    unsafe {
        len = BIG_nbits(a) as usize;
    }

    if len % 4 == 0 {
        len /= 4;
    } else {
        len /= 4;
        len += 1;
    }

    if len < MODBYTES * 2 {
        len=MODBYTES*2;
    }

    for i in (0..len).rev() {
        unsafe {
            BIG_copy(&mut b, &a);
            BIG_shr(&mut b, (i*4) as i32);
        }
        ret.push_str(&format!("{:X}", b[0]&15));
    }

    return ret;
 }

pub fn ff_to_string(x: &mut [BIG], n: c_int) -> String {
    let mut ret:String = String::new();
    unsafe {
        FF_norm(&mut x[0], n);
        for i in (0..n).rev() {
            ret.push_str(big_to_string(&mut x[i as usize]).as_str());
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
            len: 32,
            max: 32,
            val: &val[0]
        };
        unsafe {
            FF_fromOctet(&mut x[0], &mut o, 1);
            FF_fromOctet(&mut y[0], &mut o, 1);
            FF_fromOctet(&mut z[0], &mut o, 1);

            // 3 * 3 + 3 - 3 == 9
            FF_mul(&mut z[0], &mut x[0], &mut y[0], 1);
            println!("3 * 3 = {}", ff_to_string(&mut z, 1));

            FF_add(&mut x[0], &mut z[0], &mut y[0], 1);
            println!("3 * 3 + 3 = {}", ff_to_string(&mut x, 1));

            FF_sub(&mut z[0], &mut x[0], &mut y[0], 1);
            println!("3 * 3 + 3 - 3 = {}", ff_to_string(&mut z, 1));
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
        let mut o = octet {
            len: 32,
            max: 32,
            val: &val[0]
        };
        unsafe {
            FF_fromOctet(&mut x[0], &mut o, 1);
        }
        let str = ff_to_string(&mut x, 1);
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
        let mut o = octet {
            len: 32,
            max: 32,
            val: &val[0]
        };
        unsafe {
            FF_fromOctet(&mut x[0], &mut o, 1);
            FF_fromOctet(&mut y[0], &mut o, 1);
            FF_fromOctet(&mut z[0], &mut o, 1);

            println!("X = {}", ff_to_string(&mut x, 2));
            println!("Y = {}", ff_to_string(&mut y, 2));
            println!("Z = {}", ff_to_string(&mut z, 4));

            for i in 0..171 {
                FF_mul(&mut z[0], &mut x[0], &mut y[0], 2);
                x[0] = z[0];
                x[1] = z[1];
            }
        }
        let str = ff_to_string(&mut z, 4);
        println!("test_more_ops = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000000000 \
                         00000000000000000000000000000000000000000000000000000000000187AF \
                         5D211F2B422CB2A6AFB5E1D3A3B9C65D56BEC8E51AC8D04087A7E0E67AC84C71");
    }
}
