fn calibration_values(consider_words: ConsiderWords) -> impl Iterator<Item = CalibrationValues> {
    include_str!("input.txt")
        .lines()
        .map(move |line| CalibrationValues(line, consider_words))
}

#[derive(Debug, Copy, Clone)]
struct CalibrationValues(&'static str, ConsiderWords);

#[derive(Debug, Copy, Clone)]
enum ConsiderWords {
    Yes,
    No,
}

impl Iterator for CalibrationValues {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let Self(line, consider_words) = self;

        loop {
            if line.is_empty() {
                return None;
            }

            let digit = parse_digit(line, *consider_words);
            *line = &line[1..];
            if digit.is_some() {
                return digit;
            }
        }
    }
}

fn parse_digit(input: &str, consider_words: ConsiderWords) -> Option<u32> {
    let digit = input.chars().next().and_then(|c| c.to_digit(10));
    if digit.is_some() || matches!(consider_words, ConsiderWords::No) {
        return digit;
    }

    [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .into_iter()
    .find_map(|(word, value)| input.starts_with(word).then_some(value))
}

fn recover_calibration_value(mut values: CalibrationValues) -> u32 {
    let first = values.next().expect("no digits");
    let last = values.last().unwrap_or(first);

    first * 10 + last
}

fn main() {
    println!(
        "part 1: {}",
        calibration_values(ConsiderWords::No)
            .map(recover_calibration_value)
            .sum::<u32>()
    );
    println!(
        "part 2: {}",
        calibration_values(ConsiderWords::Yes)
            .map(recover_calibration_value)
            .sum::<u32>()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_digits() {
        let input = "12threefoureighthree4";

        assert_eq!(
            CalibrationValues(input, ConsiderWords::No).collect::<Vec<_>>(),
            vec![1, 2, 4]
        );
        assert_eq!(
            CalibrationValues(input, ConsiderWords::Yes).collect::<Vec<_>>(),
            vec![1, 2, 3, 4, 8, 3, 4]
        );
    }
}
