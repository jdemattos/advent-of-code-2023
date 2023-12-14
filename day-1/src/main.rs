fn main() {
    let sum = process_input();

    println!("{}", sum);
}

fn process_input() -> usize {
    let input = include_bytes!("../input.txt");
    let lines = input
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty());

    let sum = lines.map(|line| process_outer_digits(line)).sum();

    sum
}

fn process_outer_digits(line: &[u8]) -> usize {
    let first_digit = line.iter().find_map(parse_digit).unwrap();
    let last_digit = line.iter().rev().find_map(parse_digit).unwrap();

    10 * first_digit + last_digit
}

fn parse_digit(byte: &u8) -> Option<usize> {
    match byte {
        b'1'..=b'9' => Some((byte - b'0').into()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_outer_digits() {
        assert_process_outer_digits(b"1abc2", 12);
        assert_process_outer_digits(b"pqr3stu8vwx", 38);
        assert_process_outer_digits(b"a1b2c3d4e5f", 15);
        assert_process_outer_digits(b"treb7uchet", 77);
    }

    #[test]
    fn test_process_input() {
        let actual = process_input();
        let expected = 55_208;

        assert_eq!(actual, expected, "expected {}, actual {}", expected, actual);
    }

    fn assert_process_outer_digits(input: &[u8], expected: usize) {
        let actual = process_outer_digits(input);
        let input_str = std::str::from_utf8(input).unwrap();
        assert_eq!(
            actual, expected,
            "expected {} from {}, actual {}",
            expected, input_str, actual
        );
    }
}
