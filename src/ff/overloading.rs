use ff::FF;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};


impl<'a, 'b> Add<&'b FF> for &'a FF {
    type Output = FF;

    fn add(self, other: &'b FF) -> FF {
        FF::add(self, other)
    }
}

impl<'b> AddAssign<&'b FF> for FF {
    fn add_assign(&mut self, other: &'b FF) {
        *self = FF::add(self, other);
    }
}

impl<'a, 'b> Mul<&'b FF> for &'a FF {
    type Output = FF;

    fn mul(self, other: &'b FF) -> FF {
        FF::mul(self, other)
    }
}

impl<'b> MulAssign<&'b FF> for FF {
    fn mul_assign(&mut self, other: &'b FF) {
        *self = FF::mul(self, other);
    }
}

impl<'a, 'b> Sub<&'b FF> for &'a FF {
    type Output = FF;

    fn sub(self, other: &'b FF) -> FF {
        FF::sub(self, other)
    }
}

impl<'b> SubAssign<&'b FF> for FF {
    fn sub_assign(&mut self, other: &'b FF) {
        *self = FF::sub(self, other);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plus_test() {
        let x = FF::from_hex("1", 0);
        let y = FF::from_hex("2", 0);
        let z = &x + &y;
        let str = z.to_hex();
        println!("ff_mul: x = {}", &x);
        println!("ff_mul: x = {}", &y);
        println!("ff_mul: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000000003");
    }

    #[test]
    fn plus_assign_test() {
        let mut x = FF::from_hex("1", 0);
        let y = FF::from_hex("2", 0);
        x += &y;
        let str = x.to_hex();
        println!("ff_mul: x = {}", &x);
        println!("ff_mul: x = {}", &y);
        println!("ff_mul: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000000003");
    }

    #[test]
    fn sub_test() {
        let x = FF::from_hex("2", 0);
        let y = FF::from_hex("1", 0);
        let z = &x - &y;
        let str = z.to_hex();
        println!("ff_mul: x = {}", &x);
        println!("ff_mul: x = {}", &y);
        println!("ff_mul: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000000001");
    }

    #[test]
    fn sub_assign_test() {
        let mut x = FF::from_hex("2", 0);
        let y = FF::from_hex("1", 0);
        x -= &y;
        let str = x.to_hex();
        println!("ff_mul: x = {}", &x);
        println!("ff_mul: x = {}", &y);
        println!("ff_mul: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000000001");
    }

    #[test]
    fn mul_test() {
        let x = FF::from_hex("2", 0);
        let y = FF::from_hex("1", 0);
        let z = &x * &y;
        let str = z.to_hex();
        println!("ff_mul: x = {}", &x);
        println!("ff_mul: x = {}", &y);
        println!("ff_mul: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000000002");
    }

    #[test]
    fn mul_assign_test() {
        let mut x = FF::from_hex("2", 0);
        let y = FF::from_hex("1", 0);
        x *= &y;
        let str = x.to_hex();
        println!("ff_mul: x = {}", &x);
        println!("ff_mul: x = {}", &y);
        println!("ff_mul: str = {}", str);
        assert_eq!(str, "0000000000000000000000000000000000000000000000000000000000000000 \
                         0000000000000000000000000000000000000000000000000000000000000002");
    }
}
