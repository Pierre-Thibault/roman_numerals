//! A function to convert integers to their roman numeral representation as strings.
//!
//! Values from 1 to 3999 are possible otherwise, it returns an OutOfRangeError.
//! Zero does not exist in roman numerals.
use range_check::{Check, OutOfRangeError};

/// The roman numeral digits as an array of characters in ascending order.
pub const ROMAN_NUMERALS: [char; 7] = ['I', 'V', 'X', 'L', 'C', 'D', 'M'];

/// The maxium possible value. 3999
pub const MAXIMUM: u16 = if ROMAN_NUMERAL_LEN % 2 == 0 {
    u16::pow(10, ROMAN_NUMERAL_LEN / 2 - 1) * 9 - 1
} else {
    u16::pow(10, ROMAN_NUMERAL_LEN / 2) * 4 - 1
};

/// The minimum possible value. 1
pub const MINIMUM: u16 = 1;

const EMPTY_STRING: String = String::new();
const ROMAN_NUMERAL_LEN: u32 = ROMAN_NUMERALS.len() as u32;

/// Convert an integer to its roman numeral representation.
///
/// value: Value to convert.
///
/// return: A string or an OutOfRangeError.
pub fn to_roman_numeral(value: u16) -> Result<String, OutOfRangeError<u16>> {
    value.check_range(MINIMUM..MAXIMUM + 1)?;

    let mut result = String::new();
    let string_value = value.to_string();
    let string_value_length = string_value.len();
    string_value.chars().enumerate().for_each(|(index, digit)| {
        let romain_digit_index = (string_value_length - index - 1) * 2;
        let unit_roman_digit = ROMAN_NUMERALS[romain_digit_index].to_string();
        let five_roman_digit = ROMAN_NUMERALS
            .get(romain_digit_index + 1)
            .unwrap_or(&'_')
            .to_string();
        let ten_roman_digit = ROMAN_NUMERALS
            .get(romain_digit_index + 2)
            .unwrap_or(&'_')
            .to_string();
        let repeat_unit_roman_digit = |times: usize| -> String { unit_roman_digit.repeat(times) };
        assert!(
            digit < '5' || (five_roman_digit != "_" && (digit < '9' || ten_roman_digit != "_")),
            "Some roman numeral digits are missing."
        );
        result.push_str(&match digit {
            '0' => EMPTY_STRING,
            '1' => unit_roman_digit,
            '2' => repeat_unit_roman_digit(2),
            '3' => repeat_unit_roman_digit(3),
            '4' => unit_roman_digit + &five_roman_digit,
            '5' => five_roman_digit,
            '6' => five_roman_digit + &unit_roman_digit,
            '7' => five_roman_digit + &repeat_unit_roman_digit(2),
            '8' => five_roman_digit + &repeat_unit_roman_digit(3),
            '9' => unit_roman_digit + &ten_roman_digit,
            _ => panic!("Unknown digit {}", digit),
        });
    });
    Ok(result)
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    use super::*;

    #[test]
    fn test_odered_set() {
        let file = File::open("rsc/roman-ordered-set.txt").unwrap();
        for (index, line) in BufReader::new(file).lines().enumerate() {
            assert_eq!(to_roman_numeral((index + 1) as u16).unwrap(), line.unwrap());
        }
    }

    #[test]
    fn test_random_set() {
        let file = File::open("rsc/roman-random-set.txt").unwrap();
        let re = Regex::new(r"(\w+) - (\d+)").unwrap();
        for line in BufReader::new(file).lines() {
            let captures = re.captures(line.as_ref().unwrap()).unwrap();
            let value: u16 = captures[2].parse().unwrap();
            assert_eq!(to_roman_numeral(value).unwrap(), captures[1]);
        }
    }

    #[test]
    fn test_out_of_range() {
        for value in [0, 3999 + 1] {
            assert!(to_roman_numeral(value).is_err())
        }
    }

    #[test]
    fn test_last_roman_number() {
        assert!(to_roman_numeral(3999).is_ok())
    }
}
