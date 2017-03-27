#[macro_use]
use wrappers::*;

pub struct BigNum {
    size: usize,
    storage: Vec<BIG>
}

impl BigNum {

    /*
     * New
     */
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

    /*
     * from_bytes
     */
    pub fn from_bytes(val: &[u8], size: usize) -> BigNum {
        let blen = (size+31)/32;
        let mut ret = BigNum::new(blen);
        let mut o = octet::new(val, size);
        unsafe {
            FF_fromOctet(&mut ret.storage.as_mut_slice()[0], &mut o, blen as i32);
        }
        return ret;
    }

    /*
     * to_string
     */
    pub fn to_string(&mut self) -> String {
        return ff_to_string(self.storage.as_mut_slice(), self.size as i32);
    }

    /*
     * add
     * self += val
     */
    pub fn add(&mut self, val: BigNum) {
        // TODO: implement
    }

    /*
     * sub
     * self -= val
     */
    pub fn sub(&mut self, val: BigNum) {
        // TODO: implement
    }

    /*
     * mul
     * self *= val
     */
    pub fn mul(&mut self, val: BigNum) {
        // TODO: implement
    }

    /*
     * TODO: fn list
     *  pub fn FF_random(x: &mut BIG, R: &mut csprng, n: c_int) -> c_void;
     *  pub fn FF_mod(x: &mut BIG, m: &mut BIG, n: c_int) -> c_void;
     *  pub fn FF_sqr(x: &mut BIG, y: &mut BIG, n: c_int) -> c_void;
     *  pub fn FF_pow(r: &mut BIG, x: &mut BIG, e: &mut BIG, m: &mut BIG, n: c_int) -> c_void;
     *  pub fn FF_prime(x: &mut BIG, R: &mut csprng, n: c_int) -> c_int;
     */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bignum_io() {
        let val: [u8; 32] = [ 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
                              0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00,
                              0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
                              0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00 ];
        let mut x = BigNum::from_bytes(&val[0..], 32);
        let str = x.to_string();
        println!("bignum_io: str = {}", str);
        assert_eq!(str, "112233445566778899AABBCCDDEEFF00112233445566778899AABBCCDDEEFF00");
    }
}
