#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;
use self::libc::{c_int, c_void, c_char, uint32_t};

use std::mem;

use big::wrappers::*;
use randapi::wrappers::*;
use fp2::wrappers::*;
use fp4::wrappers::*;
use fp12::wrappers::*;
use ecp::wrappers::*;
use ecp2::wrappers::*;

extern {
    pub fn PAIR_ate(r: *mut FP12, P: *mut ECP2, Q: *mut ECP) -> c_void;
    pub fn PAIR_fexp(r: *const FP12) -> c_void;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair() {
        let mut rng: csprng = Default::default();
        let mut a: BIG = BIG_ZERO!();
        let mut b: BIG = BIG_ZERO!();
        let mut xa: BIG = BIG_ZERO!();
        let mut xb: BIG = BIG_ZERO!();
        let mut ya: BIG = BIG_ZERO!();
        let mut yb: BIG = BIG_ZERO!();

        let mut P: ECP2 = Default::default();
        let mut G: ECP2 = Default::default();

        let mut Q: ECP = Default::default();
        let mut R: ECP = Default::default();

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

        assert_eq!(1, 1);
    }
}
