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
    let last_digit = bytes.iter().rev().find_map(|&byte| digit(byte)).unwrap();

    10 * first_digit + last_digit
}

fn digit(byte: u8) -> Option<usize> {
    match byte {
        b'0'..=b'9' => Some((byte - b'0') as usize),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_outer_digits() {
        assert_eq!(extract_outer_digits(b"1abc2"), 12);
        assert_eq!(extract_outer_digits(b"pqr3stu8vwx"), 38);
        assert_eq!(extract_outer_digits(b"a1b2c3d4e5f"), 15);
        assert_eq!(extract_outer_digits(b"treb7uchet"), 77);
    }
}
