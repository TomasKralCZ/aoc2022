// 1 rock
// 2 paper
// 3 scissors

// 0 lose
// 3 draw
// 6 win

#[rustfmt::skip]
const LUT_1: [[u8; 3]; 3] = [
  // 0, 1, 2
    [4, 8, 3], // 0 Op - rock
    [1, 5, 9], // 1 Op - paper
    [7, 2, 6], // 2 Op - scissors
  // Me - rock, paper, scissors
];

pub fn part1() -> u32 {
    include_str!("..//inputs/day2.txt")
        .as_bytes()
        .array_chunks::<4>()
        .map(|[op, _, me, _]| LUT_1[(op - b'A') as usize][(me - b'X') as usize] as u32)
        .sum::<u32>()
}

#[inline(never)]
pub fn part1_simd() -> u32 {
    unsafe {
        use std::arch::x86_64::*;

        let input = include_str!("..//inputs/day2.txt")
            .as_bytes()
            .array_chunks::<128>();

        let remainder = input
            .remainder()
            .array_chunks::<4>()
            .map(|[op, _, me, _]| LUT_1[(op - b'A') as usize][(me - b'X') as usize] as u32)
            .sum::<u32>();

        let simd_part = input
            .map(|arr| {
                // 4 256-bit regs
                // op, _, me, _
                // op, _, me, _
                // op, _, me, _
                // op, _, me, _
                // op, _, me, _
                // op, _, me, _
                // op, _, me, _
                // op, _, me, _
                // op, _, me, _
                let mut i0 = _mm256_loadu_si256(arr.as_ptr() as _);
                let mut i1 = _mm256_loadu_si256(arr[32..].as_ptr() as _);
                let mut i2 = _mm256_loadu_si256(arr[64..].as_ptr() as _);
                let mut i3 = _mm256_loadu_si256(arr[96..].as_ptr() as _);

                // Registers are loaded from the left ?
                #[rustfmt::skip]
                let shuffles_u8_table: [i8; 32] = [
                    14, 10, 6, 2,
                    12, 8, 4, 0,
                    -1, -1, -1, -1,
                    -1, -1, -1, -1,
                    14, 10, 6, 2,
                    12, 8, 4, 0,
                    -1, -1, -1, -1,
                    -1, -1, -1, -1,
                ];

                let shuffle_u8s = _mm256_loadu_si256(shuffles_u8_table.as_ptr() as _);

                // 4 256-bit regs
                // op, op, op, op
                // me, me, me, me
                // _,  _,  _,  _
                // _,  _,  _,  _
                // op, op, op, op
                // me, me, me, me
                // _,  _,  _,  _
                // _,  _,  _,  _
                i0 = _mm256_shuffle_epi8(i0, shuffle_u8s);
                i1 = _mm256_shuffle_epi8(i1, shuffle_u8s);
                i2 = _mm256_shuffle_epi8(i2, shuffle_u8s);
                i3 = _mm256_shuffle_epi8(i3, shuffle_u8s);

                #[rustfmt::skip]
                let shuffles_u32_table: [i32; 8] = [
                    5,
                    1,
                    7,
                    7,
                    4,
                    0,
                    7,
                    7,
                ];

                let shuffle_u32s = _mm256_loadu_si256(shuffles_u32_table.as_ptr() as _);

                // 4 256-bit regs
                // op, op, op, op
                // op, op, op, op
                // _,  _,  _,  _
                // _,  _,  _,  _
                // me, me, me, me
                // me, me, me, me
                // _,  _,  _,  _
                // _,  _,  _,  _
                i0 = _mm256_permutevar8x32_epi32(i0, shuffle_u32s);
                i1 = _mm256_permutevar8x32_epi32(i1, shuffle_u32s);
                i2 = _mm256_permutevar8x32_epi32(i2, shuffle_u32s);
                i3 = _mm256_permutevar8x32_epi32(i3, shuffle_u32s);

                let i0arr: [i64; 4] = std::mem::transmute(i0);
                let i1arr: [i64; 4] = std::mem::transmute(i1);
                let i2arr: [i64; 4] = std::mem::transmute(i2);
                let i3arr: [i64; 4] = std::mem::transmute(i3);

                // 1 256-bit reg
                // op, op, op, op
                // op, op, op, op
                // op, op, op, op
                // op, op, op, op
                // op, op, op, op
                // op, op, op, op
                // op, op, op, op
                // op, op, op, op
                let mut op = _mm256_set_epi64x(i0arr[0], i1arr[0], i2arr[0], i3arr[0]);

                // 1 256-bit reg
                // me, me, me, me
                // me, me, me, me
                // me, me, me, me
                // me, me, me, me
                // me, me, me, me
                // me, me, me, me
                // me, me, me, me
                // me, me, me, me
                let mut me = _mm256_set_epi64x(i0arr[2], i1arr[2], i2arr[2], i3arr[2]);

                op = _mm256_sub_epi8(op, _mm256_set1_epi8(b'A' as i8));
                me = _mm256_sub_epi8(me, _mm256_set1_epi8(b'X' as i8));

                // Can't multiply u8s...
                let indices = _mm256_add_epi8(op, op);
                let mut indices = _mm256_add_epi8(indices, op);
                indices = _mm256_add_epi8(indices, me);

                #[rustfmt::skip]
                let lut_old = _mm256_set_epi8(
                    0, 0, 0, 0, 0, 0, 0, 6,
                    2, 7, 9, 5, 1, 3, 8, 4,
                    0, 0, 0, 0, 0, 0, 0, 6,
                    2, 7, 9, 5, 1, 3, 8, 4,
                );

                let result = _mm256_shuffle_epi8(lut_old, indices);
                let result: [u8; 32] = std::mem::transmute(result);

                result.iter().map(|e| *e as u32).sum::<u32>()
            })
            .sum::<u32>();

        remainder + simd_part
    }
}

#[rustfmt::skip]
const LUT_2: [[u32; 3]; 3] = [
    [3, 4, 8], // Op - rock
    [1, 5, 9], // Op - paper
    [2, 6, 7], // Op - scissors
// Me - lose, draw, win
];

#[inline(never)]
pub fn part2() -> u32 {
    include_str!("..//inputs/day2.txt")
        .as_bytes()
        .array_chunks::<4>()
        .map(|[op, _, me, _]| LUT_2[(op - b'A') as usize][(me - b'X') as usize] as u32)
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::{part1, part1_simd, part2};
    use test::{black_box, Bencher};

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        black_box(b.iter(part1));
    }

    #[bench]
    fn bench_part1_simd(b: &mut Bencher) {
        black_box(b.iter(part1_simd));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        black_box(b.iter(part2));
    }
}
