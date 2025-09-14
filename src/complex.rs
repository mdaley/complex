use std::{fmt, option::Option};


#[derive(Debug, Copy, Clone)]
pub struct Complex {
    pub re: f64,
    pub im: f64
}

impl Complex {
    pub const fn new(re: f64, im: f64) -> Self {
        Complex { re, im } 
    }

    pub const ZERO: Self = Self::new(0.0, 0.0);
    pub const ONE: Self = Self::new(1.0, 0.0);
    pub const I: Self = Self::new(0.0, 1.0);
    pub const MINUS_I: Self = Self::new(0.0, -1.0);

    pub fn to_std_string(&self) -> String {
        if self.im == 0.0 {
            format!("{{{}}}", self.re)
        } else if self.re == 0.0 {
            format!("{{{}}}", im_to_string(self.im))
        } else if self.im < 0.0 {
            format!("{{{} - {}}}", self.re, im_to_string(-self.im))
        } else {
            format!("{{{} + {}}}", self.re, im_to_string(self.im))
        }
    }

    pub fn to_polar_string(&self) -> String {
        let r = ((self.im * self.im) + (self.re * self.re)).sqrt();
        let theta = self.im.atan2(self.re);

        format!("@{{{}, {}}}", r, theta)
    }

    pub fn add(&self, other: Complex) -> Option<Complex> {
        Some(Complex::new(
            checked_add(self.re, other.re)?, 
            checked_add(self.im, other.im)?))
    }

    pub fn sub(&self, other: Complex) -> Option<Complex> {
        Some(Complex::new(
            checked_sub(self.re, other.re)?,
            checked_sub(self.im, other.im)?))
    }

    pub fn mul(&self, other: Complex) -> Option<Complex> {
        let re = self.re * other.re - self.im * other.im;
        let im = self.re * other.im + other.re * self.im;
        
        finite_complex_or_none(re, im)
    }

    // Uses:
    // 
    // a₁ + b₁i       a₁a₂ + b₁b₂     a₂b₁ - a₁b₂
    // -------- =    ------------- + ------------ i
    // a₂ + b₂i        a₂² + b₂²       a₂² + b₂²
    // 
    pub fn div(&self, other: Complex) -> Option<Complex> {
        let denom = other.re * other.re + other.im * other.im;
        let re_num = self.re * other.re + self.im * other.im;
        let im_num = other.re * self.im - self.re * other.im;

        let re = re_num / denom;
        let im = im_num / denom;

        finite_complex_or_none(re, im)
    }

}

fn finite_complex_or_none(re: f64, im: f64) -> Option<Complex> {
    if re.is_finite() && im.is_finite() {
        Some(Complex::new(re, im))
    } else {
        None
    }
}

fn checked_add(a: f64, b: f64) -> Option<f64> {
    let c = a + b;
    c.is_finite().then_some(c)
}

fn checked_sub(a: f64, b: f64) -> Option<f64> {
    let c = a - b;
    c.is_finite().then_some(c)
}

fn checked_mul(a: f64, b: f64) -> Option<f64> {
    let c = a * b;
    c.is_finite().then_some(c)
}

fn checked_div(a: f64, b: f64) -> Option<f64> {
    let c = a / b;
    c.is_finite().then_some(c)
}

fn im_to_string(im: f64) -> String {
    match im {
        1.0 => "i".to_owned(),
        -1.0 => "-i".to_owned(),
        _ => format!("{}i", im).to_owned(),
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_std_string())
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    /*#[rstest(
        a, b, expected,
        case::add("{1 + i}", "{2 - i}", "{3}")
    )]
    fn add_1(a: &str, b: &str, expected: &str) {
        let result = 
        assert_eq!(expected, )

    }*/

    #[test]
    fn add() {
        let a = Complex::new(1.0, 1.0);
        let b = Complex::new(1.0, -2.0);

        let c =  a.add(b);

        assert_eq!("{2 - i}", c.unwrap().to_string());

        // and the original numbers can still be used (because the Copy trait is derived)
        assert_eq!("{1 + i}", a.to_string());
        assert_eq!("{1 - 2i}", b.to_string());
    }

    #[test]
    fn add_out_of_bounds() {
        let a = Complex::new(1.0, f64::MAX);
        let b = Complex::new(1.0, f64::MAX);

        let c = a.add(b);

        assert!(c.is_none());
    }
}
