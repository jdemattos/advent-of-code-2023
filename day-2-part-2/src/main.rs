use atoi::atoi;
use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let bytes = File::open("input.txt")?;
    let reader = BufReader::new(bytes);

    let mut sum: usize = 0;
    for line in reader.lines() {
        sum += min_product_game(line?.as_bytes());
    }

    println!("{}", sum);

    Ok(())
}

fn min_product_game(bytes: &[u8]) -> usize {
    let mut tokens = bytes
        .split(|byte| *byte == b':')
        .nth(1)
        .unwrap()
        .split(|byte| !byte.is_ascii_alphanumeric())
        .filter(|token| !token.is_empty());

    let mut red_count: usize = 0;
    let mut green_count: usize = 0;
    let mut blue_count: usize = 0;

    while let Some(token) = tokens.next() {
        let count = atoi::<usize>(token).unwrap();

        match tokens.next().unwrap() {
            b"red" => {
                red_count = cmp::max(count, red_count);
            }
            b"green" => {
                green_count = cmp::max(count, green_count);
            }
            b"blue" => {
                blue_count = cmp::max(count, blue_count);
            }
            _ => unreachable!(),
        }
    }

    red_count * green_count * blue_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    #[test]
    fn test_min_product_game() {
        assert_min_product_game(
            b"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            48,
        );
        assert_min_product_game(
            b"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            12,
        );
        assert_min_product_game(
            b"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            1560,
        );
        assert_min_product_game(
            b"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            630,
        );
        assert_min_product_game(
            b"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            36,
        );
    }

    fn assert_min_product_game(input: &[u8], expected: usize) {
        let actual = min_product_game(input);
        let input_str = str::from_utf8(input).unwrap();
        assert_eq!(
            actual, expected,
            "expected {} from {}, actual {}",
            expected, input_str, actual
        );
    }
}
