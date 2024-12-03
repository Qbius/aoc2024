use aoc::*;
use regex::Regex;

const EXAMPLE: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn first(input: &str) -> usize {
    mul_sum(input)
}

fn second(input: &str) -> usize {
    input.split("do()").map(|part| mul_sum(part.split("don't()").nth(0).unwrap())).sum()
}

fn mul_sum(s: &str) -> usize {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(s).map(|m| m[1].parse::<usize>().unwrap() * m[2].parse::<usize>().unwrap()).sum()
}

aoc!();