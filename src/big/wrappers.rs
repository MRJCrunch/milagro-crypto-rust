#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;
use self::libc::{c_int, c_void, int64_t, uint8_t};

// TODO: autogenerate this part!
pub const NLEN:usize = 5;      // use amcl_build command to get this
pub const DNLEN:usize = 2*NLEN;
pub type chunk = int64_t;  // use amcl_build command to get this
pub const MODBYTES:usize = 32; // use amcl_build command to get this
// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

pub const MBITS:usize = 254;
pub const BASEBITS:usize = 56;
pub const TBITS:usize = MBITS % BASEBITS;
pub const BMASK: chunk =  ( (1 as chunk) << BASEBITS) - 1;
pub const OMASK: chunk = -( (1 as chunk) << TBITS);
pub const FEXCESS:chunk = ( (1 as chunk) << (BASEBITS*(NLEN)-MBITS));

pub const BIG_HEX_STRING_LEN:usize = NLEN * 16 + NLEN - 1;

#[repr(C)]
pub struct BIG {
    pub val: [ chunk; NLEN ]
}

#[repr(C)]
pub struct DBIG {
    pub val: [ chunk; DNLEN ]
}

impl Default for BIG {
    fn default () -> BIG {
        BIG {
            val: [ 0; NLEN ]
        }
    }
}

impl Default for DBIG {
    fn default () -> DBIG {
        DBIG {
            val: [ 0; DNLEN ]
        }
    }
}

extern {
    // TODO: maybe move to separate module "rom"

    pub static MConst: chunk;
    pub static Modulus: BIG;
    pub static CURVE_Order: BIG;
    pub static CURVE_Cof: BIG;
    pub static CURVE_B: BIG;
    pub static CURVE_Bnx: BIG;
    pub static CURVE_Cru: BIG;
    pub static CURVE_Fra: BIG;
    pub static CURVE_Frb: BIG;
    pub static CURVE_Pxa: BIG;
    pub static CURVE_Pxb: BIG;
    pub static CURVE_Pya: BIG;
    pub static CURVE_Pyb: BIG;
    pub static CURVE_Gx: BIG;
    pub static CURVE_Gy: BIG;
    pub static CURVE_W: [BIG; 2];
    pub static CURVE_SB: [[BIG; 2]; 2];
    pub static CURVE_WB: [BIG; 4];
    pub static CURVE_BB: [[BIG; 4]; 4];
    pub static CURVE_A: c_int;
    // ^^^^^^^

    // TODO: FIXME: create fp module?
    pub fn FP_neg(r: *mut BIG, a: *const BIG) -> c_void;

    pub fn BIG_invmodp(r: *mut BIG, a: *const BIG,p: *const BIG) -> c_void;
    pub fn BIG_sqr(c: *mut DBIG, a: *const BIG) -> c_void;
    pub fn BIG_modsqr(r: *mut BIG, a: *const BIG, m: *const BIG) -> c_void;
    pub fn BIG_fshr(a: *mut BIG, k: c_int) -> c_int;
    pub fn BIG_fshl(a: *mut BIG, k: c_int) -> c_int;
    pub fn BIG_nbits(a: *const BIG) -> c_int;
    pub fn BIG_copy(d: *mut BIG, s: *const BIG) -> c_void;
    pub fn BIG_shr(a: *mut BIG, k: c_int) -> c_void;
    pub fn BIG_rcopy(b: *mut BIG, a: *const BIG) -> c_void;
    pub fn BIG_comp(a: *const BIG, b: *const BIG) -> c_int;
    pub fn BIG_add(c: *mut BIG, a: *const BIG, b: *const BIG) -> c_void;
    pub fn BIG_sub(c: *mut BIG, a: *const BIG, b: *const BIG) -> c_void;
    pub fn BIG_mod(b: *mut BIG, c: *const BIG) -> c_void;
    pub fn BIG_modmul(r: *mut BIG, a: *const BIG, b: *const BIG, m: *const BIG) -> c_void;
    pub fn BIG_modneg(r: *mut BIG, a: *mut BIG, c: *const BIG) -> c_void;
    pub fn BIG_mul(c: *mut DBIG, a: *const BIG, b: *const BIG) -> c_void;
    pub fn BIG_imul(r: *mut BIG, a: *const BIG, c: c_int) -> c_void;
    pub fn BIG_norm(a: *mut BIG) -> chunk;
    pub fn BIG_toBytes(b: *mut uint8_t, a: *const BIG) -> c_void;
    pub fn BIG_fromBytes(a: *mut BIG, b: *const uint8_t) -> c_void;
}

pub fn big_to_hex(a: &BIG) -> String {
    let mut ret: String = String::with_capacity(MODBYTES*2);
    let mut b: BIG = BIG::default();
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
            BIG_copy(&mut b, a);
            BIG_shr(&mut b, (i*4) as i32);
        }
        ret.push_str(&format!("{:X}", b.val[0]&15));
    }

    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex() {
        let a: BIG = BIG::default();
        assert_eq!(big_to_hex(&a), "0000000000000000000000000000000000000000000000000000000000000000");
    }
}
