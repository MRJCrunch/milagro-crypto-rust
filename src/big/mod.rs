#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

#[macro_use]
pub mod wrappers;

extern crate libc;
use self::libc::{c_int};

use std::cmp::Ordering;
use std::fmt;
use std::str::SplitWhitespace;
use big::wrappers::*;
use randapi::Random;

impl Ord for BIG {
    fn cmp(&self, other: &BIG) -> Ordering {
        let r = unsafe { BIG_comp(self, other) };
        if r > 0 {
            return Ordering::Greater;
        }
        if r < 0 {
            return Ordering::Less;
        }
        return Ordering::Equal;
    }
}

impl Eq for BIG { }

impl PartialOrd for BIG {
    fn partial_cmp(&self, other: &BIG) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BIG {
    fn eq(&self, other: &BIG) -> bool {
        return self.cmp(other) == Ordering::Equal;
    }
}

impl Copy for BIG { }

impl Clone for BIG {
    fn clone(&self) -> BIG {
        BIG {
            val: self.val
        }
    }
}

impl BIG {
    pub fn new_int(x:isize) -> BIG {
        let mut s= BIG::default();
        s.val[0]=x as chunk;
        return s;
    }

    pub fn new_ints(a:&[chunk]) -> BIG {
        let mut s= BIG::default();
        for i in 0..NLEN {
            s.val[i]=a[i];
        }
        return s;
    }

    pub fn new_copy(y:&BIG) -> BIG {
        let mut s= BIG::default();
        for i in 0..NLEN {
            s.val[i]=y.val[i];
        }
        return s;
    }

    pub fn iszilch(a: &BIG) -> bool {
        for i in 0 ..NLEN {
            if a.val[i]!=0 {
                return false;
            }
        }
        return true;
    }

    pub fn parity(a: &BIG) -> isize {
        return (a.val[0]%2) as isize;
    }

    pub fn shl(&mut self, k: usize) {
	let n=k%BASEBITS;
	let m=k/BASEBITS;

	self.val[NLEN-1]=self.val[NLEN-1-m]<<n;
	if NLEN>=m+2 {self.val[NLEN-1]|=self.val[NLEN-m-2]>>(BASEBITS-n)}
	for i in (m+1 ..NLEN-1).rev() {
	    self.val[i]=((self.val[i-m]<<n)&BMASK)|(self.val[i-m-1]>>(BASEBITS-n));
	}
	self.val[m]=(self.val[0]<<n)&BMASK;
	for i in 0 ..m {
            self.val[i]=0;
        }
    }

    pub fn powmod(x: &mut BIG, e: &mut BIG, m: &BIG) -> BIG {
        BIG::norm(x);
        BIG::norm(e);
        let mut a=BIG::new_int(1);
        let mut z=BIG::new_copy(e);
        let mut s=BIG::new_copy(x);
        loop {
            let bt=BIG::parity(&z);
            BIG::fshr(&mut z, 1);
            if bt==1 {
                a = BIG::modmul(&a, &s, m);
            }
            if BIG::iszilch(&z) {break}
            s = BIG::modsqr(&s, m);
        }
        return a;
    }

    pub fn nbits(a: &BIG) -> i32 {
        let ret;
        unsafe {
            ret = BIG_nbits(a) as i32;
        }
        return ret;
    }

    pub fn reduce(a: &mut BIG) {
        let p = unsafe { Modulus };
        BIG::rmod(a, &p);
    }

    pub fn norm(a: &mut BIG) -> chunk {
        let ret;
        unsafe {
            ret = BIG_norm(a) as chunk;
        }
        return ret;
    }

    pub fn invmodp(a: &BIG, p: &BIG) -> BIG {
        let mut ret = BIG::default();
        unsafe {
            BIG_invmodp(&mut ret, a, p);
        }
        return ret;
    }

    pub fn fshr(a: &mut BIG, k: i32) -> i32 {
        let ret;
        unsafe {
            ret = BIG_fshr(a, k as c_int) as i32;
        }
        return ret;
    }

    pub fn fshl(a: &mut BIG, k: i32) -> i32 {
        let ret;
        unsafe {
            ret = BIG_fshl(a, k as c_int) as i32;
        }
        return ret;
    }

    pub fn copy(d: &mut BIG, s: &BIG) {
        unsafe {
            BIG_copy(d, s);
        }
    }

    pub fn shr(a: &mut BIG, k: i32) {
        unsafe {
            BIG_shr(a, k as c_int);
        }
    }

    pub fn rcopy(b: &mut BIG, a: &BIG) {
        unsafe {
            BIG_copy(b, a);
        }
    }

    pub fn comp(a: &BIG, b: &BIG) -> i32 {
        let ret;
        unsafe {
            ret = BIG_comp(a, b) as i32;
        }
        return ret;
    }

    pub fn modsqr(a: &BIG, b: &BIG) -> BIG {
        let mut r: BIG = BIG::default();
        unsafe {
            BIG_modsqr(&mut r, a, b);
        }
        return r;
    }

    pub fn add(a: &BIG, b: &BIG) -> BIG {
        let mut r = BIG::default();
        unsafe {
            BIG_add(&mut r, a, b);
        }
        return r;
    }

    pub fn sub(a: &BIG, b: &BIG) -> BIG {
        let mut r = BIG::default();
        unsafe {
            BIG_sub(&mut r, a, b);
        }
        return r;
    }

    pub fn rmod(b: &mut BIG, c: &BIG) {
        unsafe {
            BIG_mod(b, c);
        }
    }

    pub fn sqr(c: &BIG) -> DBIG {
        let mut r: DBIG = DBIG::default();
        unsafe {
            BIG_sqr(&mut r, c);
        }
        return r;
    }

