use num_complex::Complex;

pub fn from_str(s: &str) -> Result<Complex<f64>, String> {
    let result= from_bracket_form(s)
        .or_else(|| from_standard_form(s));
    
    result.ok_or(format!("Cannot parse '{s}' to a complex number"))
}

/// Parse a complex number from the bracketed form `{a, b}`.
fn from_bracket_form(s: &str) -> Option<Complex<f64>> {
    let parts: Vec<&str> = s.trim()
        .strip_prefix("{")?
        .strip_suffix("}")?
        .split(',')
        .map(|s| s.trim())
        .collect();

    if parts.len() != 2 {
        return None;
    }

    let re = parts[0].parse::<f64>().ok()?;
    let im = parts[1].parse::<f64>().ok()?;

    Some(Complex::new(re, im))
}

/// Parse a complex number from the standard form 'a + bi'. This works
/// where numbers can be negative or are exponential, e.g. `-1.2e-7 - 3.0e-10`
/// will work just fine. 
fn from_standard_form(s: &str) -> Option<Complex<f64>> {
    // Strip out all whitespace, but ensure that single spaces only 
    // exist before number signs. Make the other possible instance of
    // number signs, in exponents, safe from this though.
    let cleanish: String = s.to_lowercase()
        .replace("e-", "eM")
        .replace("e+", "eP")
        .chars()
        .filter(|c| !c.is_whitespace())
        .flat_map(|c| {
            if c == '+' || c == '-' {
                vec![' ', c]
            } else {
                vec![c]
            }
        })
        .collect();

    // an edge case with redundant `+` as in `a + -bi`.
    let cleaned = cleanish.replace("+ -", "-");

    // split on the valid spaces to get only 1 or 2 parts
    let parts: Vec<&str> = cleaned
        .split(' ')
        .filter(|p| !p.is_empty())
        .collect();

    // one part -> number is real only or imaginary only
    // two parts -> number has real part and imaginary part
    if parts.len() == 1 {
        if parts[0].contains('i') {
            let im = imaginary_value(parts[0]).ok()?;
            return Some(Complex::new(0.0, im));
        } else {
            let re = real_value(parts[0]).ok()?;
            return Some(Complex::new(re, 0.0));
        }
    } else if parts.len() == 2 {
        let re = real_value(parts[0]).ok()?;
        let im = imaginary_value(parts[1]).ok()?;
        return Some(Complex::new(re, im));
    }

    None
        
}

/// Parse to number after removing the i suffix, but also deal with special cases of
/// `i` and `-i`. Put any exponents back to their correct form.
fn imaginary_value(s: &str) -> Result<f64, std::num::ParseFloatError> {
     match s {
        "i" => Ok(1.0),
        "-i" => Ok(-1.0),
        _ => s[..s.len() - 1].replace("eM", "e-").replace("eP", "e+").parse::<f64>()
    }
}

// Parse to number but fix any exponents back to their correct form first.
fn real_value(s: &str) -> Result<f64, std::num::ParseFloatError> {
    s.replace("eM", "e-").replace("eP", "e+").parse::<f64>()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(
        input, expected,
        case::bracket_valid("{2, -2}", Complex::new(2.0, -2.0)),
        case::bracket_valid_again("{-7, 3}", Complex::new(-7.0, 3.0)),
        case::bracket_zeros("{0, 0}", Complex::ZERO),
        case::bracket_exponents("{1.0e12, -0.7E4}", Complex::new(1.0e12, -0.7e4)),
        case::bracket_whitespace_ok("  {   1.0   ,     7.0    }", Complex::new(1.0, 7.0)),
        case::plain_real_only("34.5", Complex::new(34.5, 0.0)),
        case::plain_real_only_negative("-1", Complex::new(-1.0, 0.0)),
        case::plain_real_only_exponent("-1e+8", Complex::new(-1e+8, 0.0)),
        case::plain_imaginary_only("10i", Complex::new(0.0, 10.0)),
        case::plain_imaginary_only_negative("-13i", Complex::new(0.0, -13.0)),
        case::plain_imaginary_only_exponent("1.2e-7i", Complex::new(0.0, 1.2e-7)),
        case::plain_imaginary_i_only("i", Complex::new(0.0, 1.0)),
        case::plain_imaginary_both_parts("2 + 3i", Complex::new(2.0, 3.0)),
        case::plain_imaginary_both_parts_real_negative("-2.0 + 3i", Complex::new(-2.0, 3.0)),
        case::plain_imaginary_both_parts_imaginary_negative("2 - 3.4i", Complex::new(2.0, -3.4)),
        case::plain_whitespace_ok("  +   2.3    -     4i   ", Complex::new(2.3, -4.0)),
        case::plain_with_exponents("-2.1e-7 - 4.1E-7i", Complex::new(-2.1e-7, -4.1e-7)),
        case::plain_redundant_plus("2 + -1i", Complex::new(2.0, -1.0)),
        case::plain_redundant_starting_plus("+2", Complex::new(2.0, 0.0)),
        case::plain_redundant_starting_plus_imaginary("+3i", Complex::new(0.0, 3.0))
    )]
    fn from_str_works(input: &str, expected: Complex<f64>) {
        let result = from_str(input).unwrap();
        assert_eq!(expected, result);
    }

    #[rstest(
        input,
        case::not_a_number("fgdfgdfg"),
        case::blank(""),
        case::no_brackets("1.0, -1.0"),
        case::missing_start_bracket("1.0, 2.0}"),
        case::missing_end_bracket("{1.0, 1.0"),
        case::missing_comma("{1.0 2.0}"),
        case::too_many_parts("{1.0, 2.0, 3.0}"),
        case::first_not_a_number("{b, 1.0}"),
        case::second_not_a_number("-2.0, zz"),
        case::plain_real_not_a_number("z + 2i"),
        case::plain_imaginary_not_a_number("2 - wi"),
        case::plain_too_many_parts("2 + 4 + 3i")
    )]
    fn parsing_invalid_str_returns_none(input: &str) {
        let result = from_str(input);
        assert!(result.is_err());
    }
}