use take_until::TakeUntilExt;

type Map = Vec<Vec<u32>>;

fn parse_input(input: &str) -> Map {
    input
        .lines()
        .map(|line| line.chars().map(|num| num.to_digit(10).unwrap()).collect())
        .collect()
}

fn part1(map: &Map) -> usize {
    let mut count = map.len() * 2 + map[0].len() * 2 - 4;

    for y in 1..map.len() - 1 {
        for x in 1..map[y].len() - 1 {
            let height = map[y][x];

            let up = (0..y).map(|up| map[up][x]).all(|h| h < height);

            let down = (y + 1..map.len())
                .map(|down| map[down][x])
                .all(|h| h < height);

            let left = (0..x).map(|left| map[y][left]).all(|h| h < height);

            let right = (x + 1..map[y].len())
                .map(|right| map[y][right])
                .all(|h| h < height);

            if up || down || left || right {
                count += 1;
            }
        }
    }

    count
}

fn part2(map: &Map) -> usize {
    let mut max_score = 0;

    for y in 1..map.len() - 1 {
        for x in 1..map[y].len() - 1 {
            let height = map[y][x];

            let up = (0..y)
                .rev()
                .map(|up| map[up][x])
                .take_until(|&h| h >= height)
                .count();

            let down = (y + 1..map.len())
                .map(|down| map[down][x])
                .take_until(|&h| h >= height)
                .count();

            let left = (0..x)
                .rev()
                .map(|left| map[y][left])
                .take_until(|&h| h >= height)
                .count();

            let right = (x + 1..map[y].len())
                .map(|right| map[y][right])
                .take_until(|&h| h >= height)
                .count();

            let score = up * down * left * right;
            if max_score < score {
                max_score = score;
            }
        }
    }

    max_score
}

fn main() {
    let input = include_str!("input.txt");
    let map = parse_input(input);

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
}
