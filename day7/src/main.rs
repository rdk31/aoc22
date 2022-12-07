use std::str::Lines;

#[derive(Debug)]
enum ObjectType {
    Dir(String, Vec<ObjectType>),
    File(usize),
}

fn parse_input(parent: &mut ObjectType, lines: &mut Lines) {
    while let Some(line) = lines.next() {
        if line.starts_with('$') {
            if line == "$ cd .." {
                return;
            } else if line.starts_with("$ cd ") {
                if let ObjectType::Dir(_, dir) = parent {
                    let dir_name = line.strip_prefix("$ cd ").unwrap();
                    let new_parent = dir
                        .iter_mut()
                        .find(|x| {
                            if let ObjectType::Dir(name, _) = x {
                                name == dir_name
                            } else {
                                false
                            }
                        })
                        .unwrap();

                    parse_input(new_parent, lines);
                }
            }
        } else if let ObjectType::Dir(_, dir) = parent {
            let object_info = line.split(' ').collect::<Vec<_>>();

            if let Ok(file_size) = object_info[0].parse::<usize>() {
                dir.push(ObjectType::File(file_size));
            } else {
                let dir_name = object_info[1];
                dir.push(ObjectType::Dir(dir_name.to_string(), Vec::new()));
            }
        }
    }
}

fn part1(root: &ObjectType, total_size: &mut usize) -> usize {
    match root {
        ObjectType::Dir(_, dir) => {
            let mut sum = 0;
            for d in dir.iter() {
                sum += part1(d, total_size);
            }
            if sum <= 100000 {
                *total_size += sum;
            }

            sum
        }
        ObjectType::File(size) => *size,
    }
}

fn total_size(root: &ObjectType) -> usize {
    match root {
        ObjectType::Dir(_, dir) => {
            let mut sum = 0;
            for d in dir.iter() {
                sum += total_size(d);
            }

            sum
        }
        ObjectType::File(size) => *size,
    }
}

fn part2(root: &ObjectType, total_min_size: &mut usize, min_to_free: usize) -> usize {
    match root {
        ObjectType::Dir(_, dir) => {
            let mut sum = 0;
            for d in dir.iter() {
                sum += part2(d, total_min_size, min_to_free);
            }
            if sum >= min_to_free && *total_min_size >= sum {
                *total_min_size = sum;
            }

            sum
        }
        ObjectType::File(size) => *size,
    }
}

fn main() {
    let input = include_str!("input.txt");

    let mut lines = input.lines();
    lines.next();

    let mut root = ObjectType::Dir("/".to_string(), Vec::new());
    parse_input(&mut root, &mut lines);
    //println!("{:?}", root);

    let mut part1_sum = 0;
    part1(&root, &mut part1_sum);
    println!("Part 1: {}", part1_sum);

    let min_to_free = total_size(&root) - 40000000;

    let mut part2_size = 70000000;
    part2(&root, &mut part2_size, min_to_free);
    println!("Part 2: {}", part2_size);
}
