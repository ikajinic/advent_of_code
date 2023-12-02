use std::collections::HashMap;
use std::fs::read_to_string;
use regex::Regex;

pub fn solve_first() -> u32 {
    let file = read_to_string("input").unwrap();
    let mut sum = 0;
    let re = Regex::new(r"[a-zA-A]").unwrap();
    for line in file.lines() {
        let numbers = re.replace_all(line, "");
        if numbers.is_empty() {
            continue;
        }
        let first_number = numbers.chars().next().unwrap().to_digit(10).unwrap();
        let last_number = numbers.chars().last().unwrap().to_digit(10).unwrap();
        sum += first_number * 10 + last_number;
    }
    sum
}

pub fn solve_second() -> u32 {
    let file = read_to_string("input").unwrap();
    let mut sum = 0;
    let re = Regex::new(r"[a-zA-A]").unwrap();
    let mut numbers = HashMap::new();
    numbers.insert("one", "1");
    numbers.insert("two", "2");
    numbers.insert("three", "3");
    numbers.insert("four", "4");
    numbers.insert("five", "5");
    numbers.insert("six", "6");
    numbers.insert("seven", "7");
    numbers.insert("eight", "8");
    numbers.insert("nine", "9");
    for line in file.lines() {
        let mut buffer = String::new();
        let mut replaced = line.to_string();
        for char in line.chars() {
            buffer.push(char);
            if numbers.contains_key(buffer.as_str()) {
                replaced = replaced.replace(buffer.as_str(), format!("{}{}", numbers.get(buffer.as_str()).unwrap(), buffer.chars().last().unwrap()).as_str());
                buffer.clear();
                buffer.push(char);
                continue;
            }

            for (key, value) in numbers.iter() {
                if buffer.contains(key) {
                    replaced = replaced.replace(buffer.as_str(), format!("{}{}", buffer.replace(key, value).as_str(), key.chars().last().unwrap()).as_str());
                    buffer.clear();
                    buffer.push(key.chars().last().unwrap());
                    continue;
                }
            }
        }

        let numbers = re.replace_all(&replaced, "");
        if numbers.is_empty() {
            continue;
        }
        let first_number = numbers.chars().next().unwrap().to_digit(10).unwrap();
        let last_number = numbers.chars().last().unwrap().to_digit(10).unwrap();
        sum += first_number * 10 + last_number;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_1() {
        let result = solve_first();
        assert_eq!(result, 55108);
    }

    #[test]
    fn solve_2() {
        let result = solve_second();
        assert_eq!(result, 56324);
    }
}