    pub fn sqrm(a: &mut BIG) {
        let r: DBIG = BIG::sqr(a);
        for i in 0..NLEN {
            a.val[i] = r.val[i];
        }
    }

    pub fn one(a: &mut BIG) {
        a.val[0]=1;
        for i in 1 ..NLEN {
            a.val[i]=0;
        }
    }

    pub fn modmul(a: &BIG, b: &BIG, m: &BIG) -> BIG {
        let mut r: BIG = BIG::default();
        unsafe {
            BIG_modmul(&mut r, a, b, m);
        }
        return r;
    }

    pub fn mul(a: &BIG, b: &BIG) -> DBIG {
        let mut r: DBIG = DBIG::default();
        unsafe {
            BIG_mul(&mut r, a, b);
        }
        return r;
    }

    pub fn mulm(a: &BIG, b: &BIG) -> BIG {
        let mut ret: BIG = BIG::default();
        let mut r: DBIG = DBIG::default();
        unsafe {
            BIG_mul(&mut r, a, b);
        }
        for i in 0..NLEN {
            ret.val[i] = r.val[i];
        }
        return ret;
    }

    pub fn imul(a: &BIG, b: i32) -> BIG {
        let mut r: BIG = BIG::default();
        unsafe {
            BIG_imul(&mut r, a, b as c_int);
        }
        return r;
    }

    pub fn modneg(r: &mut BIG, a: &mut BIG, m: &BIG) {
        unsafe {
            BIG_modneg(r, a, m);
        }
    }

    pub fn excess(a: &BIG) -> chunk {
        return (a.val[NLEN-1] & OMASK) >> (MBITS % BASEBITS);
    }

    pub fn neg(a: &mut BIG) {
        let v = a.clone();
        unsafe {
            FP_neg(a, &v);
        }
    }

    pub fn rsub(a: &mut BIG, x:&BIG) {
        for i in 0 ..NLEN {
            a.val[i]=x.val[i]-a.val[i]
        }
    }

    pub fn randomnum(q: &BIG, rng: &mut Random) -> BIG {
        let mut d=BIG::default();
        let mut j=0;
        let mut r:u8=0;
        let t=BIG::new_copy(q);
        for _ in 0..2*BIG::nbits(&t) {
            if j==0 {
                r=rng.getbyte();
            } else {
                r>>=1;
            }
            let b= (r as chunk)&1;
            d.shl(1);
            d.val[0]+=b;
            j+=1;
            j&=7;
        }
        BIG::rmod(&mut d, &q);
        return d;
    }

    pub fn toBytes(b: &mut [u8], a: &BIG) {
        unsafe {
            BIG_toBytes(&mut b[0], a);
        }
    }

    pub fn fromBytes(b: &[u8]) -> BIG {
        let mut ret: BIG = BIG::default();
        unsafe {
            BIG_fromBytes(&mut ret, b.as_ptr());
        }
        return ret;
    }

    pub fn to_hex(&self) -> String {
        let mut ret: String = String::with_capacity(NLEN * 16 + NLEN - 1);

        for i in 0..NLEN {
            if i == NLEN-1 {
                ret.push_str(&format!("{:X}", self.val[i]));
            } else {
                ret.push_str(&format!("{:X} ", self.val[i]));
            }
        }
        return ret;
    }

    pub fn from_hex_iter(iter: &mut SplitWhitespace) -> BIG {
        let mut ret:BIG = BIG::default();
        for i in 0..NLEN {
            let v = iter.next();
            match v {
                Some(x) => {
                    ret.val[i] = u64::from_str_radix(x, 16).unwrap() as chunk;
                },
                None => {
                    // TODO: is it error?
                    break;
                }
            }
        }
        return ret;
    }

    pub fn from_hex(val: String) -> BIG {
        let mut iter = val.split_whitespace();
        return BIG::from_hex_iter(&mut iter);
    }
}

impl fmt::Display for BIG {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BIG: [{}]", big_to_hex(self))
    }
}

impl fmt::Debug for BIG {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BIG: [{}]", big_to_hex(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate rand;
    use self::rand::os::{OsRng};
    use self::rand::Rng;

    #[test]
    fn test_random_mod_order() {
        let mut seed: Vec<u8> = vec![0; 32];
        let mut os_rng = OsRng::new().unwrap();
        os_rng.fill_bytes(&mut seed.as_mut_slice());
        let mut rng = Random::new(&seed);
        BIG::randomnum(unsafe { &CURVE_Order }, &mut rng);
    }

    #[test]
    fn test_bytes() {
        let mut bytes: [u8; MODBYTES] = [0; MODBYTES];
        let mut outbytes: [u8; MODBYTES] = [0; MODBYTES];
        bytes[0] = 0xFF;
        let a: BIG = BIG::fromBytes(&bytes[..]);
        BIG::toBytes(&mut outbytes[..], &a);
        assert_eq!(bytes, outbytes);
    }

    #[test]
    fn test_big_random() {
        let SEED: Vec<u8> = vec![ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                  0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00,
                                  0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                  0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02 ];
        let mut rng = Random::new(&SEED);
        let m = BIG::new_int(0x1000000);
        let r = BIG::randomnum(&m, &mut rng);
        println!("big_random={}", r);
    }

    #[test]
    fn test_big_hex_io() {
        let m = BIG::new_int(0x1000000);
        let s = m.to_hex();
        let r = BIG::from_hex(s.clone());
        let r2 = BIG::from_hex(String::from("A110F34E83D27B B9C4C7A7C37D6E 1A3A0CB86A5CA A4804E43EF1DCB 98035C9BEF3E44D3"));
        println!("big_hex_io=s:{},m:{},r:{},r2:{}", s, m, r, r2);
        assert_eq!(m, r);
    }
}
