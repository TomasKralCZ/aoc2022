use std::str::FromStr;

pub struct Dir<'n> {
    name: &'n str,
    size: u64,
    sub_entries: Vec<Entry<'n>>,
}

pub struct File<'n> {
    name: &'n str,
    size: u64,
}

pub enum Entry<'n> {
    Dir(Dir<'n>),
    File(File<'n>),
}

impl Entry<'_> {
    pub fn calculate_size(&mut self) -> u64 {
        match self {
            Entry::Dir(dir) => dir.calculate_size(),
            Entry::File(file) => file.size,
        }
    }
}

impl Dir<'_> {
    pub fn calculate_size(&mut self) -> u64 {
        for entry in &mut self.sub_entries {
            self.size += entry.calculate_size();
        }

        self.size
    }

    pub fn get_dir_sum_smaller<const N: u64>(&self) -> u64 {
        /* match self.sub_entries {
            None => 0,
            Some(sub_entries) => {
                let mut total = 0;

                if self.size < N {
                    total += self.size;
                }

                for entry in sub_entries {
                    total += entry.get_dir_sum_smaller::<N>();
                }

                total
            }
        } */
        todo!()
    }

    /* pub fn find_dir_to_delete<const T: u64, const N: u64>(&self) -> u64 {
        let mut res = u64::MAX;
        let min_delete_size = self.size - (T - N);

        self.find_dir_to_delete_inner(&mut res, min_delete_size);

        res
    }

    fn find_dir_to_delete_inner(&self, res: &mut u64, min_delete_size: u64) {
        if let Some(sub_entries) = self.sub_entries.as_ref() {
            if self.size >= min_delete_size && self.size < *res {
                *res = self.size;
            }

            for entry in sub_entries {
                entry.find_dir_to_delete_inner(res, min_delete_size);
            }
        }
    } */
}

pub fn parse_dir<'i>(input: &mut &'i str, name: &'i str) -> Dir<'i> {
    let mut root = Dir {
        name,
        size: 0,
        sub_entries: vec![],
    };

    while let Some((line, rest)) = input.split_once('\n') {
        *input = rest;

        let mut segments = line.split(' ').skip(1);
        let cmd = segments.next().unwrap();

        if cmd == "cd" {
            let arg = segments.next().unwrap();

            if arg == ".." {
                return root;
            } else {
                let name = line.split(' ').skip(2).next().unwrap();
                let sub_dir = parse_dir(input, name);
                root.sub_entries.push(Entry::Dir(sub_dir));
            }
        }

        if cmd == "ls" {
            while !input.starts_with("$") && input.len() > 0 {
                let (line, rest) = input.split_once('\n').unwrap();
                *input = rest;

                let mut segments = line.split(' ');
                let s1 = segments.next().unwrap();
                let s2 = segments.next().unwrap();

                if s1 != "dir" {
                    let size = u64::from_str(s1).unwrap();
                    let new_file = File { name: s2, size };

                    root.sub_entries.push(Entry::File(new_file));
                }
            }
        }
    }

    root
}

#[inline(never)]
pub fn part1() -> u64 {
    let mut input = &include_str!("..//inputs/day7.txt")[7..];
    let mut root = parse_dir(&mut input, "/");
    root.calculate_size();
    root.get_dir_sum_smaller::<100000>()
}

#[inline(never)]
pub fn part2() -> u64 {
    let mut input = &include_str!("..//inputs/day7.txt")[7..];
    let mut root = parse_dir(&mut input, "/");
    /* root.calculate_size();
    root.find_dir_to_delete::<70000000, 30000000>() */

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
