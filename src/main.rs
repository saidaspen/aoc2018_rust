mod day01;
mod day02;
use std::fs;

fn main() {
    println!("{}", day02::part1(input("input_02.txt")));
    println!("{}", day02::part2(input("input_02.txt")));
}

pub fn input(file: &str) -> Vec<String> {
    fs::read_to_string(format!(
        "{}/{}",
        "/home/mendlock/projects/aoc2018_rust/inputs", file
    ))
    .expect("Unable to get file")
    .lines()
    .map(|x| x.clone().to_owned())
    .filter(|x| x.trim().len() > 0)
    .collect::<Vec<String>>()
}
