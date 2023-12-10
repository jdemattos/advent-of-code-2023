use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let mut sum: usize = 0;
    for line in reader.lines() {
        sum += extract_outer_digits(line?.as_bytes());
    }

    println!("{}", sum);

    Ok(())
}

fn extract_outer_digits(bytes: &[u8]) -> usize {
    let first_digit = bytes.iter().find_map(|&byte| digit(byte)).unwrap();
    // NOTE: the DoubleEndedIterator trait allows us to iterate in reverse efficiently via rev(). this is pretty unusual
    // to see in a programming language.
    let last_digit = bytes.iter().rev().find_map(|&byte| digit(byte)).unwrap();

    10 * first_digit + last_digit
}

fn digit(byte: u8) -> Option<usize> {
    match byte {
        b'1'..=b'9' => Some((byte - b'0') as usize),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    #[test]
    fn test_extract_outer_digits() {
        assert_extract_outer_digits(b"1abc2", 12);
        assert_extract_outer_digits(b"pqr3stu8vwx", 38);
        assert_extract_outer_digits(b"a1b2c3d4e5f", 15);
        assert_extract_outer_digits(b"treb7uchet", 77);
    }

    fn assert_extract_outer_digits(input: &[u8], expected: usize) {
        let actual = extract_outer_digits(input);
        let input_str = str::from_utf8(input).unwrap();
        assert_eq!(
            actual, expected,
            "expected {} from {}, actual {}",
            expected, input_str, actual
        );
    }
}
