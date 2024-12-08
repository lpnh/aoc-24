use anyhow::{Error, Result};
use regex::Regex;

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_03.txt");

#[cfg(feature = "part_1")]
fn solve_part_1(input: &str) -> Result<String, Error> {
    let memory = input;
    let re = Regex::new(r"mul\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)").unwrap();

    let mut sum = 0;

    for cap in re.captures_iter(memory) {
        let n1 = &cap[1].parse::<u32>().unwrap();
        let n2 = &cap[2].parse::<u32>().unwrap();

        sum += n1 * n2;
    }

    Ok(sum.to_string())
}

#[cfg(feature = "part_2")]
fn solve_part_2(input: &str) -> Result<String, Error> {
    let re = Regex::new(r"(?s)don't\(.*?do\(\)|don't\(.*").unwrap();
    let memory = re.replace_all(input, "").to_string();

    let re_2 = Regex::new(r"mul\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)").unwrap();

    let mut sum = 0;

    for cap in re_2.captures_iter(&memory) {
        let n1 = &cap[1].parse::<u32>().unwrap();
        let n2 = &cap[2].parse::<u32>().unwrap();

        sum += n1 * n2;
    }

    Ok(sum.to_string())
}

fn main() -> Result<(), Error> {
    println!("\nDay 03\n------");

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
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";
    const SAMPLE_ANSWER_1: &str = "161";

    assert_eq!(solve_part_1(SAMPLE_INPUT_1).unwrap(), SAMPLE_ANSWER_1);
}

#[cfg(feature = "part_2")]
#[test]
fn sample_part_2() {
    const SAMPLE_INPUT_2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";
    const SAMPLE_ANSWER_2: &str = "48";

    assert_eq!(solve_part_2(SAMPLE_INPUT_2).unwrap(), SAMPLE_ANSWER_2);
}
