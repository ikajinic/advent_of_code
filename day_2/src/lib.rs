use std::fs::read_to_string;

pub fn solve_first() -> u32 {
    let file = read_to_string("input").unwrap();
    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;
    let mut sum = 0;

    'outer: for line in file.lines() {
        let mut text = line.to_string();
        text = text.replace("Game ", "");
        let mut separated = text.split(":");
        let id = separated.next().unwrap().parse::<u32>().unwrap();
        let games = separated.last().unwrap().split(";");
        for game in games {
            let pulled = game.split(",");
            for color in pulled {
                let mut parsed = color.trim().split(" ");
                let value = parsed.next().unwrap().parse::<u32>().unwrap();
                let color = parsed.last().unwrap();
                if color == "red" && value > red_limit {
                    continue 'outer;
                }
                if color == "green" && value > green_limit {
                    continue 'outer;
                }
                if color == "blue" && value > blue_limit {
                    continue 'outer;
                }
            }
        }
        sum += id;
    }
    sum
}

pub fn solve_second() -> u32 {
    let file = read_to_string("input").unwrap();
    let mut sum = 0;

    for line in file.lines() {
        let mut text = line.to_string();
        text = text.replace("Game ", "");
        let separated = text.split(":");
        let games = separated.last().unwrap().split(";");
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for game in games {
            let pulled = game.split(",");
            for color in pulled {
                let mut parsed = color.trim().split(" ");
                let value = parsed.next().unwrap().parse::<u32>().unwrap();
                let color = parsed.last().unwrap();
                if color == "red" && value > max_red {
                    max_red = value;
                }
                if color == "green" && value > max_green {
                    max_green = value;
                }
                if color == "blue" && value > max_blue {
                    max_blue = value;
                }
            }
        }
        sum += max_red * max_green * max_blue;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_1() {
        let result = solve_first();
        assert_eq!(result, 2913);
    }

    #[test]
    fn solve_2() {
        let result = solve_second();
        assert_eq!(result, 55593);
    }
}
