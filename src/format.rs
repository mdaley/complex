
// note: rust format! does half-up rounding

pub fn format_f64(f: f64, max_digit_magnitude: usize, max_precision: usize) -> String {

    let magn = digit_magnitude(f);

    let abs_f = f.abs();

    // print in exp form, if the magnitude is greater than the max magnitude
    // i.e. f = 10000. max_magn 4 -> exp form 1e5
    // and f = 0.0000001, max magn 6 -> exp form 1e-7
    // if the f is between 1.0 and -1.0 then also take into account the
    // precision so that
    // f = 0.00005, max magn 3, precision 3 doesn't print as 0 but as 5e-5 and
    // f = 0.00005, max magn 5, precision 5 prints as 0.00005
    if magn <= max_digit_magnitude && !(abs_f < 1.0 && max_precision < magn){
        format!("{:.max_precision$}", f).trim_end_matches('0').trim_end_matches('.').trim().to_string()
    } else {
        let s = format!("{:.max_precision$e}", f).to_string();
        let parts: Vec<&str> = s.split('e').collect();
        let num = parts[0].trim_end_matches('0').trim_end_matches('.');
        let exp = parts[1];
        format!("{}e{}", num, exp)
    }
}

/// Returns the number of digits before the decimal point, with the caveat that the
/// result for zero is 1, or, if the number is less than zero, the number of zeros
/// before the first non-zero digit plus 1.
/// 
/// If the number is negative an additional 1 is added to represent the minus symbol.
///  
/// This is used to work out whether a f64 should be printed in exponential form or not.
/// 
/// Some examples:
/// - 0.0 -> 1
/// - 9.99 -> 1
/// - 100.1234 -> 3
/// - -1000.3 -> 5
/// - 123456789.345678 -> 9
/// - -0.00000000001 -> 12
/// 
fn digit_magnitude(f: f64) -> usize {
    let neg: u16 = if f < 0.0 { 1 } else { 0 };

    if f == 0.0 {
        1
    } else {
        let m = f.abs().log10().floor() as i16;
        if m >= 0 {
            let mm = m.cast_unsigned() + 1 + neg;
            mm as usize

        } else {
            let mm = m.unsigned_abs() + neg;
            mm as usize
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(
        f, len, precision, expected,
        case::zero(0.0, 6, 3, "0"),
        case::simple(123.45, 6, 3, "123.45"),
        case::round_up(123.456789, 6, 3, "123.457"),
        case::round_down(123.456749, 6, 4, "123.4567"),
        case::simple_2(123.4, 3, 8, "123.4"),
        case::big(123456789.123456, 6, 3, "1.235e8"),
        case::big_neg(-123456789.123456, 6, 3, "-1.235e8"),
        case::big_neg_big_magn(-123456789.123456, 10, 3, "-123456789.123"),
        case::tiny(0.000000123456, 6, 3, "1.235e-7"),
        case::tiny_few_digits(1e-25, 6, 3, "1e-25"),
        case::large_few_digits(93000000000.0, 8, 6, "9.3e10"),
        case::below_abs_one_go_exp_to_avoid_trunc_to_zero(0.00005, 3, 3, "5e-5"),
        case::below_abs_one_go_exp_to_avoid_trunc_to_zero(-0.000059, 5, 3, "-5.9e-5"),
        case::below_abs_one_no_trunc_need(0.00005, 5, 5, "0.00005")
    )]
    fn string(f: f64, len: usize, precision: usize, expected: &str) {
        let result = format_f64(f, len, precision);
        assert_eq!(expected, result);
    }

    #[rstest(
        f, expected,
        case::zero(0.0, 1),
        case::one(1.0, 1),
        case::two_digits(11.7, 2),
        case::two_digits_max(100.0, 3),
        case::many_digits(123456789.345678, 9),
        case::max(f64::MAX, 309),
        case::point_one(0.1, 1),
        case::three_zeros_after_dp(0.0009, 4),
        case::zzzz_five(0.00005, 5),
        case::many_zeros_after_dp(0.00000000001, 11),
        case::negative_many_zeros_after_dp(-0.00000000001, 12),
        case::min_positive(f64::MIN_POSITIVE, 308),
        case::min_positive_sub_normal(f64::from_bits(1), 324),
        case::min_negative(f64::MIN, 310),
        case::min_negative_sub_normal(f64::from_bits(1u64 | (1u64 << 63)), 325),
        case::minus_one(-1.0, 2),
    )]
    fn digit_magnitude_test(f: f64, expected: usize) {
        let result = digit_magnitude(f);
        assert_eq!(expected, result);
    }
}