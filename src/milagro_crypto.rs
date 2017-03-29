extern crate libc;
use self::libc::{c_int};
use std::fmt;
use std::cmp;

#[macro_use]
use wrappers::*;

pub struct Random {
    rng: csprng
}

impl Random {
    pub fn new(seed: [u8; 32]) -> Random {
        let mut ret: csprng = csprng::new();
        let mut o = octet::new(&seed[0..], 32);
        unsafe {
            CREATE_CSPRNG(&mut ret, &mut o);
        }
        Random {
            rng: ret
        }
    }
}

impl Drop for Random {
    fn drop(&mut self) {
        unsafe {
            KILL_CSPRNG(&mut self.rng);
        }
    }
}

pub struct BigNum {
    storage: Vec<BIG>
}

impl BigNum {
    // TODO: check/assert BigNum length!

    /*
     * New
     */
    pub fn new(n: usize) -> BigNum {
        let mut arr = Vec::<BIG>::with_capacity(cmp::max(n,2));
        for _ in 0..n {
            arr.push(BIG_ZERO!());
        }
        BigNum {
            storage: arr
        }
    }

    /*
     * from_bytes
     */
    pub fn from_bytes(val: &[u8], size: usize) -> BigNum {
        let blen = size/32;
        let mut ret = BigNum::new(blen);
        let mut o = octet::new(val, size);
        unsafe {
            FF_fromOctet(&mut ret.storage.as_mut_slice()[0], &mut o, blen as i32);
        }
        return ret;
    }

    /*
     * from_hex
     */
    pub fn from_hex(val: &str) -> BigNum {
        let mut len: usize = val.len();
        len += 63;
        len &= !63;
        let mut bval = Vec::<u8>::with_capacity(len);
        let mut padded:String = String::new();
        for _ in 0..(len - val.len()) {
            padded.push('0');
        }
        padded.push_str(val);
        for i in 0..(len/2) {
            let hex = &padded[2*i..2*i+2];
            let b: u8 = u8::from_str_radix(hex, 16).unwrap();
            bval.push(b);
        }
        return BigNum::from_bytes(bval.as_slice(), len/2);
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
     * set_size
     */
    pub fn set_size(&mut self, n: usize) {
        let nn = cmp::max(2,n) - self.storage.len();
        for _ in 0..nn {
            self.storage.push(BIG_ZERO!());
        }
    }

    /*
     * add
     * self += val
     */
    pub fn add(&mut self, val: BigNum) {
        let mut res = BigNum::new(self.storage.len());
        unsafe {
            FF_add(&mut res.storage.as_mut_slice()[0],
                   &self.storage.as_slice()[0],
                   &val.storage.as_slice()[0],
                   self.storage.len() as i32);
        }
        self.storage = res.storage;
    }

    /*
     * sub
     * self -= val
     */
    pub fn sub(&mut self, val: BigNum) {
        let mut res = BigNum::new(self.storage.len());
        unsafe {
            FF_sub(&mut res.storage.as_mut_slice()[0],
                   &self.storage.as_slice()[0],
                   &val.storage.as_slice()[0],
                   self.storage.len() as i32);
        }
        self.storage = res.storage;
    }

    /*
     * mul
     * self *= val
     */
    pub fn mul(&mut self, val: BigNum) {
        let mut res = BigNum::new(2*self.storage.len());
        unsafe {
            FF_mul(&mut res.storage.as_mut_slice()[0],
                   &self.storage.as_slice()[0],
                   &val.storage.as_slice()[0],
                   self.storage.len() as i32);
        }
        self.storage = res.storage;
    }

    /*
     * sqr
     * self = self^2
     */
    pub fn sqr(&mut self) {
        let mut res = BigNum::new(2*self.storage.len());
        unsafe {
            FF_sqr(&mut res.storage.as_mut_slice()[0],
                   &self.storage.as_slice()[0],
                   self.storage.len() as i32);
        }
        self.storage = res.storage;
    }

    /*
     * mod
     * self = self mod x
     */
    pub fn modulus(&mut self, x: BigNum) {
        let len = self.storage.len() as i32;
        unsafe {
            FF_mod(&mut self.storage.as_mut_slice()[0],
                   &x.storage.as_slice()[0],
                   len);
        }
    }

    /*
     * pow
     * self=self^e mod p
     */
    pub fn pow(&mut self, e: BigNum, p: BigNum) {
        let len = self.storage.len();
        let mut res = BigNum::new(len);
        unsafe {
            FF_pow(&mut res.storage.as_mut_slice()[0],
                   &self.storage.as_slice()[0],
                   &e.storage.as_slice()[0],
                   &p.storage.as_slice()[0],
                   len as i32);
        }
        self.storage = res.storage;
    }

    /*
     * is_prime
     */
    pub fn is_prime(&mut self, rng: &mut Random) -> bool {
        let ret: c_int;
        let len = self.storage.len();
        unsafe {
            ret = FF_prime(&mut self.storage.as_mut_slice()[0],
                           &mut rng.rng,
                           len as i32);
        }
        return ret != 0;
    }

    /*
     * random
     */
    pub fn random(&mut self, rng: &mut Random) {
        let len = self.storage.len();
        unsafe {
            FF_random(&mut self.storage.as_mut_slice()[0],
                      &mut rng.rng,
                      len as i32);
        }
    }

    /*
     * randomnum
     */
    pub fn randomnum(&mut self, x: BigNum, rng: &mut Random) {
        let len = self.storage.len();
        unsafe {
            FF_randomnum(&mut self.storage.as_mut_slice()[0],
                         &x.storage.as_slice()[0],
                         &mut rng.rng,
                         len as i32);
        }
    }
}


impl fmt::Display for BigNum {
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
    fn test_bignum_io() {
        let x = BigNum::from_hex("112233445566778899AABBCCDDEEFF00112233445566778899AABBCCDDEEFF00");
        let str = x.to_hex();
        println!("bignum_io: str = {}", x);
        assert_eq!(str, "112233445566778899AABBCCDDEEFF00112233445566778899AABBCCDDEEFF00");
    }

