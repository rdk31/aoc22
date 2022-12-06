use std::collections::HashSet;

fn part(input: &str, step: usize) -> Option<usize> {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(step)
        .map(|w| -> HashSet<char> { HashSet::from_iter(w.iter().cloned()) })
        .enumerate()
        .find(|(_, w)| w.len() == step)
        .map(|(i, _)| i + step)
}

fn main() {
    let input = include_str!("input.txt");

    let part1 = part(input, 4);
    let part2 = part(input, 14);

    println!("part1: {:?}", part1);
    println!("part2: {:?}", part2);
}
