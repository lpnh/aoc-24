use anyhow::{Error, Result};

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_02.txt");

#[cfg(feature = "part_1")]
fn solve_part_1(input: &str) -> Result<String, Error> {
    let report_lines: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|levels| levels.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    // to consider everything safe by default is a terrible idea
    // i'm really sorry 😭
    let mut safe_report_count = report_lines.len();

    for line in &report_lines {
        let mut iter_line = line.iter();
        let mut cur = *iter_line.next().unwrap();

        let mut is_decreasing = cur > *line.get(1).unwrap_or(&cur);

        for &next in iter_line {
            if next == cur {
                safe_report_count -= 1;
                break;
            }

            if is_decreasing {
                if next < cur - 3 || next > cur {
                    safe_report_count -= 1;
                    break;
                }
            } else if next > cur + 3 || next < cur {
                safe_report_count -= 1;
                break;
            }

            is_decreasing = next < cur;
            cur = next;
        }
    }

    Ok(safe_report_count.to_string())
}

#[cfg(feature = "part_2")]
fn solve_part_2(input: &str) -> Result<String, Error> {
    let solution = input.lines().next().unwrap().replace("input", "answer");

    Ok(solution)
}

fn main() -> Result<(), Error> {
    println!("\nDay 02\n------");

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
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    const SAMPLE_ANSWER_1: &str = "2";

    assert_eq!(solve_part_1(SAMPLE_INPUT_1).unwrap(), SAMPLE_ANSWER_1);
}

#[cfg(feature = "part_2")]
#[test]
fn sample_part_2() {
    const SAMPLE_INPUT_2: &str = "\
sample part 2 input
goes here
like this
";
    const SAMPLE_ANSWER_2: &str = "sample part 2 answer";

    assert_eq!(solve_part_2(SAMPLE_INPUT_2).unwrap(), SAMPLE_ANSWER_2);
}
