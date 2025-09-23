use std::{fmt, option::Option};

use crate::format::format_f64;


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Complex {
    // real part
    pub re: f64,
    // imaginary part
    pub im: f64
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PolarComplex {
    // modulus
    r: f64,
    // angle (in radians)
    theta: f64
}

impl PolarComplex {
    pub const fn new(r: f64, theta: f64) -> Self {
        PolarComplex { r, theta }
    }

    pub fn to_complex(&self) -> Complex {
        let re = self.r * f64::cos(self.theta);
        let im = self.r * f64::sin(self.theta);

        Complex::new(re, im)
    }

    pub fn to_std_string(&self, magnitude: usize, precision: usize) -> String {
        format!("@{{{}, {}}}", format_f64(self.r, magnitude, precision), format_f64(self.theta, magnitude, precision))
    }
}

impl Complex {
    pub const fn new(re: f64, im: f64) -> Self {
        Complex { re, im } 
    }

    pub const ZERO: Self = Self::new(0.0, 0.0);
    pub const ONE: Self = Self::new(1.0, 0.0);
    pub const I: Self = Self::new(0.0, 1.0);
    pub const MINUS_I: Self = Self::new(0.0, -1.0);

    pub fn to_std_string(&self, magnitude: usize, precision: usize) -> String {
        if self.im == 0.0 {
            format!("{{{}}}", format_f64(self.re, magnitude, precision))
        } else if self.re == 0.0 {
            format!("{{{}}}", im_to_string(self.im, magnitude, precision))
        } else if self.im < 0.0 {
            format!("{{{} - {}}}", format_f64(self.re, magnitude, precision), im_to_string(-self.im, magnitude, precision))
        } else {
            format!("{{{} + {}}}", format_f64(self.re, magnitude, precision), im_to_string(self.im, magnitude, precision))
        }
    }

    pub fn to_polar(&self) -> PolarComplex {
        let r = f64::sqrt(self.re * self.re + self.im * self.im);
        let theta = f64::atan2(self.im, self.re);
        PolarComplex::new(r, theta)
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

/*fn checked_mul(a: f64, b: f64) -> Option<f64> {
    let c = a * b;
    c.is_finite().then_some(c)
}

fn checked_div(a: f64, b: f64) -> Option<f64> {
    let c = a / b;
    c.is_finite().then_some(c)
}*/

fn im_to_string(im: f64, magnitude: usize, precision: usize) -> String {
    match im {
        1.0 => "i".to_owned(),
        -1.0 => "-i".to_owned(),
        _ => format!("{}i", format_f64(im, magnitude, precision)).to_owned(),
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let magnitude = fmt.width().unwrap_or(12);
        let precision = fmt.precision().unwrap_or(6);

        write!(fmt, "{}", self.to_std_string(magnitude, precision))
    }
}

impl fmt::Display for PolarComplex {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let magnitude = fmt.width().unwrap_or(12);
        let precision = fmt.precision().unwrap_or(6);

        write!(fmt, "{}", self.to_std_string(magnitude, precision))
    }
}

#[cfg(test)]
mod tests {
    use core::f64;

    use rstest::rstest;
    use super::*;

    // assert that two numbers are close
    macro_rules! assert_close {
        ($x:expr, $y:expr, $df:expr) => {
            if ($x - $y).abs() > $df {
                panic!("difference between {} and {} too large", $x, $y);
            }
        };
    }

    #[rstest(
        c, expected,
        case::zero(Complex::new(0.0, 0.0), PolarComplex::new(0.0, 0.0)),
        case::one(Complex::new(1.0, 0.0), PolarComplex::new(1.0, 0.0)),
        case::i(Complex::new(0.0, 1.0), PolarComplex::new(1.0, 1.57)),
        case::minus_i(Complex::new(0.0, -1.0), PolarComplex::new(1.0, -1.571)),
        case::minus_one(Complex::new(-1.0, 0.0), PolarComplex::new(1.0, 3.142)),
        case::one_plus_i(Complex::new(1.0, 1.0), PolarComplex::new(1.414, 0.785)),
        case::minus_one_minus_i(Complex::new(-1.0, -1.0), PolarComplex::new(1.414, -2.357)),
        case::a_bigger_number(Complex::new(-123.0, 26.0), PolarComplex::new(125.718, 2.933))
    )]
    fn to_polar_form(c: Complex, expected: PolarComplex) {
        let result = c.to_polar();
        assert_close!(expected.r, result.r, 0.001);
        assert_close!(expected.theta, result.theta, 0.001);
    }

    #[rstest(
        c, expected,
        case::re_and_im_one(Complex::new(1.0, 1.0), "{1 + i}"),
        case::re_and_im(Complex::new(1.0, 1.5), "{1 + 1.5i}"),
        case::re_and_negative_im_one(Complex::new(1.0, -1.0), "{1 - i}"),
        case::re_and_negative_im(Complex::new(1.0, -7.999), "{1 - 7.999i}"),
        case::re_only(Complex::new(-1.25, 0.0), "{-1.25}"),
        case::im_only(Complex::new(0.0, 1.0), "{i}")
    )]
    fn string(c: Complex, expected: &str) {
        let result = c.to_string();
        assert_eq!(expected, result);
    }

    // TODO: testing formatting once scheme for this implemented
    #[rstest(
        c, magnitude, precision, expected,
        case::one(Complex::new(0.00005, 50000.0), 6, 6, "{0.00005 + 50000i}")
    )]
    fn string_formatted(c: Complex, magnitude: usize, precision: usize, expected: &str) {
        let result = format!("{:magnitude$.precision$}", c);
        assert_eq!(expected, result);
    }

    // TODO: Extend this once formatting sorted out
    #[rstest(
        c, expected,
        case::re_and_im_one(Complex::new(1.0, 0.0), "@{1, 0}")
    )]
    fn polar_string(c: Complex, expected: &str) {
        let result = c.to_polar().to_string();
        assert_eq!(expected, result);
    }

    #[rstest(
        a, b, expected,
        case::add(Complex::new(1.0, 1.0), Complex::new(2.0, -1.0), Some(Complex::new(3.0, 0.0))),
        case::add_infinity_none(Complex::new(1.0, 1.0), Complex::new(2.0, f64::INFINITY), None),
        case::add_nan_none(Complex::new(f64::NAN, 1.0), Complex::new(2.0, -1.0), None)
    )]
    fn add(a: Complex, b: Complex, expected: Option<Complex>) {
        let result = a.add(b);
        assert_eq!(expected, result);
    }

    #[rstest(
        a, b, expected,
        case::sub(Complex::new(1.0, 1.0), Complex::new(2.0, -1.0), Some(Complex::new(-1.0, 2.0))),
        case::sub_infinity_none(Complex::new(1.0, 1.0), Complex::new(2.0, f64::INFINITY), None),
        case::sub_nan_none(Complex::new(f64::NAN, 1.0), Complex::new(2.0, -1.0), None)
    )]
    fn sub(a: Complex, b: Complex, expected: Option<Complex>) {
        let result = a.sub(b);
        assert_eq!(expected, result);
    }

    #[rstest(
        a, b, expected,
        case::mul(Complex::new(1.0, 1.0), Complex::new(2.0, -1.0), Some(Complex::new(3.0, 1.0))),
        case::mul_overflow(Complex::new(1.1, 1.0), Complex::new(f64::MAX, -1.0), None),
        case::sub_infinity_none(Complex::new(1.0, 1.0), Complex::new(2.0, f64::INFINITY), None),
        case::sub_nan_none(Complex::new(f64::NAN, 1.0), Complex::new(2.0, -1.0), None)
    )]
    fn mul(a: Complex, b: Complex, expected: Option<Complex>) {
        let result = a.mul(b);
        assert_eq!(expected, result);
    }

    #[rstest(
        a, b, expected,
        case::div(Complex::new(3.0, 1.0), Complex::new(2.0, -1.0), Some(Complex::new(1.0, 1.0))),
        case::div_overflow(Complex::new(1.1, 1.0), Complex::new(f64::MAX, -1.0), None),
        case::div_by_zero(Complex::new(1.1, 1.0), Complex::ZERO, None),
        case::div_infinity_none(Complex::new(1.0, 1.0), Complex::new(2.0, f64::INFINITY), None),
        case::div_nan_none(Complex::new(f64::NAN, 1.0), Complex::new(2.0, -1.0), None)
    )]
    fn div(a: Complex, b: Complex, expected: Option<Complex>) {
        let result = a.div(b);
        assert_eq!(expected, result);
    }
}
