#[macro_use]
use wrappers::*;

pub struct BigNum {
    size: usize,
    storage: Vec<BIG>
}

impl BigNum {
    pub fn new(n: usize) -> BigNum {
        let mut arr = Vec::<BIG>::with_capacity(n);
        for i in 0..n {
            arr.push(BIG_ZERO!());
        }
        BigNum {
            size: n,
            storage: arr
        }
    }

    pub fn from_bytes(val: &[u8], size: usize) -> BigNum {
        let blen = (size+31)/32;
        let mut ret = BigNum::new(blen);
        let mut o = octet::new(val, size);
        unsafe {
            FF_fromOctet(&mut ret.storage.as_mut_slice()[0], &mut o, blen as i32);
        }
        return ret;
    }

    pub fn to_string(&mut self) -> String {
        return ff_to_string(self.storage.as_mut_slice(), self.size as i32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bignum_io() {
        let val: [u8; 32] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03 ];
        let mut x = BigNum::from_bytes(&val[0..], 32);
        println!("bignum_io: str = {}", x.to_string());
    }
}
