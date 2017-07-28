#![allow(non_snake_case)]

pub mod wrappers;

extern crate libc;

use fp12::wrappers::*;
use ecp::wrappers::*;
use ecp2::wrappers::*;
use pair::wrappers::*;
use big::wrappers::*;

pub struct PAIR {
}

impl PAIR {
    pub fn ate(r: &mut FP12, P: &mut ECP2, Q: &mut ECP) {
        unsafe {
            PAIR_ate(r, P, Q);
        }
    }

    pub fn fexp(r: &FP12) {
        unsafe {
            PAIR_fexp(r);
        }
    }

    pub fn g1_mul(P: &mut ECP, e: &BIG) {
        unsafe {
            PAIR_G1mul(P, e);
        }
    }

    pub fn g2_mul(P: &mut ECP2, e: &BIG) {
        unsafe {
            PAIR_G2mul(P, e);
        }
    }

    pub fn gt_pow(f: &mut FP12, e: &BIG) {
        unsafe {
            PAIR_GTpow(f, e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use big::wrappers::*;
    use fp2::wrappers::*;

    #[test]
    fn test_pair() {
        let mut a: BIG = BIG::default();
        let mut b: BIG = BIG::default();
        let mut xa: BIG = BIG::default();
        let mut xb: BIG = BIG::default();
        let mut ya: BIG = BIG::default();
        let mut yb: BIG = BIG::default();

        let mut P: ECP2 = Default::default();
//        let G: ECP2 = Default::default();

        let mut Q: ECP = Default::default();
//        let R: ECP = Default::default();

        let mut g: FP12 = Default::default();

        let mut x: FP2 = Default::default();
        let mut y: FP2 = Default::default();
        let mut X: FP2 = Default::default();

        BIG::rcopy(&mut a, unsafe { &CURVE_Fra });
        BIG::rcopy(&mut b, unsafe { &CURVE_Frb });
        FP2::from_BIGs(&mut X, &a, &b);

        BIG::rcopy(&mut xa, unsafe { &CURVE_Gx} );
        BIG::rcopy(&mut ya, unsafe { &CURVE_Gy });

        ECP::set(&mut Q, &xa, &ya);

        if Q.inf != 0 {
            println!("Failed to set - point not on curve");
        } else {
            println!("G1 set success");
        }

        println!("Q={}", Q);

        BIG::rcopy(&mut xa, unsafe { &CURVE_Pxa });
        BIG::rcopy(&mut xb, unsafe { &CURVE_Pxb });
        BIG::rcopy(&mut ya, unsafe { &CURVE_Pya });
        BIG::rcopy(&mut yb, unsafe { &CURVE_Pyb });

        FP2::from_BIGs(&mut x, &xa, &xb);
        FP2::from_BIGs(&mut y, &ya, &yb);

        ECP2::set(&mut P, &x, &y);

        if P.inf != 0 {
            println!("Failed to set - point not on curve");
        } else {
            println!("G2 set success");
        }

        println!("P={}", P);

        for _ in 0..1000 {
            PAIR::ate(&mut g, &mut P, &mut Q);
            PAIR::fexp(&mut g);
        }
        println!("g={}", g);
        // no assert, segfault means test failed
    }
}
