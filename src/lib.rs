use std::fmt::Display;

use num::Integer;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TZError {
    #[error("Invalid TZ length: {0}")]
    InvalidTZLength(String),
    #[error("Invalid digit in TZ: {0}")]
    InvalidTZDigit(String),
    #[error("Invalid TZ: {0}")]
    InvalidTZ(String),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TZDigit {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl Into<u8> for TZDigit {
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for TZDigit {
    type Error = TZError;

    fn try_from(d: u8) -> Result<Self, Self::Error> {
        match d {
            0 => Ok(TZDigit::Zero),
            1 => Ok(TZDigit::One),
            2 => Ok(TZDigit::Two),
            3 => Ok(TZDigit::Three),
            4 => Ok(TZDigit::Four),
            5 => Ok(TZDigit::Five),
            6 => Ok(TZDigit::Six),
            7 => Ok(TZDigit::Seven),
            8 => Ok(TZDigit::Eight),
            9 => Ok(TZDigit::Nine),
            _ => Err(TZError::InvalidTZDigit(d.to_string())),
        }
    }
}

impl TryFrom<char> for TZDigit {
    type Error = TZError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(TZDigit::Zero),
            '1' => Ok(TZDigit::One),
            '2' => Ok(TZDigit::Two),
            '3' => Ok(TZDigit::Three),
            '4' => Ok(TZDigit::Four),
            '5' => Ok(TZDigit::Five),
            '6' => Ok(TZDigit::Six),
            '7' => Ok(TZDigit::Seven),
            '8' => Ok(TZDigit::Eight),
            '9' => Ok(TZDigit::Nine),
            _ => Err(TZError::InvalidTZDigit(c.to_string())),
        }
    }
}

impl Display for TZDigit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl TZDigit {
    pub fn to_sum(&self) -> Self {
        match self {
            Self::Zero => Self::Zero,
            Self::One => Self::Two,
            Self::Two => Self::Four,
            Self::Three => Self::Six,
            Self::Four => Self::Eight,
            Self::Five => Self::One,
            Self::Six => Self::Three,
            Self::Seven => Self::Five,
            Self::Eight => Self::Seven,
            Self::Nine => Self::Nine,
        }
    }
}

pub type TZ = [TZDigit; 9];
pub const MAX_TZ_RANGE: usize = 999_999_999;

pub fn str2tz(s: &str) -> Result<TZ, TZError> {
    let s9 = format!("{:0>9}", s);
    if s9.len() != 9 {
        return Err(TZError::InvalidTZLength(s.to_string()));
    }
    let mut result = [TZDigit::Zero; 9];
    for (i, c) in s9.chars().enumerate() {
        let digit = TZDigit::try_from(c)?;
        result[i] = digit;
    }
    Ok(result)
}

pub fn int2tz<T>(n: T) -> Result<TZ, TZError>
where
    T: Integer + std::fmt::Display + std::convert::From<usize>,
{
    if n > MAX_TZ_RANGE.into() {
        return Err(TZError::InvalidTZLength(n.to_string()));
    }
    if n < 0.into() {
        return Err(TZError::InvalidTZ(n.to_string()));
    }
    str2tz(&n.to_string())
}

fn tz_last_digit(tz: &TZ) -> Result<TZDigit, TZError> {
    let sum = tz[0] as u8
        + tz[1].to_sum() as u8
        + tz[2] as u8
        + tz[3].to_sum() as u8
        + tz[4] as u8
        + tz[5].to_sum() as u8
        + tz[6] as u8
        + tz[7].to_sum() as u8;
    let remainder = sum % 10;
    if remainder == 0 {
        Ok(TZDigit::Zero)
    } else {
        TZDigit::try_from(10 - remainder)
    }
}

pub fn validate(tz: &TZ) -> bool {
    tz[8] == tz_last_digit(tz).unwrap_or(TZDigit::Zero)
}

pub fn tz2str(tz: &TZ) -> String {
    tz.iter().map(|d| d.to_string()).collect()
}

pub fn generate(start: usize, end: usize) -> Vec<TZ> {
    let mut end = if end > MAX_TZ_RANGE {
        MAX_TZ_RANGE
    } else {
        end
    };
    let mut start = if start > end { end } else { start };
    start -= start % 10;
    end -= end % 10;
    let capacity = (end - start + 1) / 10;
    let mut result = Vec::with_capacity(capacity);
    for i in (start..=end).step_by(10) {
        let mut tz = int2tz(i).unwrap();
        tz[8] = tz_last_digit(&tz).unwrap();
        result.push(tz);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_9() {
        let tz = str2tz("123456789").unwrap();
        assert_eq!(
            tz,
            [
                TZDigit::One,
                TZDigit::Two,
                TZDigit::Three,
                TZDigit::Four,
                TZDigit::Five,
                TZDigit::Six,
                TZDigit::Seven,
                TZDigit::Eight,
                TZDigit::Nine
            ]
        );
        let tz = str2tz("002345678").unwrap();
        assert_eq!(
            tz,
            [
                TZDigit::Zero,
                TZDigit::Zero,
                TZDigit::Two,
                TZDigit::Three,
                TZDigit::Four,
                TZDigit::Five,
                TZDigit::Six,
                TZDigit::Seven,
                TZDigit::Eight
            ]
        );
        let tz = str2tz("000000000").unwrap();
        assert_eq!(tz, [TZDigit::Zero; 9]);
        let tz = str2tz("999999999").unwrap();
        assert_eq!(tz, [TZDigit::Nine; 9]);
    }

    #[test]
    fn test_str_long() {
        let tz = str2tz("1234567890");
        assert!(tz.is_err());
        let tz = str2tz("12345678901");
        assert!(tz.is_err());
        let tz = str2tz("123456789012");
        assert!(tz.is_err());
    }

    #[test]
    fn test_str_short() {
        assert_eq!(str2tz("12345678").unwrap(), str2tz("012345678").unwrap());
        assert_eq!(str2tz("1234567").unwrap(), str2tz("001234567").unwrap());
        assert_eq!(str2tz("123456").unwrap(), str2tz("000123456").unwrap());
        assert_eq!(str2tz("12345").unwrap(), str2tz("000012345").unwrap());
        assert_eq!(str2tz("1234").unwrap(), str2tz("000001234").unwrap());
        assert_eq!(str2tz("123").unwrap(), str2tz("000000123").unwrap());
        assert_eq!(str2tz("12").unwrap(), str2tz("000000012").unwrap());
        assert_eq!(str2tz("1").unwrap(), str2tz("000000001").unwrap());
        assert_eq!(str2tz("").unwrap(), str2tz("000000000").unwrap());
    }

    #[test]
    fn test_int_good() {
        assert_eq!(int2tz(123456789).unwrap(), str2tz("123456789").unwrap());
        assert_eq!(int2tz(11).unwrap(), str2tz("000000011").unwrap());
        assert_eq!(int2tz(MAX_TZ_RANGE).unwrap(), str2tz("999999999").unwrap());
    }

    #[test]
    fn test_bad_input() {
        let tz = int2tz(1_000_000_000);
        assert!(tz.is_err());
        let tz = str2tz("-1");
        assert!(tz.is_err());
        let tz = str2tz("12345a789");
        assert!(tz.is_err());
    }

    #[test]
    fn test_valid_tz() {
        let input = [37015971, 037015906, 037015955, 037015963];
        for i in input {
            let tz = int2tz(i).unwrap();
            assert!(validate(&tz));
        }
    }

    #[test]
    fn test_invalid_tz() {
        let input = [3701591, 037015936, 037515955, 0000963];
        for i in input {
            let tz = int2tz(i).unwrap();
            assert!(!validate(&tz));
        }
    }

    #[test]
    fn test_generate() {
        let list = generate(37015900, 37016000);
        assert_eq!(list.len(), 11);
        let valid = str2tz("037015971").unwrap();
        assert!(list.contains(&valid));
        let list = generate(100, 200);
        assert_eq!(list.len(), 11);
        let valid = int2tz(141).unwrap();
        assert!(list.contains(&valid));
    }

    #[test]
    fn test_generate_invalid() {
        let list = generate(300, 3);
        assert_eq!(list.len(), 1);
        assert_eq!(list[0], [TZDigit::Zero; 9]);
        let list = generate(0, 1);
        assert_eq!(list.len(), 1);
        assert_eq!(list[0], [TZDigit::Zero; 9]);
        let list = generate(0, 0);
        assert_eq!(list.len(), 1);
        assert_eq!(list[0], [TZDigit::Zero; 9]);
        let list = generate(MAX_TZ_RANGE + 10, MAX_TZ_RANGE + 200);
        assert_eq!(list.len(), 1);
        assert_eq!(list[0], int2tz(999_999_998).unwrap());
    }
}
