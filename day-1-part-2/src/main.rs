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
    let mut machine = StateMachine::new();

    let first_digit = bytes
        .iter()
        .find_map(|byte| match byte {
            b'1'..=b'9' => Some((byte - b'0') as usize),
            _ => machine.take(*byte),
        })
        .unwrap();

    machine.reset();

    let last_digit = bytes
        .iter()
        .rev()
        .find_map(|byte| match byte {
            b'1'..=b'9' => Some((byte - b'0') as usize),
            _ => machine.take_rev(*byte),
        })
        .unwrap();

    10 * first_digit + last_digit
}

// NOTE: there almost certainly is a better way to do this... but I went down the rabbit hole of trying to keep the
// solution with compile time behavior. I'll investigate using some kind of Trie data structure later.
struct StateMachine {
    state: State,
}

impl StateMachine {
    fn new() -> Self {
        Self {
            state: State::Reset,
        }
    }

    fn reset(&mut self) {
        self.state = State::Reset;
    }

    fn take(&mut self, byte: u8) -> Option<usize> {
        self.state = match (byte, &self.state) {
            (b'n', State::Fo) => State::On,
            (b'n', State::O) => State::On,
            (b'e', State::On) => State::One,

            (b'w', State::T) => State::Tw,
            (b'o', State::Tw) => State::Two,

            (b'h', State::T) => State::Th,
            (b'r', State::Th) => State::Thr,
            (b'e', State::Thr) => State::Thre,
            (b'e', State::Thre) => State::Three,

            (b'o', State::F) => State::Fo,
            (b'u', State::Fo) => State::Fou,
            (b'r', State::Fou) => State::Four,

            (b'i', State::F) => State::Fi,
            (b'v', State::Fi) => State::Fiv,
            (b'e', State::Fiv) => State::Five,

            (b'i', State::S) => State::Si,
            (b'x', State::Si) => State::Six,

            (b'e', State::S) => State::Se,
            (b'v', State::Se) => State::Sev,
            (b'e', State::Sev) => State::Seve,
            (b'n', State::Seve) => State::Seven,

            (b'i', State::Thre) => State::Ei,
            (b'i', State::Se) => State::Ei,
            (b'i', State::Seve) => State::Ei,
            (b'i', State::E) => State::Ei,
            (b'g', State::Ei) => State::Eig,
            (b'h', State::Eig) => State::Eigh,
            (b't', State::Eigh) => State::Eight,

            (b'i', State::Nin) => State::Ni,
            (b'i', State::N) => State::Ni,
            (b'n', State::Ni) => State::Nin,
            (b'e', State::Nin) => State::Nine,

            (b'e', _) => State::E,
            (b'f', _) => State::F,
            (b'n', _) => State::N,
            (b'o', _) => State::O,
            (b's', _) => State::S,
            (b't', _) => State::T,

            _ => State::Reset,
        };

        match self.state {
            State::One => Some(1),
            State::Two => Some(2),
            State::Three => Some(3),
            State::Four => Some(4),
            State::Five => Some(5),
            State::Six => Some(6),
            State::Seven => Some(7),
            State::Eight => Some(8),
            State::Nine => Some(9),
            _ => None,
        }
    }

    fn take_rev(&mut self, byte: u8) -> Option<usize> {
        self.state = match (byte, &self.state) {
            (b'n', State::Ee) => State::En,
            (b'n', State::Ne) => State::En,
            (b'n', State::Neve) => State::En,
            (b'n', State::E) => State::En,
            (b'o', State::En) => State::Eno,

            (b'w', State::Ruo) => State::Ow,
            (b'w', State::O) => State::Ow,
            (b't', State::Ow) => State::Owt,

            (b'e', State::Ee) => State::Ee,
            (b'e', State::Ne) => State::Ee,
            (b'e', State::Neve) => State::Ee,
            (b'e', State::E) => State::Ee,
            (b'r', State::Ee) => State::Eer,
            (b'h', State::Eer) => State::Eerh,
            (b't', State::Eerh) => State::Eerht,

            (b'u', State::Eer) => State::Ru,
            (b'u', State::R) => State::Ru,
            (b'o', State::Ru) => State::Ruo,
            (b'f', State::Ruo) => State::Ruof,

            (b'v', State::Ee) => State::Ev,
            (b'v', State::Neve) => State::Ev,
            (b'v', State::E) => State::Ev,
            (b'i', State::Nev) => State::Evi,
            (b'i', State::Ev) => State::Evi,
            (b'f', State::Evi) => State::Evif,

            (b'i', State::X) => State::Xi,
            (b's', State::Xi) => State::Xis,

            (b'e', State::En) => State::Ne,
            (b'e', State::N) => State::Ne,
            (b'v', State::Ne) => State::Nev,
            (b'e', State::Nev) => State::Neve,
            (b's', State::Neve) => State::Neves,

            (b'h', State::T) => State::Th,
            (b'g', State::Th) => State::Thg,
            (b'i', State::Thg) => State::Thgi,
            (b'e', State::Thgi) => State::Thgie,

            (b'i', State::En) => State::Eni,
            (b'n', State::Eni) => State::Enin,

            (b'e', _) => State::E,
            (b'n', _) => State::N,
            (b'o', _) => State::O,
            (b'r', _) => State::R,
            (b't', _) => State::T,
            (b'x', _) => State::X,

            _ => State::Reset,
        };

        match self.state {
            State::Eno => Some(1),
            State::Owt => Some(2),
            State::Eerht => Some(3),
            State::Ruof => Some(4),
            State::Evif => Some(5),
            State::Xis => Some(6),
            State::Neves => Some(7),
            State::Thgie => Some(8),
            State::Enin => Some(9),
            _ => None,
        }
    }
}

enum State {
    Reset,
    O,
    On,
    One,
    T,
    Tw,
    Two,
    Th,
    Thr,
    Thre,
    Three,
    F,
    Fo,
    Fou,
    Four,
    Fi,
    Fiv,
    Five,
    S,
    Si,
    Six,
    Se,
    Sev,
    Seve,
    Seven,
    E,
    Ei,
    Eig,
    Eigh,
    Eight,
    N,
    Ni,
    Nin,
    Nine,

    // reverse
    En,
    Eno,
    Ow,
    Owt,
    Ee,
    Eer,
    Eerh,
    Eerht,
    R,
    Ru,
    Ruo,
    Ruof,
    Ev,
    Evi,
    Evif,
    X,
    Xi,
    Xis,
    Ne,
    Nev,
    Neve,
    Neves,
    Thg,
    Thgi,
    Thgie,
    Eni,
    Enin,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    #[test]
    fn test_extract_outer_digits() {
        assert_extract_outer_digits(b"two1nine", 29);
        assert_extract_outer_digits(b"eightwothree", 83);
        assert_extract_outer_digits(b"abcone2threexyz", 13);
        assert_extract_outer_digits(b"xtwone3four", 24);
        assert_extract_outer_digits(b"4nineeightseven2", 42);
        assert_extract_outer_digits(b"zoneight234", 14);
        assert_extract_outer_digits(b"7pqrstsixteen", 76);
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
