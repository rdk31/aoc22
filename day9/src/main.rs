use std::{collections::HashSet, iter::repeat};

#[derive(Copy, Clone, PartialEq, Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn step(&mut self, m: Move) {
        match m {
            Move::Up => self.y += 1,
            Move::Down => self.y -= 1,
            Move::Left => self.x -= 1,
            Move::Right => self.x += 1,
        }
    }

    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn is_touching(&self, other: &Self) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn follow(&mut self, other: &Self) {
        self.x += (other.x - self.x).signum();
        self.y += (other.y - self.y).signum();
    }
}

fn parse_input(input: &str) -> Vec<Move> {
    input
        .lines()
        .flat_map(|line| {
            let s: Vec<&str> = line.split(' ').collect();
            let r = s[1].parse().unwrap();
            match s[0] {
                "U" => repeat(Move::Up),
                "D" => repeat(Move::Down),
                "L" => repeat(Move::Left),
                "R" => repeat(Move::Right),
                _ => panic!("Invalid input"),
            }
            .take(r)
        })
        .collect()
}

fn simulate(moves: &[Move], knots: usize) -> usize {
    let mut visited = HashSet::new();

    let mut knots = vec![Knot::new(); knots];

    for m in moves {
        knots[0].step(*m);

        let mut old_knot = knots[0].clone();

        for knot in knots.iter_mut().skip(1) {
            if !knot.is_touching(&old_knot) {
                knot.follow(&old_knot);
            }
            old_knot = knot.clone();
        }

        visited.insert(knots.last().unwrap().position());
    }

    visited.len()
}

fn main() {
    let input = include_str!("input.txt");

    let moves = parse_input(input);

    println!("Part 1: {}", simulate(&moves, 2));
    println!("Part 2: {}", simulate(&moves, 10));
}
