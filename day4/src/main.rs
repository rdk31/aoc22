use std::ops::RangeInclusive;

use itertools::Itertools;

trait InclusiveRangeExt {
    fn contains_range(&self, other: &Self) -> bool;

    fn contains_or_is_contained(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }

    fn overlaps(&self, other: &Self) -> bool;

    fn overlaps_or_is_overlapped(&self, other: &Self) -> bool {
        self.overlaps(other) || other.overlaps(self)
    }
}

impl<T> InclusiveRangeExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

fn main() {
    let input = include_str!("input.txt");

    let pairs: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|range| {
                    range
                        .split('-')
                        .map(|n| n.parse().unwrap())
                        .collect_tuple::<(u32, u32)>()
                        .map(|(start, end)| start..=end)
                        .unwrap()
                })
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .collect();

    let part1 = pairs
        .iter()
        .filter(|(a, b)| a.contains_or_is_contained(b))
        .count();

    let part2 = pairs
        .iter()
        .filter(|(a, b)| a.overlaps_or_is_overlapped(b))
        .count();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
