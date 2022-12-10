
#[inline(never)]
pub fn part1() -> u32 {
    let input = include_bytes!("..//inputs/day.txt");



    0
}

#[inline(never)]
pub fn part2() -> u32 {
    let input = include_bytes!("..//inputs/day.txt");

    0
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use test::{black_box, Bencher};

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        black_box(b.iter(part1));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        black_box(b.iter(part2));
    }
}
