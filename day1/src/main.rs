use std::cmp::Reverse;

use itertools::Itertools;

fn main() {
    let s: Vec<u64> = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|mut it| (&mut it).map_while(|x| x).sum1::<u64>())
        .map(Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .collect();

    println!("Part 1: {}", s.first().unwrap());
    println!("Part 2: {}", s.iter().sum::<u64>());
}
