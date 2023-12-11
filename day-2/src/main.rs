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
    let mut iter = TokenIterator::new(bytes);

    let token = iter.next();
    debug_assert!(token.unwrap() == Token::Game);
    let Token::Uint(game_id) = iter.next().unwrap() else {
        unreachable!();
    };
    let token = iter.next();
    debug_assert!(token.unwrap() == Token::Colon);

    while let Some(token) = iter.next() {
        if let Token::Uint(count) = token {
            let Token::Color(color) = iter.next().unwrap() else {
                unreachable!();
            };

            match color {
                Color::Red => {
                    if count > RED_COUNT {
                        return None;
                    }
                }
                Color::Green => {
                    if count > GREEN_COUNT {
                        return None;
                    }
                }
                Color::Blue => {
                    if count > BLUE_COUNT {
                        return None;
                    }
                }
            }
        }
    }

    Some(game_id)
}

const RED_COUNT: usize = 12;
const GREEN_COUNT: usize = 13;
const BLUE_COUNT: usize = 14;

// NOTE: I wrote my own token parser because I wasn't sure if how "lazy" Rust std libraries are for identifying tokens
// and producing byte slices in an efficient manner. I may rewrite this using something like .split(b' ') which may have
// the same performance. I also just wanted an excuse to try token parsing with Rust enum types.

/*
EBNF notation
game ::= 'Game ' uint ': ' samples
samples ::= sample ( '; ' sample )*
sample ::= count_color ( ', ' count_color )*
count_color ::= uint ' ' color
uint ::= digit+
digit ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
color ::= 'red' | 'green' | 'blue'
 */

#[derive(Debug, PartialEq)]
enum Token {
    Game,
    Colon,
    Semicolon,
    Comma,
    Uint(usize),
    Color(Color),
}

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

struct TokenIterator<'a> {
    bytes: &'a [u8],
    index: usize,
}

impl<'a> TokenIterator<'a> {
    fn new(bytes: &'a [u8]) -> TokenIterator<'a> {
        TokenIterator { bytes, index: 0 }
    }

    fn parse_uint(&mut self) -> Token {
        let start = self.index;
        while self.index < self.bytes.len() && self.bytes[self.index].is_ascii_digit() {
            self.index += 1;
        }

        let span = &self.bytes[start..self.index];
        let number = atoi::<usize>(span).unwrap();

        Token::Uint(number)
    }

    fn parse_color(&mut self) -> Token {
        let start = self.index;
        while self.index < self.bytes.len() && self.bytes[self.index].is_ascii_alphabetic() {
            self.index += 1;
        }

        let span = &self.bytes[start..self.index];
        match span {
            b"red" => Token::Color(Color::Red),
            b"green" => Token::Color(Color::Green),
            b"blue" => Token::Color(Color::Blue),
            _ => unreachable!(),
        }
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.bytes.len() {
            match self.bytes[self.index] {
                b' ' => self.index += 1,
                b'G' => {
                    self.index += b"Game ".len();
                    return Some(Token::Game);
                }
                b':' => {
                    self.index += b": ".len();
                    return Some(Token::Colon);
                }
                b';' => {
                    self.index += b"; ".len();
                    return Some(Token::Semicolon);
                }
                b',' => {
                    self.index += b", ".len();
                    return Some(Token::Comma);
                }
                b'0'..=b'9' => {
                    return Some(self.parse_uint());
                }
                _ => {
                    return Some(self.parse_color());
                }
            }
        }
        None
    }
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
