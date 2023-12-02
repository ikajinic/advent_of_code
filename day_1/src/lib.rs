use std::error::Error;
use std::fs::read_to_string;

fn solve_first() -> Result<u32, Box<dyn Error>> {
    solve(|line| line.to_string())
}

fn solve_second() -> Result<u32, Box<dyn Error>> {
    solve(|line| {
        line.replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e")
    })
}

fn solve<F>(transform: F) -> Result<u32, Box<dyn Error>>
where
    F: Fn(&str) -> String,
{
    let file = read_to_string("input")?;
    Ok(file
        .lines()
        .map(|line| transform(line))
        .filter_map(|line| {
            let digits = line.chars().filter(|c| c.is_digit(10)).collect::<String>();
            let first = digits.chars().next()?;
            let last = digits.chars().last()?;
            format!("{first}{last}").parse::<u32>().ok()
        })
        .sum::<u32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_1() {
        let result = solve_first();
        match result {
            Err(e) => panic!("{:?}", e),
            Ok(result) => assert_eq!(result, 55108),
        }
    }

    #[test]
    fn solve_2() {
        let result = solve_second();
        match result {
            Err(e) => panic!("{:?}", e),
            Ok(result) => assert_eq!(result, 56324),
        }
    }
}
