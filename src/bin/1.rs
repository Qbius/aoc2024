use aoc::*;
use counter::Counter;

fn first((mut left, mut right): (Vec<usize>, Vec<usize>)) -> usize {
    left.sort();
    right.sort();
    left.into_iter().zip(right.into_iter()).map(|(a, b)| a.abs_diff(b)).sum()
}

fn second((left, right): (Vec<usize>, Vec<usize>)) -> usize {
    let counts: Counter<_> = right.into_iter().collect();
    left.into_iter().map(|a| counts.get(&a).map(|&b| a * b).unwrap_or(0)).sum()
}

const EXAMPLE: &str = r#"
3   4
4   3
2   5
1   3
3   9
3   3"#;

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    input.split("\n").map(|line| {
        match line.split("   ").collect_tuple() {
            Some((one, two)) => {
                (one.parse::<usize>().unwrap(), two.parse::<usize>().unwrap())
            },
            _ => {
                panic!("weird input");
            }
        }
    }).unzip()
}

aoc!(parse);