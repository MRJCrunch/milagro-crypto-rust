#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;
use self::libc::{c_void};

use fp12::wrappers::*;
use ecp::wrappers::*;
use ecp2::wrappers::*;
use big::wrappers::*;

extern {
    pub fn PAIR_ate(r: *mut FP12, P: *mut ECP2, Q: *mut ECP) -> c_void;
    pub fn PAIR_fexp(r: *const FP12) -> c_void;
    pub fn PAIR_G1mul(P: *mut ECP, e: *const BIG) -> c_void;
    pub fn PAIR_G2mul(P: *mut ECP2, e: *const BIG) -> c_void;
    pub fn PAIR_GTpow(f: *mut FP12, e: *const BIG) -> c_void;
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

        unsafe {
            BIG_rcopy(&mut a, &CURVE_Fra);
            BIG_rcopy(&mut b, &CURVE_Frb);
            FP2_from_BIGs(&mut X, &a, &b);

            BIG_rcopy(&mut xa, &CURVE_Gx);
            BIG_rcopy(&mut ya, &CURVE_Gy);

            ECP_set(&mut Q, &xa, &ya);
        }

        if Q.inf != 0 {
            println!("Failed to set - point not on curve");
        } else {
            println!("G1 set success");
        }

        print!("Q= ");
        unsafe {
            ECP_output(&Q);
        }
        println!("");

        unsafe {
            BIG_rcopy(&mut xa, &CURVE_Pxa);
            BIG_rcopy(&mut xb, &CURVE_Pxb);
            BIG_rcopy(&mut ya, &CURVE_Pya);
            BIG_rcopy(&mut yb, &CURVE_Pyb);

            FP2_from_BIGs(&mut x, &xa, &xb);
            FP2_from_BIGs(&mut y, &ya, &yb);

            ECP2_set(&mut P, &x, &y);
        }
        if P.inf != 0 {
            println!("Failed to set - point not on curve");
        } else {
            println!("G2 set success");
        }

        print!("P= ");
        unsafe {
            ECP2_output(&P);
        }
        println!("");

        for _ in 0..1000 {
            unsafe {
                PAIR_ate(&mut g, &mut P, &mut Q);
                PAIR_fexp(&mut g);
            }
        }
        print!("g= ");
        unsafe {
            FP12_output(&g);
        }
        println!("");
        // no assert, segfault means test failed
    }
}
