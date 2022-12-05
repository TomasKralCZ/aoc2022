#![feature(iter_array_chunks)]
#![feature(array_chunks)]
#![feature(test)]
extern crate test;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

fn main() {
    day5::part1();
    day5::part2();
}
