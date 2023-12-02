use regex::Regex;
use std::fs::read_to_string;

fn solve_first() -> Result<u32, Errors> {
    solve(|line| line.to_string())
}

fn solve_second() -> Result<u32, Errors> {
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

fn solve<F>(transform: F) -> Result<u32, Errors>
where
    F: Fn(&str) -> String,
{
    let file = read_to_string("input").map_err(|_| Errors::FileError)?;
    let re = Regex::new(r"[a-zA-A]").map_err(|_| Errors::RegexError)?;
    Ok(file
        .lines()
        .into_iter()
        .map(|line| transform(line))
        .map(|line| re.replace_all(&line, "").to_string())
        .flat_map(|line| parse_digits(line))
        .map(|digits| digits.0 * 10 + digits.1)
        .sum::<u32>())
}

fn parse_digits(line: String) -> Result<(u32, u32), Errors> {
    let numbers: Vec<u32> = line.chars().flat_map(|c| c.to_digit(10)).collect();
    match (numbers.first(), numbers.last()) {
        (Some(first), Some(last)) => Ok((*first, *last)),
        _ => Err(Errors::NoDigitFound),
    }
}

#[derive(Debug)]
enum Errors {
    NoDigitFound,
    FileError,
    RegexError,
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
