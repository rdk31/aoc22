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
            let mut visible = true;

            for up in 0..y {
                if map[up][x] >= map[y][x] {
                    visible = false;
                    break;
                }
            }

            if visible {
                count += 1;
                continue;
            }

            visible = true;
            for down in y + 1..map.len() {
                if map[down][x] >= map[y][x] {
                    visible = false;
                    break;
                }
            }

            if visible {
                count += 1;
                continue;
            }

            visible = true;
            for left in 0..x {
                if map[y][left] >= map[y][x] {
                    visible = false;
                    break;
                }
            }

            if visible {
                count += 1;
                continue;
            }

            visible = true;
            for right in x + 1..map[y].len() {
                if map[y][right] >= map[y][x] {
                    visible = false;
                    break;
                }
            }

            if visible {
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

            let mut up = 0;
            for h in map.iter().take(y).rev().map(|row| row[x]) {
                up += 1;
                if h >= height {
                    break;
                }
            }

            let mut down = 0;
            for h in map
                .iter()
                .rev()
                .take(map.len() - y - 1)
                .rev()
                .map(|row| row[x])
            {
                down += 1;
                if h >= height {
                    break;
                }
            }

            let mut left = 0;
            for h in map[y].iter().take(x).rev() {
                left += 1;
                if *h >= height {
                    break;
                }
            }

            let mut right = 0;
            for h in map[y].iter().rev().take(map[y].len() - x - 1).rev() {
                right += 1;
                if *h >= height {
                    break;
                }
            }

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
