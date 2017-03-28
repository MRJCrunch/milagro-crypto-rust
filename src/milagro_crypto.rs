extern crate libc;
use self::libc::{c_int};
use std::fmt;

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
        let mut arr = Vec::<BIG>::with_capacity(n);
        for i in 0..n {
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
     * to_string
     */
    pub fn to_string(&mut self) -> String {
        let len = self.storage.len() as i32;
        return ff_to_string(self.storage.as_mut_slice(), len);
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
        write!(f, "{}", self.to_string())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bignum_io() {
        let val: [u8; 32] = [ 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
                              0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00,
                              0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
                              0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00 ];
        let x = BigNum::from_bytes(&val[0..], 32);
        let str = x.to_string();
        println!("bignum_io: str = {}", x);
        assert_eq!(str, "112233445566778899AABBCCDDEEFF00112233445566778899AABBCCDDEEFF00");
    }

    #[test]
    fn test_bignum_add() {
        let val: [u8; 32] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01 ];
        let mut x = BigNum::from_bytes(&val[0..], 32);
        let y = BigNum::from_bytes(&val[0..], 32);
        x.add(y);
        let str = x.to_string();
        println!("bignum_add: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000002");
    }

    #[test]
    fn test_bignum_sub() {
        let valx: [u8; 32] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01 ];
        let valy: [u8; 32] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02 ];
        let mut x = BigNum::from_bytes(&valx[0..], 32);
        let y = BigNum::from_bytes(&valy[0..], 32);
        x.sub(y);
        let str = x.to_string();
        println!("bignum_sub: str = {}", str);
        assert_eq!(str, "00000000000000000000000000000000000000000000000000000000000000FF");
    }

    #[test]
    fn test_bignum_mul() {
        let val: [u8; 32] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01 ];
        let mut x = BigNum::from_bytes(&val[0..], 32);
        let y = BigNum::from_bytes(&val[0..], 32);
        x.mul(y);
        let str = x.to_string();
        println!("bignum_mul: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000010201");
    }

    #[test]
    fn test_bignum_sqr() {
        let val: [u8; 32] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00 ];
        let mut x = BigNum::from_bytes(&val[0..], 32);
        x.sqr();
        let str = x.to_string();
        println!("bignum_sqr: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000010000");
    }

    #[test]
    fn test_bignum_modulus() {
        let valx: [u8; 32] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x23, 0x45 ];
        let valy: [u8; 32] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00 ];
        let mut x = BigNum::from_bytes(&valx[0..], 32);
        let y = BigNum::from_bytes(&valy[0..], 32);
        x.modulus(y);
        let str = x.to_string();
        println!("bignum_modulus: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000002345");
    }

    #[test]
    fn test_bignum_pow() {
        let valx: [u8; 32] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03 ];
        let vale: [u8; 32] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20 ];
        let valp: [u8; 32] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00 ];
        let mut x = BigNum::from_bytes(&valx[0..], 32);
        let e = BigNum::from_bytes(&vale[0..], 32);
        let p = BigNum::from_bytes(&valp[0..], 32);
        x.pow(e,p);
        let str = x.to_string();
        println!("bignum_modulus: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000003E81");
    }

    #[test]
    fn test_bignum_is_prime() {
        let seed: [u8; 32] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02 ];
        let valn: [u8; 32] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02 ];
        let valp: [u8; 32] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x79 ];
        let mut bp = BigNum::from_bytes(&valp[0..], 32);
        let mut bn = BigNum::from_bytes(&valn[0..], 32);
        let mut rng = Random::new(seed);
        let p = bp.is_prime(&mut rng);
        let n = bn.is_prime(&mut rng);
        println!("bignum_is_prime: p = {}, n = {}", p, n);
        assert!(p);
        assert!(!n);
    }

    #[test]
    fn test_bignum_randoms() {
        let seed: [u8; 32] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02 ];
        let mut r = BigNum::new(1);
        let mut rn = BigNum::new(1);
        let mut rng = Random::new(seed);

        let modv: [u8; 32] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00 ];
        let bv = BigNum::from_bytes(&modv[0..], 32);

        r.random(&mut rng);
        rn.randomnum(bv, &mut rng);

        println!("bignum_randoms: r = {}, rn = {}", r, rn);
    }
}
