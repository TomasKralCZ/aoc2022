use std::str::FromStr;

pub fn part1() -> u32 {
    include_bytes!("..//inputs/day4.txt")
        .split(|c| *c == b'\n')
        .map(|l| l.split(|c| *c == b',').map(|p| p.split(|c| *c == b'-')))
        .flatten()
        .flatten()
        .array_chunks::<4>()
        .fold(0u32, |acc, [s0, e0, s1, e1]| {
            let s0 = u32::from_str(std::str::from_utf8(s0).unwrap()).unwrap();
            let e0 = u32::from_str(std::str::from_utf8(e0).unwrap()).unwrap();
            let s1 = u32::from_str(std::str::from_utf8(s1).unwrap()).unwrap();
            let e1 = u32::from_str(std::str::from_utf8(e1).unwrap()).unwrap();

            let r0 = s0..=e0;
            let r1 = s1..=e1;

            // for part2 just switch to && instead of ||
            let increment =
                r0.contains(&s1) || r0.contains(&e1) || r1.contains(&s0) || r1.contains(&e0);

            acc + increment as u32
        })
}
