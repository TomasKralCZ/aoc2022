use std::{fs, str::FromStr};

pub fn part1() {
    let res = fs::read_to_string(".//inputs/day1.txt")
        .unwrap()
        .as_str()
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|food| u64::from_str(food).unwrap_or(0))
                .sum::<u64>()
        })
        .max()
        .unwrap();

    println!("{}", res);
}

pub fn part2() {
    let res = fs::read_to_string(".//inputs/day1.txt")
        .unwrap()
        .as_str()
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|food| u64::from_str(food).unwrap_or(0))
                .sum::<u64>()
        })
        .fold([0, 0, 0], |mut acc, x| {
            if x > acc[0] {
                acc.copy_within(0..=1, 1);
                acc[0] = x;
            } else if x > acc[1] {
                acc[2] = acc[1];
                acc[1] = x;
            } else if x > acc[2] {
                acc[2] = x;
            }

            acc
        })
        .iter()
        .sum::<u64>();

    println!("{:?}", res);
}
