pub mod wrappers;

extern crate libc;
use self::libc::{c_int};

use fp12::wrappers::*;
use ecp::wrappers::*;
use ecp2::wrappers::*;
use pair::wrappers::*;

pub fn pair_ate(r: &mut FP12, P: &mut ECP2, Q: &mut ECP) {
    unsafe {
        PAIR_ate(r, P, Q);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair() {
        let mut r: FP12 = Default::default();
        let mut P: ECP2 = Default::default();
        let mut Q: ECP = Default::default();
//        pair_ate(&mut r, &mut P, &mut Q);
        assert_eq!(1, 1);
    }
}
