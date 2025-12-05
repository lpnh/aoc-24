use anyhow::Result;

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_02.txt");

#[cfg(feature = "part_1")]
fn solution_part_1(input: &str) -> Result<String> {
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
fn solution_part_2(input: &str) -> Result<String> {
    let report_lines: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|levels| levels.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut safe_report_count = 0;

    for line in report_lines {
        if is_valid_report(&line) {
            safe_report_count += 1;
            continue;
        }

        let mut safe = false;

        for skip_index in 0..line.len() {
            let mut skipped_report = line.clone();

            skipped_report.remove(skip_index);

            if is_valid_report(&skipped_report) {
                safe = true;
                break;
            }
        }

        if safe {
            safe_report_count += 1;
        }
    }

    Ok(safe_report_count.to_string())
}

fn is_valid_report(report: &[i32]) -> bool {
    let trend = report[1] > report[0];

    for i in 1..report.len() {
        if (report[i] > report[i - 1]) != trend {
            return false;
        }

        let diff = report[i] - report[i - 1];

        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
    }

    true
}

fn main() -> Result<()> {
    #[cfg(feature = "part_1")]
    println!("Part One: {}", solution_part_1(PUZZLE_INPUT)?);

    #[cfg(feature = "part_2")]
    println!("Part Two: {}", solution_part_2(PUZZLE_INPUT)?);

    Ok(())
}

#[cfg(feature = "part_1")]
#[test]
fn test_part_1() -> Result<()> {
    const EXAMPLE_INPUT_1: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    const EXAMPLE_OUTPUT_1: &str = "2";

    assert_eq!(solution_part_1(EXAMPLE_INPUT_1)?, EXAMPLE_OUTPUT_1);

    Ok(())
}

#[cfg(feature = "part_2")]
#[test]
fn test_part_2() -> Result<()> {
    const EXAMPLE_INPUT_2: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    const EXAMPLE_OUTPUT_2: &str = "4";

    assert_eq!(solution_part_2(EXAMPLE_INPUT_2)?, EXAMPLE_OUTPUT_2);

    Ok(())
}
