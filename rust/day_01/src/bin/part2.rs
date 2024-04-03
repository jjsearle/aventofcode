fn main() {
    let input = include_str!("../input.txt");
    let answer = process(input);
    println!("Answer: {answer}");
}

fn process(input: &str) -> u32 {
    const TABLE: [(&str, u32); 10] = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let mut first = 0;
            let mut last = 0;
            for n in 0..line.len() {
                let char = chars.next().unwrap();
                match char.is_numeric() {
                    true => {
                        let val = char.to_digit(10).unwrap();
                        if first == 0 {
                            first = val
                        };
                        last = val
                    }
                    _ => {
                        for (s, val) in TABLE {
                            if line[n..].starts_with(s) {
                                if first == 0 {
                                    first = val
                                };
                                last = val
                            }
                        }
                    }
                }
            }
            first * 10 + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let examples = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let sum = process(&examples);
        assert_eq!(sum, 281);
    }
}
