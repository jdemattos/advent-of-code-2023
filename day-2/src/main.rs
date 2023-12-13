use atoi::atoi;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let mut sum: usize = 0;
    for line in reader.lines() {
        if let Some(game_id) = valid_game(line?.as_bytes()) {
            sum += game_id;
        }
    }

    println!("{}", sum);

    Ok(())
}

fn valid_game(bytes: &[u8]) -> Option<usize> {
    let mut alphanumeric_tokens = bytes
        .split(|byte| !byte.is_ascii_alphanumeric())
        .filter(|token| !token.is_empty());

    // skip "Game" token
    alphanumeric_tokens.next();

    let game_id = atoi::<usize>(alphanumeric_tokens.next().unwrap()).unwrap();

    let mut count: usize = 0;
    for token in alphanumeric_tokens {
        match token {
            b"red" => {
                if count > 12 {
                    return None;
                }
            }
            b"green" => {
                if count > 13 {
                    return None;
                }
            }
            b"blue" => {
                if count > 14 {
                    return None;
                }
            }
            _ => count = atoi::<usize>(token).unwrap(),
        }
    }

    Some(game_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    #[test]
    fn test_valid_game() {
        assert_valid_game(
            b"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            Some(1),
        );
        assert_valid_game(
            b"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            Some(2),
        );
        assert_valid_game(
            b"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            None,
        );
        assert_valid_game(
            b"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            None,
        );
        assert_valid_game(
            b"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            Some(5),
        );
    }

    fn assert_valid_game(input: &[u8], expected: Option<usize>) {
        let actual = valid_game(input);
        let input_str = str::from_utf8(input).unwrap();
        assert_eq!(
            actual, expected,
            "expected {:?} from {}, actual {:?}",
            expected, input_str, actual
        );
    }
}
