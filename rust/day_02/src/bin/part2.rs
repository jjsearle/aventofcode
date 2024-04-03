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
            let _ = parse_game_id(game_it.next().unwrap());
            let grab_strings = split_grab_strings(game_it.next().unwrap());
            let mut h_red: u32 = 0;
            let mut h_green: u32 = 0;
            let mut h_blue: u32 = 0;
            let grabs: Vec<Grab> = grab_strings
                .iter()
                .map(|grab_str| parse_grab(grab_str))
                .collect();
            for grab in grabs.iter() {
                if grab.0 > h_red {
                    h_red = grab.0
                }
                if grab.1 > h_green {
                    h_green = grab.1
                }
                if grab.2 > h_blue {
                    h_blue = grab.2
                }
            }
            h_red * h_green * h_blue
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
    #[allow(dead_code)]
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
        let example = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let sum = process(&example);
        assert_eq!(sum, 48);
    }
}
