use std::error::Error;
use std::fs::read_to_string;

fn solve_first() -> Result<u32, Box<dyn Error>> {
    solve(|(id, games)| {
        if games
            .iter()
            .all(|pull| pull.red <= 12 && pull.green <= 13 && pull.blue <= 14)
        {
            id
        } else {
            0
        }
    })
}

fn solve_second() -> Result<u32, Box<dyn Error>> {
    solve(|(_, games)| {
        match (
            games.iter().map(|pull| pull.red).max(),
            games.iter().map(|pull| pull.green).max(),
            games.iter().map(|pull| pull.blue).max(),
        ) {
            (Some(red), Some(green), Some(blue)) => red * green * blue,
            _ => 0,
        }
    })
}

fn solve<F>(transform: F) -> Result<u32, Box<dyn Error>>
where
    F: Fn((u32, Vec<Pull>)) -> u32,
{
    let file = read_to_string("input")?;

    Ok(file
        .lines()
        .filter_map(|line| line.split_once(":"))
        .filter_map(|id_and_game| {
            id_and_game
                .0
                .replace("Game ", "")
                .trim()
                .parse::<u32>()
                .ok()
                .map(|id| (id, id_and_game.1))
        })
        .map(|(id, games)| (id, parse_row(games.to_string())))
        .map(|(id, games)| transform((id, games)))
        .filter(|r| *r != 0)
        .sum::<u32>())
}

fn parse_row(games: String) -> Vec<Pull> {
    games
        .split(";")
        .map(|game| parse_pull(game.trim().to_string()))
        .collect()
}

fn parse_pull(item: String) -> Pull {
    item.trim()
        .split(",")
        .filter_map(|color| color.trim().split_once(" "))
        .filter_map(|color| color.0.trim().parse::<u32>().ok().map(|i| (color.1, i)))
        .fold(
            Pull {
                red: 0,
                green: 0,
                blue: 0,
            },
            |acc, (color, num)| match color.trim() {
                "red" => Pull { red: num, ..acc },
                "green" => Pull { green: num, ..acc },
                "blue" => Pull { blue: num, ..acc },
                _ => acc,
            },
        )
}

struct Pull {
    red: u32,
    green: u32,
    blue: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_1() {
        let start = std::time::Instant::now();
        let result = solve_first();
        match result {
            Err(e) => panic!("{:?}", e),
            Ok(result) => assert_eq!(result, 2913),
        }
        println!("solve_1: {:?}", start.elapsed().as_micros());
    }

    #[test]
    fn solve_2() {
        let start = std::time::Instant::now();
        let result = solve_second();
        match result {
            Err(e) => panic!("{:?}", e),
            Ok(result) => assert_eq!(result, 55593),
        }
        println!("solve_2: {:?}", start.elapsed().as_micros());
    }
}
