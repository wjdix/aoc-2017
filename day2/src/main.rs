#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![feature(match_default_bindings)]

#[macro_use]
extern crate itertools;

use std::{env, fs, io};
use std::io::BufRead;

fn row_checksum(input: &[i32]) -> i32 {
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();
    max - min
}

fn row_checksum2(input: &[i32]) -> i32 {
    let (x, y) = iproduct!(input, input)
        .find(|(&i, &j)| i != j && i % j == 0)
        .unwrap();
    x / y
}

fn checksum(input: &Vec<Vec<i32>>) -> i32 {
    input.iter().fold(0, |acc, vec| acc + row_checksum(vec))
}

fn checksum2(input: &Vec<Vec<i32>>) -> i32 {
    input.iter().fold(0, |acc, vec| acc + row_checksum2(vec))
}

fn parse_line(input: String) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|i| i32::from_str_radix(i, 10).unwrap())
        .collect()
}


fn main() {
    let filename = env::args().nth(1).expect("missing filename");
    let file = fs::File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    let mut input = vec![];
    for line in reader.lines() {
        match line {
            Ok(line) => input.push(parse_line(line)),
            Err(_) => println!("Error"),
        }
    }
    println!("Result: {}", checksum(&input));
    println!("Result2: {}", checksum2(&input));

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        let input = vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]];
        assert_eq!(18, checksum(&input));
    }

    #[test]
    fn test_row_checksum() {
        assert_eq!(8, row_checksum(&vec![5, 1, 9, 5]));
        assert_eq!(4, row_checksum(&vec![7, 5, 3]));
        assert_eq!(6, row_checksum(&vec![2, 4, 6, 8]));
    }

    #[test]
    fn test_checksum2() {
        let input = vec![vec![5, 9, 2, 8], vec![9, 4, 7, 3], vec![3, 8, 6, 5]];
        assert_eq!(9, checksum2(&input));
    }

    #[test]
    fn test_row_checksum2() {
        assert_eq!(4, row_checksum2(&vec![5, 9, 2, 8]));
        assert_eq!(3, row_checksum2(&vec![9, 4, 7, 3]));
        assert_eq!(2, row_checksum2(&vec![3, 8, 6, 5]));
    }
}
