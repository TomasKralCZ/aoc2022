#![feature(iter_array_chunks)]
#![feature(byte_slice_trim_ascii)]
#![feature(array_chunks)]
#![feature(test)]
extern crate test;

pub mod day1;
pub mod day10;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day9;

fn main() {
    println!("{}", day10::part1());
    day10::part2();
}
