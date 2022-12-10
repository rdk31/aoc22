use std::{collections::HashSet, str::FromStr};

struct Rucksack {
    contents: String,
}

impl Rucksack {
    fn find_in_both_compartments(&self) -> Vec<char> {
        let mut contains = HashSet::new();

        let middle_pos = self.contents.len() / 2;
        let first_compartment = &self.contents[..middle_pos];
        let second_compartment = &self.contents[middle_pos..];

        for c in first_compartment.chars() {
            if second_compartment.contains(c) {
                contains.insert(c);
            }
        }

        contains.into_iter().collect()
    }

    fn find_in_all_three_rucksacks(&self, second: &Self, third: &Self) -> Vec<char> {
        let mut contains = HashSet::new();

        for c in self.contents.chars() {
            if second.contents.contains(c) && third.contents.contains(c) {
                contains.insert(c);
            }
        }

        contains.into_iter().collect()
    }
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rucksack {
            contents: s.to_string(),
        })
    }
}

fn priority(c: char) -> usize {
    if c.is_uppercase() {
        c as usize - 'A' as usize + 27
    } else {
        c as usize - 'a' as usize + 1
    }
}

fn main() {
    let input = include_str!("input.txt");

    let rucksacks: Vec<Rucksack> = input.lines().map(|line| line.parse().unwrap()).collect();

    let part1 = rucksacks
        .iter()
        .flat_map(|rucksack| rucksack.find_in_both_compartments())
        .map(priority)
        .sum::<usize>();

    let part2 = rucksacks
        .chunks(3)
        .flat_map(|rucksacks| {
            rucksacks[0].find_in_all_three_rucksacks(&rucksacks[1], &rucksacks[2])
        })
        .map(priority)
        .sum::<usize>();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
