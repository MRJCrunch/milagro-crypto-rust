#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;
use self::libc::{c_int};
use std::fmt;
use std::cmp;

pub mod wrappers;

use big::wrappers::{BIG, NLEN, MODBYTES};
use randapi::wrappers::{octet};
use ff::wrappers::*;
use randapi::Random;

pub struct FF {
    storage: Vec<BIG>
}

impl FF {
    /*
     * New
     */
    pub fn new(n: usize) -> FF {
        let len = cmp::max(n,2);
        let mut arr = Vec::<BIG>::with_capacity(len);
        for _ in 0..len {
            arr.push(BIG_ZERO!());
        }
        FF {
            storage: arr
        }
    }

    /*
     * from_bytes
     */
    pub fn from_bytes(val: &[u8], size: usize) -> FF {
        let blen = size/MODBYTES;
        let mut ret = FF::new(blen);
        let mut o = octet::new(val, size);
        unsafe {
            FF_fromOctet(&mut ret.storage.as_mut_slice()[0], &mut o, blen as i32);
        }
        return ret;
    }

    /*
     * from_hex
     */
    pub fn from_hex(val: &str) -> FF {
        let mut len: usize = val.len();
        len += 63;
        len &= !63;
        let mut bval = Vec::<u8>::with_capacity(len/2);
        let mut padded:String = String::with_capacity(len);
        for _ in 0..(len - val.len()) {
            padded.push('0');
        }
        padded.push_str(val);
        for i in 0..(len/2) {
            let hex = &padded[2*i..2*i+2];
            let b: u8 = u8::from_str_radix(hex, 16).unwrap();
            bval.push(b);
        }
        return FF::from_bytes(bval.as_slice(), len/2);
    }

    /*
     * to_hex
     */
    pub fn to_hex(&self) -> String {
        let len = self.storage.len() as i32;
        let mut slice = self.storage.clone();
        return ff_to_hex(slice.as_mut_slice(), len);
    }

    /*
     * to_bytes
     */
    pub fn to_bytes(&self) -> Vec<u8> {
        let len = self.storage.len();
        let mut val = vec![0; len*MODBYTES];
        let mut ret:Vec<u8> = Vec::<u8>::with_capacity(len*MODBYTES);
        unsafe {
            let mut octet = octet::new(val.as_mut_slice(), len*MODBYTES);
            FF_toOctet(&mut octet, &self.storage.as_slice()[0], len as i32);
        }
        for i in 0..len*MODBYTES {
            ret.push(val[i]);
        }
        return ret;
    }

    /*
     * add
     * a + b -> r
     */
    pub fn add(a: &FF, b: &FF) -> FF {
        let len = a.storage.len();
        assert_eq!(a.storage.len(), b.storage.len());
        let mut res = FF::new(len);
        unsafe {
            FF_add(&mut res.storage.as_mut_slice()[0],
                   &a.storage.as_slice()[0],
                   &b.storage.as_slice()[0],
                   len as i32);
        }
        return res;
    }

    /*
     * sub
     * a - b -> r
     */
    pub fn sub(a: &FF, b: &FF) -> FF {
        let len = a.storage.len();
        assert_eq!(a.storage.len(), b.storage.len());
        let mut res = FF::new(len);
        unsafe {
            FF_sub(&mut res.storage.as_mut_slice()[0],
                   &a.storage.as_slice()[0],
                   &b.storage.as_slice()[0],
                   len as i32);
        }
        return res;
    }

    /*
     * mul
     * a * b -> r
     * TODO: check if double result size is ok
     */
    pub fn mul(a :&FF, b: &FF) -> FF {
        let len = a.storage.len();
        assert_eq!(a.storage.len(), b.storage.len());
        let mut res = FF::new(2*len);
        unsafe {
            FF_mul(&mut res.storage.as_mut_slice()[0],
                   &a.storage.as_slice()[0],
                   &b.storage.as_slice()[0],
                   len as i32);
        }
        return res;
    }

    /*
     * sqr
     * a^2 -> r
     */
    pub fn sqr(a: &FF) -> FF {
        let len = a.storage.len();
        let mut res = FF::new(len);
        unsafe {
            FF_sqr(&mut res.storage.as_mut_slice()[0],
                   &a.storage.as_slice()[0],
                   len as i32);
        }
        return res;
    }

    /*
     * mod
     * a = a mod x
     */
    pub fn modulus(a: &mut FF, x: &FF) {
        let len = a.storage.len() as i32;
        unsafe {
            FF_mod(&mut a.storage.as_mut_slice()[0],
                   &x.storage.as_slice()[0],
                   len);
        }
    }

    /*
     * pow
     * x^e mod p -> r
     */
    pub fn pow(x: &FF, e: &FF, p: &FF) -> FF {
        let len = p.storage.len();
        let mut res = FF::new(len);
        unsafe {
            FF_pow(&mut res.storage.as_mut_slice()[0],
                   &x.storage.as_slice()[0],
                   &e.storage.as_slice()[0],
                   &p.storage.as_slice()[0],
                   len as i32);
        }
        return res;
    }

