use anyhow::{Error, Result};
use std::collections::HashMap;

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_01.txt");

#[cfg(feature = "part_1")]
fn solve_part_1(input: &str) -> Result<String, Error> {
    let mut left_ids: Vec<i32> = vec![];
    let mut right_ids: Vec<i32> = vec![];

    input.lines().for_each(|line| {
        let mut pairs = line.split_whitespace();
        left_ids.push(pairs.next().unwrap().parse::<i32>().unwrap());
        right_ids.push(pairs.next().unwrap().parse::<i32>().unwrap());
    });

    left_ids.sort();
    right_ids.sort();

    let solution = left_ids
        .iter()
        .zip(right_ids.iter())
        .map(|(left, right)| i32::abs(left - right))
        .sum::<i32>()
        .to_string();

    Ok(solution)
}

#[cfg(feature = "part_2")]
fn solve_part_2(input: &str) -> Result<String, Error> {
    let mut left_ids: Vec<i32> = vec![];
    let mut right_ids: Vec<i32> = vec![];

    input.lines().for_each(|line| {
        let mut pairs = line.split_whitespace();
        left_ids.push(pairs.next().unwrap().parse::<i32>().unwrap());
        right_ids.push(pairs.next().unwrap().parse::<i32>().unwrap());
    });

    let mut right_list_counts = HashMap::new();

    right_ids.iter().for_each(|&number| {
        *right_list_counts.entry(number).or_insert(0) += 1;
    });

    let solution = left_ids
        .iter()
        .map(|n| n * right_list_counts.get(n).unwrap_or(&0))
        .sum::<i32>()
        .to_string();

    Ok(solution)
}

fn main() -> Result<(), Error> {
    println!("\nDay 01\n------");

    #[cfg(feature = "part_1")]
    {
        let answer_part_1 = solve_part_1(PUZZLE_INPUT)?;
        println!("Part One: {answer_part_1}");
    }

    #[cfg(feature = "part_2")]
    {
        let answer_part_2 = solve_part_2(PUZZLE_INPUT)?;
        println!("Part Two: {answer_part_2}");
    }

    println!();

    Ok(())
}

#[cfg(feature = "part_1")]
#[test]
fn sample_part_1() {
    const SAMPLE_INPUT_1: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";
    const SAMPLE_ANSWER_1: &str = "11";

    assert_eq!(solve_part_1(SAMPLE_INPUT_1).unwrap(), SAMPLE_ANSWER_1);
}

#[cfg(feature = "part_2")]
#[test]
fn sample_part_2() {
    const SAMPLE_INPUT_2: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";
    const SAMPLE_ANSWER_2: &str = "31";

    assert_eq!(solve_part_2(SAMPLE_INPUT_2).unwrap(), SAMPLE_ANSWER_2);
}
