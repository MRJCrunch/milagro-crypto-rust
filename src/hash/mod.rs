pub mod wrappers;

extern crate libc;
use self::libc::{c_int};

use hash::wrappers::*;

impl hash256 {
    pub fn new() -> hash256 {
        let mut ret = hash256::new_zeroed();
        unsafe {
            HASH256_init(&mut ret);
        }
        return ret;
    }

    pub fn process(&mut self, byte: u8) {
        unsafe {
            HASH256_process(self, byte as c_int);
        }
    }

    pub fn hash(&mut self) -> Vec<i8> {
        let mut ret: Vec<i8> = vec![0; 32];
        unsafe {
            HASH256_hash(self, &mut ret.as_mut_slice()[0]);
        }
        return ret;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        let golden: Vec<i8> = vec![ 0x02, 0x1f, 0xb5, 0x96, 0xdb, 0x81, 0xe6, 0xd0,
                                    0x2b, 0xf3, 0xd2, 0x58, 0x6e, 0xe3, 0x98, 0x1f,
                                    0xe5, 0x19, 0xf2, 0x75, 0xc0, 0xac, 0x9c, 0xa7,
                                    0x6b, 0xbc, 0xf2, 0xeb, 0xb4, 0x09, 0x7d, 0x96 ];

        let mut sh: hash256 = hash256::new();
        sh.process(123);
        let digest = sh.hash();

        println!("sha256(mod): {:?}", digest);
        assert_eq!(golden, digest);
    }
}
