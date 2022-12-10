use std::str::FromStr;

pub struct Handheld {
    pub cycle: u32,
    pub x: i32,
    pub signal_duty_cycle: u32,
    pub scanline_duty_cycle: u32,
    pub signal_strength: i32,
}

impl Handheld {
    pub fn new() -> Self {
        Self {
            cycle: 1,
            x: 1,
            signal_duty_cycle: 19,
            scanline_duty_cycle: 0,
            signal_strength: 0,
        }
    }

    pub fn clock<const DISPLAY: bool>(&mut self) {
        if self.signal_duty_cycle == 0 {
            self.signal_duty_cycle = 39;
            self.signal_strength = self.cycle as i32 * self.x
        } else {
            self.signal_duty_cycle -= 1;
            self.signal_strength = 0;
        };

        if DISPLAY {
            if (self.x - self.scanline_duty_cycle as i32).abs() <= 1 {
                print!("#");
            } else {
                print!(".");
            }

            if self.scanline_duty_cycle == 39 {
                self.scanline_duty_cycle = 0;
                print!("\n");
            } else {
                self.scanline_duty_cycle += 1;
            };
        }

        self.cycle += 1;
    }

    pub fn display(&mut self) {}
}

#[inline(never)]
pub fn part1() -> i32 {
    let input = include_bytes!("..//inputs/day10.txt");

    let mut handheld = Handheld::new();
    let mut sum = 0;

    for line in input.split(|b| *b == b'\n') {
        handheld.clock::<false>();
        sum += handheld.signal_strength;

        if line.len() != 4 {
            handheld.clock::<false>();
            sum += handheld.signal_strength;

            let to_add = &line[5..];
            let to_add = i32::from_str(std::str::from_utf8(to_add).unwrap()).unwrap();
            handheld.x += to_add;
        }

        if handheld.cycle > 220 {
            break;
        }
    }

    sum
}

#[inline(never)]
pub fn part2() {
    let input = include_bytes!("..//inputs/day10.txt");

    let mut handheld = Handheld::new();

    for line in input.split(|b| *b == b'\n') {
        handheld.clock::<true>();

        if line.len() != 4 {
            handheld.clock::<true>();

            let to_add = &line[5..];
            let to_add = i32::from_str(std::str::from_utf8(to_add).unwrap()).unwrap();
            handheld.x += to_add;
        }

        if handheld.cycle > 240 {
            break;
        }
    }
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
