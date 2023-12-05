use std::collections::{HashMap, HashSet};
use std::error::Error;

pub fn solve_first(input: &str) -> Result<u32, Box<dyn Error>> {
    Ok(input
        .lines()
        .filter_map(|line| line.split_once(":").map(|(_, values)| values))
        .filter_map(|line| {
            line.split_once("|")
                .map(|(left, right)| (left.trim(), right.trim()))
        })
        .map(|(left, right)| {
            (
                left.split_whitespace()
                    .filter_map(|number| number.parse::<u32>().ok())
                    .collect::<HashSet<u32>>(),
                right
                    .split_whitespace()
                    .filter_map(|number| number.parse::<u32>().ok())
                    .collect::<Vec<u32>>(),
            )
        })
        .map(|(left, right)| right.iter().filter(|n| left.contains(n)).count() as u32)
        .map(|n| if n == 0 { 0 } else { 2_u32.pow(n - 1) })
        .sum())
}

pub fn solve_second(input: &str) -> Result<u32, Box<dyn Error>> {
    let values = input
        .lines()
        .filter_map(|line| line.split_once(":").map(|(id, values)| (id, values)))
        .filter_map(|(id, line)| {
            id.replace("Card ", "")
                .trim()
                .parse::<u32>()
                .ok()
                .map(|i| (i, line))
        })
        .filter_map(|(id, line)| {
            line.split_once("|")
                .map(|(left, right)| (id, left.trim(), right.trim()))
        })
        .map(|(id, left, right)| {
            let required_numbers = left
                .split_whitespace()
                .filter_map(|number| number.parse::<u32>().ok())
                .collect::<HashSet<u32>>();
            let pulled_numbers = right
                .split_whitespace()
                .filter_map(|number| number.parse::<u32>().ok())
                .collect::<Vec<u32>>();
            let counts = pulled_numbers
                .iter()
                .filter(|n| required_numbers.contains(n))
                .count() as u32;

            (
                id,
                if counts == 0 {
                    Vec::new()
                } else {
                    (id + 1..id + counts + 1).collect::<Vec<u32>>()
                },
            )
        })
        .collect::<HashMap<u32, Vec<u32>>>();

    let mut keys = values.keys().map(|k| *k).collect::<Vec<u32>>();
    keys.sort();

    let mut vec: [u32; 221] = [0; 221];
    for x in keys {
        let current = values.get(&x).unwrap().clone();
        let copied = vec[x as usize] != 0;
        vec[x as usize] += 1;
        for y in current {
            vec[y as usize] += if copied { vec[x as usize] } else { 1 };
        }
    }

    Ok(vec.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_1_test() {
        let input = include_str!("../test_input");
        let start = std::time::Instant::now();
        let result = solve_first(input);
        match result {
            Err(e) => panic!("{:?}", e),
            Ok(result) => assert_eq!(result, 13),
        }
        println!("solve_1: {:?}", start.elapsed().as_micros());
    }

    #[test]
    fn solve_1() {
        let input = include_str!("../input");
        let start = std::time::Instant::now();
        let result = solve_first(input);
        match result {
            Err(e) => panic!("{:?}", e),
            Ok(result) => assert_eq!(result, 28750),
        }
        println!("solve_1: {:?}", start.elapsed().as_micros());
    }

    #[test]
    fn solve_2_test() {
        let input = include_str!("../test_input");
        let start = std::time::Instant::now();
        let result = solve_second(input);
        match result {
            Err(e) => panic!("{:?}", e),
            Ok(result) => assert_eq!(result, 30),
        }
        println!("solve_1: {:?}", start.elapsed().as_micros());
    }

    #[test]
    fn solve_2() {
        let input = include_str!("../input");
        let start = std::time::Instant::now();
        let result = solve_second(input);
        match result {
            Err(e) => panic!("{:?}", e),
            Ok(result) => assert_eq!(result, 10212704),
        }
        println!("solve_1: {:?}", start.elapsed().as_micros());
    }
}
