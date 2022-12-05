pub fn part1() {
    let input = include_str!("../inputs/day5.txt");
    let (in_stacks, instr) = input.split_at(input.find("\n\n").unwrap());

    let first_line = in_stacks.split_at(in_stacks.find('\n').unwrap()).0;
    let num_stacks = (first_line.len() - 3) / 4 + 1;
    let mut stacks: Vec<Vec<char>> = vec![vec![]; num_stacks];

    for l in in_stacks
        .split('\n')
        .rev()
        .skip(1)
        .map(|l| l.as_bytes().iter().skip(1).step_by(4))
    {
        for (i, c) in l.enumerate().filter(|(_, c)| **c != b' ') {
            stacks[i].push(*c as char);
        }
    }

    for [num_move, from, to] in instr
        .trim()
        .split('\n')
        .flat_map(|l| {
            l.split(' ')
                .skip(1)
                .step_by(2)
                .map(|num| num.parse::<u32>().unwrap())
        })
        .array_chunks::<3>()
    {
        for _ in 0..num_move {
            let craté = stacks[(from - 1) as usize].pop().unwrap();
            stacks[(to - 1) as usize].push(craté);
        }
    }

    for s in stacks {
        print!("{}", s.last().unwrap());
    }
    println!()
}

// PWPWHGFZS
pub fn part2() {
    let input = include_str!("../inputs/day5.txt");
    let (in_stacks, instr) = input.split_at(input.find("\n\n").unwrap());

    let first_line = in_stacks.split_at(in_stacks.find('\n').unwrap()).0;
    let num_stacks = (first_line.len() - 3) / 4 + 1;
    let mut stacks: Vec<Vec<char>> = vec![vec![]; num_stacks];

    in_stacks
        .split('\n')
        .rev()
        .skip(1)
        .map(|l| {
            l.as_bytes()
                .iter()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| **c != b' ')
                .for_each(|(i, c)| stacks[i].push(*c as char))
        })
        .for_each(drop);

    for [num_move, from, to] in instr
        .trim()
        .split('\n')
        .flat_map(|l| {
            l.split(' ')
                .skip(1)
                .step_by(2)
                .map(|num| num.parse::<u32>().unwrap())
        })
        .array_chunks::<3>()
    {
        let src_i = stacks[from as usize - 1].len() - 1;

        for i in (0..num_move).rev() {
            let craté = stacks[from as usize - 1][src_i - i as usize];
            stacks[to as usize - 1].push(craté);
        }

        for _ in 0..num_move {
            stacks[from as usize - 1].pop();
        }
    }

    for s in stacks {
        print!("{}", s.last().unwrap());
    }
    println!()
}
