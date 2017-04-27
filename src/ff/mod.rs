#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern crate libc;

use self::libc::{c_int};
use std::fmt;
use std::cmp;

pub mod wrappers;
pub mod overloading;

use big::wrappers::{BIG, MODBYTES};
use randapi::wrappers::{octet};
use ff::wrappers::*;
use randapi::Random;
use std::cmp::Ordering;

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
            arr.push(BIG::default());
        }
        FF {
            storage: arr
        }
    }

    /*
     * len
     */
    pub fn len(&self) -> usize {
        return self.storage.len();
    }

    /*
     * set_size
     */
    pub fn set_size(&mut self, n: usize) -> &FF {
        let nn = cmp::max(2,n) - self.storage.len();
        for _ in 0..nn {
            self.storage.push(BIG::default());
        }
        self
    }

    /*
     * from_bytes
     */
    pub fn from_bytes(val: &[u8], size: usize, bigsize: usize) -> FF {
        assert!(size%MODBYTES==0);
        let blen = size/MODBYTES;
        let mut ret = FF::new(cmp::max(blen, bigsize));
        let mut o = octet::new(val, size);
        unsafe {
            FF_fromOctet(&mut ret.storage.as_mut_slice()[0], &mut o, blen as i32);
        }
        return ret;
    }

    /*
     * from_hex
     */
    pub fn from_hex(val: &str, bigsize: usize) -> FF {
        let mut len: usize = val.len();
        len += 63;
        len &= !63;
        len = cmp::max(len, 2*bigsize*MODBYTES);
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
        return FF::from_bytes(bval.as_slice(), len/2, 0);
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
     * inc
     * self += m
     */
    pub fn inc(&mut self, m: i32) {
        let len = self.storage.len();
        unsafe {
            FF_inc(&mut self.storage.as_mut_slice()[0],
                   m,
                   len as i32);
        }
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
        res.storage.truncate(len);
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
    pub fn modulus(a: &FF, x: &FF) -> FF {
        let len = a.storage.len() as i32;
        let mut r = a.clone();
        unsafe {
            FF_mod(&mut r.storage.as_mut_slice()[0],
                   &x.storage.as_slice()[0],
                   len);
        }
        r
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

    /*
     * Set r=1/a mod p. Binary method - a<p on entry
     */
    pub fn inv(r: &mut FF, a: &FF, p: &FF) {
        let len = a.storage.len() as i32;
        unsafe {
            FF_invmodp(
                &mut r.storage.as_mut_slice()[0],
                &a.storage.as_slice()[0],
                &p.storage.as_slice()[0],
                len
            );
        }
    }
}

impl fmt::Display for FF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl fmt::Debug for FF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}


impl Ord for FF {
    fn cmp(&self, other: &FF) -> Ordering {
        assert_eq!(self.storage.len(), other.storage.len());
        let r = unsafe { FF_comp(&self.storage.as_slice()[0],
                                 &other.storage.as_slice()[0],
                                 self.len() as i32) };
        if r > 0 {
            return Ordering::Greater;
        }
        if r < 0 {
            return Ordering::Less;
        }
        return Ordering::Equal;
    }
}

impl Eq for FF {}

impl PartialOrd for FF {
    fn partial_cmp(&self, other: &FF) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for FF {
    fn eq(&self, other: &FF) -> bool {
        self.storage == other.storage
    }
}

//impl Copy for FF {}
impl Clone for FF {
    fn clone(&self) -> FF {
        FF {
            storage: self.storage.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ff_io() {
        let x = FF::from_hex("112233445566778899AABBCCDDEEFF00112233445566778899AABBCCDDEEFF00", 0);
        let str = x.to_hex();
        println!("ff_io: str = {}", x);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         112233445566778899AABBCCDDEEFF00112233445566778899AABBCCDDEEFF00");
    }

    #[test]
    fn test_ff_io_sized_hex() {
        let x = FF::from_hex("112233445566778899AABBCCDDEEFF00112233445566778899AABBCCDDEEFF00", 4);
        let str = x.to_hex();
        println!("ff_io_sized_hex: str = {}", x);
        assert_eq!(x.len(), 4);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000000000 \
                         112233445566778899AABBCCDDEEFF00112233445566778899AABBCCDDEEFF00");
    }

    #[test]
    fn test_ff_io_sized_bytes() {
        let mut bytes: [ u8; 32 ] = [ 0; 32 ];
        for i in 0..32 {
            bytes[i] = i as u8;
        }
        let x = FF::from_bytes(&bytes[0..], 32, 4);
        let str = x.to_hex();
        println!("ff_io_sized_bytes: str = {}", x);
        assert_eq!(x.len(), 4);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000000000 \
                         000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F");
    }

    #[test]
    fn test_ff_bytes() {
        let mut bytes: [ u8; 32 ] = [ 0; 32 ];
        for i in 0..32 {
            bytes[i] = i as u8;
        }
        let bv = FF::from_bytes(&bytes[0..], 32, 0);
        println!("ff_bytes: bv = {}", bv);
        let obytes = bv.to_bytes();
        println!("ff_bytes: obytes = {:?}", obytes);
        assert_eq!(bytes[0..], obytes[32..]);
    }

    #[test]
    fn test_ff_add() {
        let x = FF::from_hex("1", 0);
        let y = FF::from_hex("1", 0);
        let z = FF::add(&x, &y);
        let str = z.to_hex();
        println!("ff_add: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000000002");
    }

    #[test]
    fn test_ff_sub() {
        let x = FF::from_hex("100", 0);
        let y = FF::from_hex("1", 0);
        let z = FF::sub(&x, &y);
        let str = z.to_hex();
        println!("ff_sub: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         00000000000000000000000000000000000000000000000000000000000000FF");
    }

    #[test]
    fn test_ff_mul() {
        let x = FF::from_hex("101", 0);
        let y = FF::from_hex("101", 0);
        let z = FF::mul(&x, &y);
        let str = z.to_hex();
        println!("ff_mul: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000010201");
    }

    #[test]
    fn test_ff_sqr() {
        let x = FF::from_hex("100", 0);
        let z = FF::sqr(&x);
        let str = z.to_hex();
        println!("ff_sqr: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000010000");
    }

    #[test]
    fn test_ff_modulus() {
        let mut x = FF::from_hex("12345", 0);
        let y = FF::from_hex("10000", 0);
        let r = FF::modulus(&mut x, &y);
        let str = r.to_hex();
        println!("ff_modulus: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000002345");
    }

    #[test]
    fn test_ff_compare() {
        let x = FF::from_hex("12345", 0);
        let y = FF::from_hex("10000", 0);
        assert!(&x > &y);
    }

    #[test]
    fn test_ff_pow() {
        let x = FF::from_hex("3", 0);
        let e = FF::from_hex("20", 0);
        let p = FF::from_hex("10000", 0);
        let z = FF::pow(&x, &e, &p);
        let str = z.to_hex();
        println!("ff_modulus: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000009A1F");

        let b = FF::from_hex("469BF69C74EC5F22BEA6D00598E39469D42537EA8ED6097FE106B40F55399BE\
                              FB0F59C5F777D2A43090A6624E276771578E9E51218982CF7A2BFABE79B1E17\
                              C50482F75AA64C1A9DF202CC689F0A507BEC7C50B500B3C23079988014093DC\
                              09AC6C210D2AE47CD1BB9A0488F53C5A5F9225DEFD59B7CFA07067CECC32DAA\
                              5B5240605C3167337165DC8D81AF62B5634111BA8C0AF28703BCBBEF1BD5398\
                              1627651089FAF7241406A38D465E2790CC741DEC964B7BE90D30AC2370D4B2F\
                              A7AB939180AAF334F010F44B6858F70C790551691C06EAA33249E2873225724\
                              41B0A6CFE49FE274DA94875C70140016B384E229AE03418E869B93ADE6144D9\
                              CDB8035EB5CF54EEBBD1A93D07F1CF35BE4FFEFF7CE5F4D65109373B5997552\
                              8E6C850DEDE85B293921401603AABFD372F63D6ABED1A139E83605A5E0E10CB\
                              67E53774760B74BEAAD57E1119EF3A082549BA270A8CB410936BB976A2E5622\
                              168C72645246F239C63259338FA5DFC961AF1AD18C0E70AEED50E6D369302A7\
                              2BB2EA9E473907F2ED168C390A778F7BF2EEDAC7C81C2088DC397D124FC6540\
                              727D96EA80C98EF9DD22DFBC184EC4788F72C493D488819984B268D706215BC\
                              A01C382BBC98E2F8B1DD02510154BC02F7A7FEED739DAE730EE034EA69BFCA7\
                              B6B5BB0917038924D48CBB7195A30C9B2555127B7A872C9C2998DAC759B0131\
                              612980D9F40E2924", 64);
        let p = FF::from_hex("2F25FA4163E25083717F150B16229C2FA57D56DCB5048C522A64251A23E0334\
                              2D0772B46CF47FC9E66E705B1910BE1B968F45AF1E6FF1D95FE1E319BEC7DC3\
                              4F60E33A664AB202AFBE4098F09AA7F9E233A82AC0C1958B900C2A7B26F8DCE\
                              2EBF774B35ACFBF9A87F682498D5913D476300A558CB536C8FACC9EF6F7A8A8\
                              925CFAE17F913CDDC9D4582C9DFCA648AD074F88113F261839C4342FA8F3365\
                              3979582D7D4C0716FE892371161712F8B77AF31545420D6F075474F8847DDDC\
                              2821B32125FBA3807957E05218E655F5A8C8E7B96F1E1FFF38B9177EF81E30A\
                              E3CACAAF64E5987C2FCFDC197AC2E43800ACF3709AE381B0196F1A1BFF153B6\
                              E93A4088D", 64);
        let two = FF::from_hex("2", p.len());
        let p_sub_2 = FF::sub(&p, &two);
        let b_pow_c = FF::pow(&b, &p_sub_2, &p);
        println!("ff_pow: {} ^ {} mod {}", b, p_sub_2, p);
        println!("ff_pow ret: {}", b_pow_c);
        // TODO: assert?
    }

    #[test]
    fn test_ff_inc() {
        let mut x = FF::from_hex("3", 0);
        x.inc(6);
        let str = x.to_hex();
        println!("ff_inc: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000000009");
    }

    #[test]
    fn test_ff_set_size() {
        let mut x = FF::from_hex("3", 0);
        assert_eq!(x.set_size(5).to_hex(),
                   "0000000000000000000000000000000000000000000000000000000000000000 \
                    0000000000000000000000000000000000000000000000000000000000000000 \
                    0000000000000000000000000000000000000000000000000000000000000000 \
                    0000000000000000000000000000000000000000000000000000000000000000 \
                    0000000000000000000000000000000000000000000000000000000000000003");
    }

    #[test]
    fn test_ff_is_prime() {
        let SEED: Vec<u8> = vec![ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                  0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00,
                                  0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                  0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02 ];
        let mut rng = Random::new(&SEED);
        let bp = FF::from_hex("7FFFFFFF", 0);
        let bn = FF::from_hex("4", 0);
        let p = FF::is_prime(&bp, &mut rng);
        let n = FF::is_prime(&bn, &mut rng);
        println!("ff_is_prime: {} = {}, {} = {}", bp, p, bn, n);
        assert!(p);
        assert!(!n);
    }

    #[test]
    fn test_ff_randoms() {
        let SEED: Vec<u8> = vec![ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                  0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00,
                                  0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                  0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02 ];
        let mut rng = Random::new(&SEED);
        let r = FF::random(&mut rng, 3);
        let bv = FF::from_hex("100", 0);
        let rn = FF::randomnum(&bv, &mut rng);
        println!("ff_randoms: r = {}, rn = {}", r, rn);
    }

    #[test]
    fn test_ff_randomN() {
        let SEED: Vec<u8> = vec![ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                  0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00,
                                  0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                  0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02 ];
        let mut rng = Random::new(&SEED);

        const N: usize = 2048; // number of bits
        const bsize: usize = N / 8 + 1; // number of bytes for mod value
        const bigsize: usize = (bsize + MODBYTES - 1) / MODBYTES; // number of BIGs for mod value

        // init mod bytes with 0 and set 1 in proper place
        let mut bytes: [ u8; bigsize*MODBYTES ] = [ 0; bigsize*MODBYTES ];
        bytes[bigsize*MODBYTES-bsize] = (1 as u8).wrapping_shl((N - (bsize-1)*8) as u32);

        let bv = FF::from_bytes(&bytes[0..], bigsize*MODBYTES, 0);
        let r = FF::randomnum(&bv, &mut rng);
        println!("ff_randomN: bsize = {}, bigsize = {}, bv = {}, r = {}", bsize, bigsize, bv, r);
    }

    #[test]
    fn inv_test() {
        let mut r = FF::new(0);
        let a = FF::from_hex("3", 0);
        let p = FF::from_hex("7", 0);

        FF::inv(&mut r, &a, &p);
        let str = r.to_hex();
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000000005");
    }

    #[test]
    fn plus_test() {
        let x = FF::from_hex("1", 0);
        let y = FF::from_hex("2", 0);
        let z = &x + &y;
        let str = z.to_hex();
        println!("ff_mul: x = {}", &x);
        println!("ff_mul: x = {}", &y);
        println!("ff_mul: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000000003");
    }

    #[test]
    fn sub_test() {
        let x = FF::from_hex("2", 0);
        let y = FF::from_hex("1", 0);
        let z = &x - &y;
        let str = z.to_hex();
        println!("ff_mul: x = {}", &x);
        println!("ff_mul: x = {}", &y);
        println!("ff_mul: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000000001");
    }

    #[test]
    fn mul_test() {
        let x = FF::from_hex("2", 0);
        let y = FF::from_hex("1", 0);
        let z = &x * &y;
        let str = z.to_hex();
        println!("ff_mul: x = {}", &x);
        println!("ff_mul: x = {}", &y);
        println!("ff_mul: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000000002");
    }

    #[test]
    fn test_copy() {
        let x = FF::from_hex("2", 0);
        let y = x.clone();
        assert!(x == y);
    }
}
