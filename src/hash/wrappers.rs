#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;
use self::libc::{c_int, c_void, uint8_t, uint32_t};

#[repr(C)]
pub struct hash256 {
    length: [uint32_t;2],
    h: [uint32_t;8],
    w: [uint32_t; 80],
    hlen: c_int
}

impl hash256 {
    pub fn new_zeroed() -> hash256 {
        hash256 {
            length: [0; 2],
            h: [0; 8],
            w: [0; 80],
            hlen: 0
        }
    }
}
extern {
    pub fn HASH256_init(sh: *mut hash256) -> c_void;
    pub fn HASH256_process(sh: *mut hash256, byte: c_int) -> c_void;
    pub fn HASH256_hash(sh: *mut hash256, digest: *mut uint8_t) -> c_void;
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_sha256() {
        let golden: [uint8_t; 32] = [ 0x02, 0x1f, 0xb5, 0x96, 0xdb, 0x81, 0xe6, 0xd0,
                                      0x2b, 0xf3, 0xd2, 0x58, 0x6e, 0xe3, 0x98, 0x1f,
                                      0xe5, 0x19, 0xf2, 0x75, 0xc0, 0xac, 0x9c, 0xa7,
                                      0x6b, 0xbc, 0xf2, 0xeb, 0xb4, 0x09, 0x7d, 0x96 ];
        let mut digest: [uint8_t; 32] = [0; 32];
        let mut sh: hash256 = unsafe { mem::zeroed() };
        unsafe {
            HASH256_init(&mut sh);
            HASH256_process(&mut sh, 123);
            HASH256_hash(&mut sh, &mut digest[0]);
        }
        println!("sha256(wrappers): {:?}", digest);
        assert_eq!(golden, digest);
    }
}
