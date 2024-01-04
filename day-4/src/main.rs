use atoi::atoi;
use std::collections::BTreeSet;

fn main() {
    let input = include_bytes!("../input.txt");
    let score = process_input(input);

    println!("{}", score);
}

fn process_input(bytes: &[u8]) -> usize {
    let lines = bytes
        .split(|&byte| byte == b'\n')
        .filter(|line| !line.is_empty());

    let mut total_score = 0;

    for line in lines {
        let mut sections = line.split(|&byte| byte == b':' || byte == b'|');
        let _card_section = sections.next();

        let winning_numbers_section = sections.next().unwrap();
        let mut winning_numbers = BTreeSet::new();
        for winning_number in numbers(winning_numbers_section) {
            winning_numbers.insert(winning_number);
        }

        let numbers_section = sections.next().unwrap();
        let mut win_count: usize = 0;
        for number in numbers(numbers_section) {
            if winning_numbers.contains(&number) {
                win_count += 1;
            }
        }

        match win_count {
            0 => {}
            _ => {
                total_score += 1 << (win_count - 1);
            }
        }
    }

    total_score
}

fn numbers(bytes: &[u8]) -> impl Iterator<Item = usize> + '_ {
    bytes
        .split(|&byte| byte == b' ')
        .filter(|&slice| !slice.is_empty())
        .map(|slice| atoi::<usize>(slice).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input_with_sample() {
        let input = br#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;
        println!("{}", std::str::from_utf8(input).unwrap());
        let actual = process_input(input);
        let expected = 13;

        assert_eq!(actual, expected, "expected {}, actual {}", expected, actual);
    }

    #[test]
    fn test_process_input() {
        let input = include_bytes!("../input.txt");
        let actual = process_input(input);
        let expected = 25004;

        assert_eq!(actual, expected, "expected {}, actual {}", expected, actual);
    }
}
