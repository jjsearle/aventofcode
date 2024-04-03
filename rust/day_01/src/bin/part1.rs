fn main() {
    let input = include_str!("../input.txt");
    let answer = process(input);
    println!("Answer: {answer}");
}

fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = line.chars().find_map(|c| c.to_digit(10)).unwrap();
            let last = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
            10 * first + last
        })
        .sum()
}
