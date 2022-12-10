#[derive(Debug)]
struct Cpu {
    register_x: i32,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu { register_x: 1 }
    }

    fn run(&mut self, op: Op) {
        match op {
            Op::Noop => (),
            Op::Addx(x) => self.register_x += x,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Op {
    Noop,
    Addx(i32),
}

impl Op {
    fn parse(line: &str) -> Op {
        let parts: Vec<&str> = line.split(' ').collect();
        match parts[0] {
            "addx" => {
                let x = parts[1].parse::<i32>().unwrap();
                Op::Addx(x)
            }
            "noop" => Op::Noop,
            _ => panic!("Unknown instruction"),
        }
    }

    fn cycles(&self) -> usize {
        match self {
            Op::Noop => 1,
            Op::Addx(_) => 2,
        }
    }
}

fn part1(program: &Vec<Op>) -> i32 {
    let mut signal_strength = 0;

    let mut cpu = Cpu::new();

    let mut cycle = 0;
    let mut op_iter = program.iter();
    let mut current_op = op_iter.next().unwrap();
    let mut cycle_execution = current_op.cycles();

    while cycle < program.len() * 2 {
        cycle += 1;

        if cycle % 40 == 20 {
            signal_strength += cycle as i32 * cpu.register_x;
        }

        cycle_execution -= 1;
        if cycle_execution == 0 {
            cpu.run(*current_op);

            if let Some(op) = op_iter.next() {
                current_op = op;
                cycle_execution = op.cycles();
            } else {
                break;
            }
        }
    }

    signal_strength
}

fn part2(program: &Vec<Op>) {
    let mut screen = [false; 240];

    let mut cpu = Cpu::new();

    let mut cycle = 0;
    let mut op_iter = program.iter();
    let mut current_op = op_iter.next().unwrap();
    let mut cycle_execution = current_op.cycles();

    while cycle < program.len() * 2 {
        let mod_40 = (cycle % 40) as i32;
        if mod_40 == cpu.register_x - 1 || mod_40 == cpu.register_x || mod_40 == cpu.register_x + 1
        {
            screen[cycle] = true;
        }

        cycle += 1;

        cycle_execution -= 1;
        if cycle_execution == 0 {
            cpu.run(*current_op);

            if let Some(op) = op_iter.next() {
                current_op = op;
                cycle_execution = op.cycles();
            } else {
                break;
            }
        }
    }

    println!("Part 2: ");
    for (i, on) in screen.iter().enumerate() {
        if *on {
            print!("#");
        } else {
            print!(".");
        }

        if i % 40 == 39 {
            println!();
        }
    }
}

fn main() {
    let input = include_str!("input.txt");

    let program: Vec<Op> = input.lines().map(Op::parse).collect();

    println!("Part 1: {}", part1(&program));
    part2(&program);
}
