#[rustfmt::skip]
pub fn part1() -> u32 {
    let input = include_bytes!("..//inputs/day6.txt");

    if input.len() < 4 {
        panic!();
    }

    for i in 3..input.len() {
        let b3 = input[i];
        let b2 = input[i - 1];
        let b1 = input[i - 2];
        let b0 = input[i - 3];

        // I'm assuming the source doesn't containt a null character
        let as_u32_1 = u64::from_ne_bytes([b0, b0, b0, b1, b1, b2, 1, 1]);
        let as_u32_2 = u64::from_ne_bytes([b1, b2, b3, b2, b3, b3, 0, 0]);

        let mask = as_u32_1 ^ as_u32_2;

        // https://jameshfisher.com/2017/01/24/bitwise-check-for-zero-byte/
        let no_zero_byte = ((mask - 0x0101010101010101u64) & !mask & 0x8080808080808080u64) == 0;

        if no_zero_byte {
            return i as u32 + 1;
        }
    }

    0
}

#[inline(never)]
pub fn part1_naive() -> u32 {
    let input = include_bytes!("..//inputs/day6.txt");

    if input.len() < 4 {
        panic!();
    }

    for i in 3..input.len() {
        let b3 = input[i];
        let b2 = input[i - 1];
        let b1 = input[i - 2];
        let b0 = input[i - 3];

        if b0 != b1 && b0 != b2 && b0 != b3 && b1 != b2 && b1 != b3 && b2 != b3 {
            return i as u32 + 1;
        }
    }

    0
}

#[inline(never)]
pub fn part2() -> u32 {
    let input = include_bytes!("..//inputs/day6.txt");

    let mut mask: u64 = 0;
    for i in 0..14 {
        mask ^= 1 << (input[i] - b'a');
    }

    if mask.count_ones() == 14 {
        return 1;
    }

    for i in 14..input.len() {
        mask ^= 1 << (input[i - 14] - b'a');
        mask ^= 1 << (input[i] - b'a');

        if mask.count_ones() == 14 {
            return i as u32 + 1;
        }
    }

    0
}

#[inline(always)]
fn find_header_idx(buf: &[u8]) -> usize {
    let mut mask: u64 = (1 << (buf[0] - b'a'))
        ^ (1 << (buf[1] - b'a'))
        ^ (1 << (buf[2] - b'a'))
        ^ (1 << (buf[3] - b'a'));
    if mask.count_ones() == 4 {
        return 1;
    }

    for i in 4..buf.len() {
        mask ^= 1 << (buf[i - 4] - b'a');
        mask ^= 1 << (buf[i] - b'a');
        if mask.count_ones() == 4 {
            return i + 1;
        }
    }

    unreachable!()
}

#[inline(never)]
pub fn part_1_nerd() -> usize {
    let bytes = include_bytes!("..//inputs/day6.txt");

    find_header_idx(bytes)
}

#[cfg(test)]
mod tests {
    use super::{part1, part1_naive, part2, part_1_nerd};
    use test::{black_box, Bencher};

    #[bench]
    fn bench_day6_part1(b: &mut Bencher) {
        black_box(b.iter(part1));
    }

    #[bench]
    fn bench_day6_part1_naive(b: &mut Bencher) {
        black_box(b.iter(part1_naive));
    }

    #[bench]
    fn bench_day6_part1_nerd(b: &mut Bencher) {
        black_box(b.iter(part_1_nerd));
    }

    #[bench]
    fn bench_day6_part2(b: &mut Bencher) {
        black_box(b.iter(part2));
    }
}
