// 7908
#[inline(never)]
pub fn part1() -> u32 {
    let input = include_bytes!("..//inputs/day3.txt");

    let mut letters: [u8; 256] = [0; 256];

    let mut sum = 0u32;

    for line in input.split(|b| *b == b'\n') {
        let (first, second) = line.split_at(line.len() / 2);

        for l in first {
            letters[*l as usize] = 1;
        }

        for l in second {
            letters[*l as usize] |= 2;
        }

        let mut line_sum = 0u32;

        for i in b'A'..=b'Z' {
            let priority = i - 38;

            if letters[i as usize] == 3 {
                line_sum += priority as u32;
            }
        }

        for i in b'a'..=b'z' {
            let priority = i - 96;

            if letters[i as usize] == 3 {
                line_sum += priority as u32;
            }
        }

        sum += line_sum;
        letters = [0; 256];
    }

    sum
}

pub fn prio3(b: u8) -> u8 {
    (b & !32) - b'A' + 1 + 26 * (b >= b'a') as u8
}

// 2838
#[inline(never)]
pub fn part2() -> u32 {
    let input = include_bytes!("..//inputs/day3.txt");

    let mut letters1 = 0u64;
    let mut letters2 = 0u64;
    let mut letters3 = 0u64;

    let mut sum = 0u32;

    for group in input.split(|b| *b == b'\n').array_chunks::<3>() {
        for l in group[0] {
            letters1 |= 1 << (*l - b'A');
        }

        for l in group[1] {
            letters2 |= 1 << (*l - b'A');
        }

        for l in group[2] {
            letters3 |= 1 << (*l - b'A');
        }

        let same = letters1 & letters2 & letters3;

        let mut priority = same.trailing_zeros();
        if priority > 25 {
            priority -= 31;
        } else {
            priority += 27;
        }

        sum += priority;

        letters1 = 0;
        letters2 = 0;
        letters3 = 0;
    }

    sum
}

#[inline(never)]
pub fn part2_lol() -> u32 {
    let input = include_bytes!("..//inputs/day3.txt");

    let mut letters: [[u8; 4]; 256] = [[0; 4]; 256];

    let mut sum = 0u32;

    'groups: for group in input.split(|b| *b == b'\n').array_chunks::<3>() {
        for l in group[0] {
            letters[*l as usize][0] = 0xFF;
        }

        for l in group[1] {
            letters[*l as usize][1] = 0xFF;
        }

        for l in group[2] {
            letters[*l as usize][2] = 0xFF;
        }

        for i in b'A'..=b'Z' {
            let priority = i - 38;

            let as_u32: u32 = bytemuck::cast(letters[i as usize]);

            if as_u32 == 0x00FFFFFF {
                sum += priority as u32;
                letters = [[0; 4]; 256];
                continue 'groups;
            }
        }

        for i in b'a'..=b'z' {
            let priority = i - 96;

            let as_u32: u32 = bytemuck::cast(letters[i as usize]);

            if as_u32 == 0x00FFFFFF {
                sum += priority as u32;
                letters = [[0; 4]; 256];

                continue 'groups;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, part2_lol};
    use test::{black_box, Bencher};

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        black_box(b.iter(part1));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        black_box(b.iter(part2));
    }

    #[bench]
    fn bench_part2_lol(b: &mut Bencher) {
        black_box(b.iter(part2_lol));
    }
}