    /*
     * is_prime
     */
    pub fn is_prime(x: &FF, rng: &mut Random) -> bool {
        let ret: c_int;
        let len = x.storage.len();
        unsafe {
            ret = FF_prime(&x.storage.as_slice()[0],
                           &mut rng.rng,
                           len as i32);
        }
        return ret != 0;
    }

    /*
     * random
     */
    pub fn random(rng: &mut Random, size: usize) -> FF {
        let mut res = FF::new(size);
        unsafe {
            FF_random(&mut res.storage.as_mut_slice()[0],
                      &mut rng.rng,
                      size as i32);
        }
        return res;
    }

    /*
     * randomnum
     */
    pub fn randomnum(x: &FF, rng: &mut Random) -> FF{
        let mut res = FF::new(x.storage.len());
        unsafe {
            FF_randomnum(&mut res.storage.as_mut_slice()[0],
                         &x.storage.as_slice()[0],
                         &mut rng.rng,
                         x.storage.len() as i32);
        }
        return res;
    }
}


impl fmt::Display for FF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const SEED: [u8; 32] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                             0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00,
                             0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                             0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02 ];

    #[test]
    fn test_ff_io() {
        let x = FF::from_hex("112233445566778899AABBCCDDEEFF00112233445566778899AABBCCDDEEFF00");
        let str = x.to_hex();
        println!("ff_io: str = {}", x);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         112233445566778899AABBCCDDEEFF00112233445566778899AABBCCDDEEFF00");
    }

    #[test]
    fn test_ff_bytes() {
        let mut bytes: [ u8; 32 ] = [ 0; 32 ];
        for i in 0..32 {
            bytes[i] = i as u8;
        }
        let bv = FF::from_bytes(&bytes[0..], 32);
        println!("ff_bytes: bv = {}", bv);
        let obytes = bv.to_bytes();
        println!("ff_bytes: obytes = {:?}", obytes);
        assert_eq!(bytes[0..], obytes[32..]);
    }

    #[test]
    fn test_ff_add() {
        let x = FF::from_hex("1");
        let y = FF::from_hex("1");
        let z = FF::add(&x, &y);
        let str = z.to_hex();
        println!("ff_add: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000000002");
    }

    #[test]
    fn test_ff_sub() {
        let x = FF::from_hex("100");
        let y = FF::from_hex("1");
        let z = FF::sub(&x, &y);
        let str = z.to_hex();
        println!("ff_sub: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         00000000000000000000000000000000000000000000000000000000000000FF");
    }

    #[test]
    fn test_ff_mul() {
        let x = FF::from_hex("101");
        let y = FF::from_hex("101");
        let z = FF::mul(&x, &y);
        let str = z.to_hex();
        println!("ff_mul: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000010201");
    }

    #[test]
    fn test_ff_sqr() {
        let x = FF::from_hex("100");
        let z = FF::sqr(&x);
        let str = z.to_hex();
        println!("ff_sqr: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000010000");
    }

    #[test]
    fn test_ff_modulus() {
        let mut x = FF::from_hex("12345");
        let y = FF::from_hex("10000");
        FF::modulus(&mut x, &y);
        let str = x.to_hex();
        println!("ff_modulus: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000002345");
    }

    #[test]
    fn test_ff_pow() {
        let x = FF::from_hex("3");
        let e = FF::from_hex("20");
        let mut p = FF::from_hex("10000");
        let z = FF::pow(&x, &e, &p);
        let str = z.to_hex();
        println!("ff_modulus: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000009A1F");
    }

    #[test]
    fn test_ff_is_prime() {
        let mut rng = Random::new(SEED);
        let mut bp = FF::from_hex("7FFFFFFF");
        let mut bn = FF::from_hex("4");
        let p = FF::is_prime(&bp, &mut rng);
        let n = FF::is_prime(&bn, &mut rng);
        println!("ff_is_prime: {} = {}, {} = {}", bp, p, bn, n);
        assert!(p);
        assert!(!n);
    }

    #[test]
    fn test_ff_randoms() {
        let mut rng = Random::new(SEED);
        let r = FF::random(&mut rng, 3);
        let bv = FF::from_hex("100");
        let rn = FF::randomnum(&bv, &mut rng);
        println!("ff_randoms: r = {}, rn = {}", r, rn);
    }

    #[test]
    fn test_ff_randomN() {
        let mut rng = Random::new(SEED);

        const N: usize = 2048; // number of bits
        const bsize: usize = N / 8 + 1; // number of bytes for mod value
        const bigsize: usize = (bsize + MODBYTES - 1) / MODBYTES; // number of BIGs for mod value

        // init mod bytes with 0 and set 1 in proper place
        let mut bytes: [ u8; bigsize*MODBYTES ] = [ 0; bigsize*MODBYTES ];
        bytes[bigsize*MODBYTES-bsize] = (1 as u8).wrapping_shl((N - (bsize-1)*8) as u32);

        let bv = FF::from_bytes(&bytes[0..], bigsize*MODBYTES);
        let r = FF::randomnum(&bv, &mut rng);
        println!("ff_randomN: bsize = {}, bigsize = {}, bv = {}, r = {}", bsize, bigsize, bv, r);
    }
}
