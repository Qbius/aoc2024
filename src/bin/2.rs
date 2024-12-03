use aoc::*;
use counter::Counter;

fn first(reports: Vec<Vec<isize>>) -> usize {
    reports.into_iter().filter(|report| {
        let diffs = report[1..].iter().zip(report.iter()).map(|(&a, &b)| a - b).collect_vec();
        diffs.iter().all(|&d| d < 0 && d > -4) || diffs.iter().all(|&d| d > 0 && d < 4)
    }).count()
}

fn second(reports: Vec<Vec<isize>>) -> usize {
    reports.into_iter().filter(|report| {
        let diffs = report[1..].iter().zip(report.iter()).map(|(&a, &b)| a - b).collect_vec();
        let lower_count = diffs.iter().filter(|&&d| d > -1 || d < -3).count();
        let upper_count = diffs.iter().filter(|&&d| d < 1 || d > 3).count();
        lower_count <= 1 || upper_count <= 1
    }).count()
}

const EXAMPLE: &str = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

#[lines]
fn parse(input: Vec<String>) -> Vec<Vec<isize>> {
    input.into_iter().map(|line| line.split(" ").map(|n| n.parse::<isize>().unwrap()).collect()).collect()
}

aoc!(parse);