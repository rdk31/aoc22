use std::collections::VecDeque;

type Crates = Vec<VecDeque<char>>;

#[derive(Debug)]
struct Operation {
    from: usize,
    to: usize,
    count: usize,
}

fn parse(input: &str) -> (Crates, Vec<Operation>) {
    let mut num_of_cols = None;
    let mut describing_operations = false;

    let mut crates: Crates = Vec::new();
    let mut operations: Vec<Operation> = Vec::new();

    for line in input.lines() {
        if num_of_cols.is_none() {
            num_of_cols = Some(line.len() / 4 + 1);
            crates.resize(num_of_cols.unwrap(), VecDeque::new());
        } else if line.is_empty() {
            describing_operations = true;
            continue;
        }

        if !describing_operations {
            for (i, s) in crates.iter_mut().enumerate() {
                let c = line.chars().nth(i * 4 + 1).unwrap();
                if !c.is_numeric() && !c.is_whitespace() {
                    s.push_back(c);
                }
            }
        } else {
            let cmds: Vec<usize> = line
                .split(' ')
                .filter_map(|c| c.parse::<usize>().ok())
                .collect();

            operations.push(Operation {
                from: cmds[1] - 1,
                to: cmds[2] - 1,
                count: cmds[0],
            });
        }
    }

    (crates, operations)
}

fn part1(crates: &Crates, operations: &Vec<Operation>) -> String {
    let mut crates = crates.clone();
    for operation in operations {
        for _ in 0..operation.count {
            let c = crates[operation.from].pop_front().unwrap();
            crates[operation.to].push_front(c);
        }
    }

    crates.iter().map(|s| s.front().unwrap()).collect()
}

fn part2(crates: &Crates, operations: &Vec<Operation>) -> String {
    let mut crates = crates.clone();
    for operation in operations {
        let moved: Vec<char> = crates[operation.from]
            .drain(0..operation.count)
            .rev()
            .collect();

        for c in moved {
            crates[operation.to].push_front(c);
        }
    }

    crates.iter().map(|s| s.front().unwrap()).collect()
}

fn main() {
    let input = include_str!("input.txt");

    let (crates, operations) = parse(input);

    println!("part1: {}", part1(&crates, &operations));
    println!("part2: {}", part2(&crates, &operations));
}
