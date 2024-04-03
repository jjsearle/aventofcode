fn main() {
    let input = include_str!("../input.txt");
    let answer = process(input);
    println!("Answer: {answer}");
}

fn process(input: &str) -> u32 {
    let sum: u32 = input
        .lines()
        .map(|game| {
            let mut game_it = game.split(':');
            let game_id = parse_game_id(game_it.next().unwrap());
            let grab_strings = split_grab_strings(game_it.next().unwrap());
            let impossible_grabs: Vec<Grab> = grab_strings
                .iter()
                .map(|grab_str| parse_grab(grab_str))
                .filter(|g| !g.is_possible())
                .collect();
            if impossible_grabs.len() < 1 {
                game_id
            } else {
                0
            }
        })
        .sum();
    sum
}

fn split_grab_strings(input: &str) -> Vec<&str> {
    input.split(';').collect()
}

fn parse_grab(input: &str) -> Grab {
    let mut r: u32 = 0;
    let mut g: u32 = 0;
    let mut b: u32 = 0;

    for type_str in input.split(',') {
        let mut color_number_it = type_str.trim().split(' ');
        let num: u32 = color_number_it.next().unwrap().parse::<u32>().unwrap();
        let color: &str = color_number_it.next().unwrap();
        match color {
            "red" => r = num,
            "green" => g = num,
            "blue" => b = num,
            &_ => todo!(),
        }
    }
    Grab(r, g, b)
}

fn parse_game_id(input: &str) -> u32 {
    input.strip_prefix("Game ").unwrap().parse::<u32>().unwrap()
}

struct Grab(u32, u32, u32);

impl Grab {
    fn is_possible(&self) -> bool {
        const MAX_RED: u32 = 12;
        const MAX_GREEN: u32 = 13;
        const MAX_BLUE: u32 = 14;
        !(self.0 > MAX_RED || self.1 > MAX_GREEN || self.2 > MAX_BLUE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let examples = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let sum = process(&examples);
        assert_eq!(sum, 8);
    }

    #[test]
    fn test_is_possible() {
        let possible = Grab(10, 10, 10).is_possible();
        assert_eq!(possible, true);
        let possible = Grab(13, 10, 10).is_possible();
        assert_eq!(possible, false);
        let possible = Grab(10, 14, 10).is_possible();
        assert_eq!(possible, false);
        let possible = Grab(10, 10, 15).is_possible();
        assert_eq!(possible, false);
    }
}
