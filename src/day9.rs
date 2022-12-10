use std::collections::HashSet;
use std::str::FromStr;

use glam::IVec2;

/*
H - IVec2(35, -50)
T - IVec2(36, -50)
6236
*/
#[inline(never)]
pub fn part1() -> usize {
    let input = include_str!("..//inputs/day9.txt");

    let mut head_pos = IVec2::ZERO;
    let mut tail_pos = IVec2::ZERO;

    let mut positions = HashSet::<IVec2>::with_capacity(8192);

    for [dir, len] in input
        .trim_end()
        .split('\n')
        .flat_map(|l| l.split(' '))
        .array_chunks::<2>()
    {
        let len = i32::from_str(len).unwrap();
        for _ in 0..len {
            let delta = match dir.as_bytes()[0] {
                b'U' => IVec2::new(0, 1),
                b'R' => IVec2::new(1, 0),
                b'D' => IVec2::new(0, -1),
                b'L' => IVec2::new(-1, 0),
                _ => unreachable!(),
            };

            head_pos += delta;

            let mut diff = head_pos - tail_pos;
            if diff.x == 0 || diff.y == 0 {
                // Move horizontally
                tail_pos += diff / 2;
            } else if diff.x.abs() != diff.y.abs() {
                // Move diagonally
                if diff.x.abs() == 2 {
                    diff.x /= 2;
                } else if diff.y.abs() == 2 {
                    diff.y /= 2;
                }

                tail_pos += diff;
            }

            positions.insert(tail_pos);
        }
    }

    positions.len()
}

// 2449
#[inline(never)]
pub fn part2() -> usize {
    let input = include_str!("..//inputs/day9.txt");

    let mut knots = [IVec2::ZERO; 10];

    let mut positions = HashSet::<IVec2>::with_capacity(8192);

    for [dir, len] in input
        .trim_end()
        .split('\n')
        .flat_map(|l| l.split(' '))
        .array_chunks::<2>()
    {
        let len = i32::from_str(len).unwrap();
        for _ in 0..len {
            let delta = match dir.as_bytes()[0] {
                b'U' => IVec2::new(0, 1),
                b'R' => IVec2::new(1, 0),
                b'D' => IVec2::new(0, -1),
                b'L' => IVec2::new(-1, 0),
                _ => unreachable!(),
            };

            knots[0] += delta;

            for k in 1..10 {
                let mut diff = knots[k - 1] - knots[k];
                if diff.x.abs() == 1 && diff.y.abs() == 1 {
                    break;
                }

                if diff.x == 0 || diff.y == 0 {
                    // Move horizontally
                    diff /= 2;
                    if diff.x == 0 && diff.y == 0 {
                        break;
                    }
                    knots[k] += diff;
                } else {
                    // Move diagonally
                    if diff.x.abs() == 2 {
                        diff.x /= 2;
                    }

                    if diff.y.abs() == 2 {
                        diff.y /= 2;
                    }

                    knots[k] += diff;
                }

                if k == 9 {
                    positions.insert(knots[k]);
                }
            }
        }
    }

    positions.len()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use test::{black_box, Bencher};

    #[bench]
    fn bench_day9_part1(b: &mut Bencher) {
        black_box(b.iter(part1));
    }

    #[bench]
    fn bench_day9_part2(b: &mut Bencher) {
        black_box(b.iter(part2));
    }
}
