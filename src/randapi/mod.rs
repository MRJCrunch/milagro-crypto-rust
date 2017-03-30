#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub mod wrappers;

use randapi::wrappers::*;

pub struct Random {
    pub rng: csprng
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

#[cfg(test)]
mod tests {
    use super::*;

    const SEED: [u8; 32] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                             0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00,
                             0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                             0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02 ];

    #[test]
    fn test_random() {
        let r = Random::new(SEED);
        // no assert, segfault means test failed
    }
}