    #[test]
    fn test_bignum_add() {
        let mut x = BigNum::from_hex("1");
        let y = BigNum::from_hex("1");
        x.add(y);
        let str = x.to_hex();
        println!("bignum_add: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000002");
    }

    #[test]
    fn test_bignum_sub() {
        let mut x = BigNum::from_hex("100");
        let y = BigNum::from_hex("1");
        x.sub(y);
        let str = x.to_hex();
        println!("bignum_sub: str = {}", str);
        assert_eq!(str, "00000000000000000000000000000000000000000000000000000000000000FF");
    }

    #[test]
    fn test_bignum_mul() {
        let mut x = BigNum::from_hex("101");
        let y = BigNum::from_hex("101");
        x.mul(y);
        let str = x.to_hex();
        println!("bignum_mul: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000010201");
    }

    #[test]
    fn test_bignum_sqr() {
        let mut x = BigNum::from_hex("100");
        x.sqr();
        let str = x.to_hex();
        println!("bignum_sqr: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000010000");
    }

    #[test]
    fn test_bignum_modulus() {
        let mut x = BigNum::from_hex("12345");
        let y = BigNum::from_hex("10000");
        x.modulus(y);
        let str = x.to_hex();
        println!("bignum_modulus: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000002345");
    }

    #[test]
    fn test_bignum_pow() {
        let mut x = BigNum::from_hex("3");
        let e = BigNum::from_hex("20");
        let p = BigNum::from_hex("10000");
        x.set_size(2); // pow needs 2 BIGs at least, infinite recursion otherwise
        x.pow(e,p);
        let str = x.to_hex();
        println!("bignum_modulus: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000009A1F");
    }

    #[test]
    fn test_bignum_is_prime() {
        let mut rng = Random::new(SEED);
        let mut bp = BigNum::from_hex("7FFFFFFF");
        let mut bn = BigNum::from_hex("4");
        bp.set_size(2); // prime->pow needs 2 BIGs at least, infinite recursion otherwise
        bn.set_size(2); // prime->pow needs 2 BIGs at least, infinite recursion otherwise
        let p = bp.is_prime(&mut rng);
        let n = bn.is_prime(&mut rng);
        println!("bignum_is_prime: {} = {}, {} = {}", bp, p, bn, n);
        assert!(p);
        assert!(!n);
    }

    #[test]
    fn test_bignum_randoms() {
        let mut r = BigNum::new(1);
        let mut rn = BigNum::new(1);
        let mut rng = Random::new(SEED);
        let bv = BigNum::from_hex("100");
        r.random(&mut rng);
        rn.randomnum(bv, &mut rng);
        println!("bignum_randoms: r = {}, rn = {}", r, rn);
    }
}
