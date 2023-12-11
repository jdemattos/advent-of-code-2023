use atoi::atoi;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let bytes = File::open("input.txt")?;
    let reader = BufReader::new(bytes);

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
    let mut divider = bytes.split(|byte| *byte == b':');

    let game_id = atoi::<usize>(
        divider
            .next()
            .unwrap()
            .split(|byte| *byte == b' ')
            .nth(1)
            .unwrap(),
    )
    .unwrap();

    let tokens = divider
        .next()
        .unwrap()
        .split(|byte| !byte.is_ascii_alphanumeric())
        .filter(|token| !token.is_empty());

    let mut count: usize = 0;
    for token in tokens {
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

    #[test]
    fn test_valid_game() {
        assert_eq!(
            valid_game(b"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Some(1)
        );
        assert_eq!(
            valid_game(b"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            Some(2)
        );
        assert_eq!(
            valid_game(b"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            None
        );
        assert_eq!(
            valid_game(b"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            None
        );
        assert_eq!(
            valid_game(b"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            Some(5)
        );
    }
}
