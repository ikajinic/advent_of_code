use std::fs::read_to_string;

fn solve_first() -> Result<u32, Errors> {
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

fn solve_second() -> Result<u32, Errors> {
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

fn solve<F>(transform: F) -> Result<u32, Errors>
where
    F: Fn((u32, Vec<Pull>)) -> u32,
{
    let file = read_to_string("input").map_err(|_| Errors::FileError)?;

    Ok(file
        .lines()
        .into_iter()
        .map(|line| line.split(":"))
        .flat_map(|mut id_and_games| {
            match (
                id_and_games
                    .next()
                    .map(|i| i.replace("Game ", "").trim().parse::<u32>()),
                id_and_games.last(),
            ) {
                (Some(Ok(id)), Some(games)) => Ok((id, games)),
                _ => Err(Errors::ParsingError),
            }
        })
        .map(|(id, games)| (id, parse_row(games.to_string())))
        .map(|(id, games)| transform((id, games)))
        .filter(|r| *r != 0)
        .sum::<u32>())
}

fn parse_row(games: String) -> Vec<Pull> {
    games
        .split(";")
        .flat_map(|game| parse_pull(game.trim().to_string()))
        .collect()
}

fn parse_pull(item: String) -> Result<Pull, Errors> {
    Ok(item
        .trim()
        .split(",")
        .map(|color| color.trim().split(" "))
        .flat_map(
            |mut color| match (color.next().map(|i| i.parse::<u32>()), color.last()) {
                (Some(Ok(num)), Some(c)) => Ok((c, num)),
                _ => Err(Errors::NoDigitFound),
            },
        )
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
        ))
}

struct Pull {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
enum Errors {
    NoDigitFound,
    FileError,
    ParsingError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_1() {
        let result = solve_first();
        match result {
            Err(e) => panic!("{:?}", e),
            Ok(result) => assert_eq!(result, 2913),
        }
    }

    #[test]
    fn solve_2() {
        let result = solve_second();
        match result {
            Err(e) => panic!("{:?}", e),
            Ok(result) => assert_eq!(result, 55593),
        }
    }
}
