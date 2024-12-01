pub use aoc_maker::*;
use regex::Regex;
use std::collections::HashMap;
use std::iter::Iterator;
use std::str::FromStr;
use std::vec::IntoIter;
use gcd::Gcd;
pub use itertools::Itertools;
pub use std::convert::identity;

#[macro_export]
macro_rules! aoc {
    (@common) => {
        fn get_input() -> Option<String> {
            let example = std::env::args().find(|arg| *arg == String::from("--example") || *arg == String::from("-e")).is_some();
            let day = std::path::Path::new(file!()).file_stem()?.to_str()?;
            let input_file = format!("./inputs/{day}.txt");
            let input = match example {
                true => String::from(EXAMPLE),
                false => std::fs::read_to_string(&input_file).ok()?,
            };
            Some(input.trim().replace("\r", "").to_string())
        }
    };
    () => {
        aoc!(@common);

        fn main() {
            let input = get_input().expect("Couldn't find input!");
            let now1 = std::time::SystemTime::now();
            let first = first(&input);
            println!("First: {first} (elapsed: {:?})", now1.elapsed().unwrap());
            let now2 = std::time::SystemTime::now();
            let second = second(&input);
            println!("Second: {second} (elapsed: {:?})", now2.elapsed().unwrap());
        }
    };
    (part1) => {
        aoc!(@common);

        fn main() {
            let input = get_input().expect("Couldn't find input!");
            let now1 = std::time::SystemTime::now();
            let first = first(&input);
            println!("First: {first} (elapsed: {:?})", now1.elapsed().unwrap());
        }
    };
    ($fun:ident) => {
        aoc!(@common);

        fn main() {
            let input = get_input().expect("Couldn't find input!");
            let now1 = std::time::SystemTime::now();
            let first = first($fun(&input));
            println!("First: {first} (elapsed: {:?})", now1.elapsed().unwrap());
            let now2 = std::time::SystemTime::now();
            let second = second($fun(&input));
            println!("Second: {second} (elapsed: {:?})", now2.elapsed().unwrap());
        }
    }
}

pub struct Numbers<T: FromStr + Copy = usize> {
    iter: IntoIter<T>,
    count: usize,
    numbers: Vec<T>,
}

impl<T: FromStr + Copy> Numbers<T> {
    pub fn parse(s: &String) -> Self {
        let re = Regex::new(r"\d+").expect("Weird regex");
        let numbers: Vec<T> = re.find_iter(&s).filter_map(|n| n.as_str().parse::<T>().ok()).collect();
        let count = numbers.len();
        let iter = numbers.to_vec().into_iter();
        Numbers {
            iter,
            count,
            numbers,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn vec(&self) -> Vec<T> {
        self.numbers.to_vec()
    }
}

impl<T: FromStr + Copy> Iterator for Numbers<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub fn gcd(a: usize, b: usize) -> usize {
    a.gcd_euclid(b)
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}
pub use Direction::*;

impl Direction {
    pub fn from_char(c: char) -> Self {
        match c {
            'L' => Left,
            'U' => Up,
            'R' => Right,
            'D' => Down,
            other => panic!("unrecognizable direction char {other}"),
        }
    }

    pub fn mirror(&self) -> Self {
        match self {
            Left => Right,
            Up => Down,
            Right => Left,
            Down => Up,
        }
    }

    pub fn traverse(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Left => (x - 1, y),
            Up => (x, y - 1),
            Right => (x + 1, y),
            Down => (x, y + 1),
        }
    }

    pub fn traverse_n(&self, point: (usize, usize), n: usize) -> (usize, usize) {
        (0..n).fold(point, |acc, _| self.traverse(acc))
    }

    pub fn itraverse(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match self {
            Left => (x - 1, y),
            Up => (x, y - 1),
            Right => (x + 1, y),
            Down => (x, y + 1),
        }
    }

    pub fn itraverse_n(&self, point: (isize, isize), n: isize) -> (isize, isize) {
        (0..n).fold(point, |acc, _| self.itraverse(acc))
    }
}

pub trait Grid {
    fn xmax(&self) -> usize;
    fn ymax(&self) -> usize;
}

impl<T: Copy> Grid for HashMap<(usize, usize), T> {
    fn xmax(&self) -> usize {
        let (xs, _): (Vec<_>, Vec<_>) = self.keys().cloned().unzip();
        xs.into_iter().max().expect("Grid is empty")
    }
    fn ymax(&self) -> usize {
        let (_, ys): (Vec<_>, Vec<_>) = self.keys().cloned().unzip();
        ys.into_iter().max().expect("Grid is empty")
    }
}

pub trait GetPointsable {
    type Value: Copy + PartialEq;

    fn points(&self, v: Self::Value) -> Vec<(usize, usize)>;
}

impl<T: Copy + PartialEq> GetPointsable for HashMap<(usize, usize), T> {
    type Value = T;

    fn points(&self, v: Self::Value) -> Vec<(usize, usize)> {
        self.iter().filter(|(_point, sv)| **sv == v).map(|(point, _c)| *point).collect()
    }
}

pub fn area(mut vertices: Vec<(usize, usize)>) -> usize {
    match vertices.first() {
        Some(&first_vertex) => {
            vertices.push(first_vertex);
            let payload: isize = vertices.into_iter().tuple_windows().map(|((x1, y1), (x2, y2))| ((x1 as isize, y1 as isize), (x2 as isize, y2 as isize))).map(|((x1, y1), (x2, y2))| (x1 * y2) - (x2 * y1) + x1.abs_diff(x2) as isize + y1.abs_diff(y2) as isize).sum();
            payload as usize / 2 + 1
        }
        None => {
            0
        }
    }
}

pub fn iarea(mut vertices: Vec<(isize, isize)>) -> usize {
    match vertices.first() {
        Some(&first_vertex) => {
            vertices.push(first_vertex);
            let payload: isize = vertices.into_iter().tuple_windows().map(|((x1, y1), (x2, y2))| (x1 * y2) - (x2 * y1) + x1.abs_diff(x2) as isize + y1.abs_diff(y2) as isize).sum();
            payload as usize / 2 + 1
        }
        None => {
            0
        }
    }
}