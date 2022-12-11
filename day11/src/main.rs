use itertools::Itertools;

fn sample_input() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![79, 98],
            operation: |x| x * 19,
            test_number: 23,
            throw_true: 2,
            throw_false: 3,
            inspected_items: 0,
        },
        Monkey {
            items: vec![54, 65, 75, 74],
            operation: |x| x + 6,
            test_number: 19,
            throw_true: 2,
            throw_false: 0,
            inspected_items: 0,
        },
        Monkey {
            items: vec![79, 60, 97],
            operation: |x| x * x,
            test_number: 13,
            throw_true: 1,
            throw_false: 3,
            inspected_items: 0,
        },
        Monkey {
            items: vec![74],
            operation: |x| x + 3,
            test_number: 17,
            throw_true: 0,
            throw_false: 1,
            inspected_items: 0,
        },
    ]
}

fn input() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![52, 78, 79, 63, 51, 94],
            operation: |x| x * 13,
            test_number: 5,
            throw_true: 1,
            throw_false: 6,
            inspected_items: 0,
        },
        Monkey {
            items: vec![77, 94, 70, 83, 53],
            operation: |x| x + 3,
            test_number: 7,
            throw_true: 5,
            throw_false: 3,
            inspected_items: 0,
        },
        Monkey {
            items: vec![98, 50, 76],
            operation: |x| x * x,
            test_number: 13,
            throw_true: 0,
            throw_false: 6,
            inspected_items: 0,
        },
        Monkey {
            items: vec![92, 91, 61, 75, 99, 63, 84, 69],
            operation: |x| x + 5,
            test_number: 11,
            throw_true: 5,
            throw_false: 7,
            inspected_items: 0,
        },
        Monkey {
            items: vec![51, 53, 83, 52],
            operation: |x| x + 7,
            test_number: 3,
            throw_true: 2,
            throw_false: 0,
            inspected_items: 0,
        },
        Monkey {
            items: vec![76, 76],
            operation: |x| x + 4,
            test_number: 2,
            throw_true: 4,
            throw_false: 7,
            inspected_items: 0,
        },
        Monkey {
            items: vec![75, 59, 93, 69, 76, 96, 65],
            operation: |x| x * 19,
            test_number: 17,
            throw_true: 1,
            throw_false: 3,
            inspected_items: 0,
        },
        Monkey {
            items: vec![89],
            operation: |x| x + 2,
            test_number: 19,
            throw_true: 2,
            throw_false: 4,
            inspected_items: 0,
        },
    ]
}

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: fn(usize) -> usize,
    test_number: usize,
    throw_true: usize,
    throw_false: usize,
    inspected_items: usize,
}

fn rounds(monkeys: &[Monkey], worry_divisor: usize, rounds: usize) -> usize {
    let mut monkeys = monkeys.to_vec();

    let mut modulus = 1;
    for monkey in monkeys.iter() {
        modulus *= monkey.test_number;
    }

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for item in monkeys[i].clone().items.iter() {
                monkeys[i].inspected_items += 1;

                let mut item = (monkeys[i].operation)(*item);

                item %= modulus;
                item /= worry_divisor;

                let throw = if item % monkeys[i].test_number == 0 {
                    monkeys[i].throw_true
                } else {
                    monkeys[i].throw_false
                };

                monkeys[throw].items.push(item);
            }
            monkeys[i].items.clear();
        }
    }
    monkeys
        .iter()
        .map(|x| x.inspected_items)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn main() {
    //let monkeys = sample_input();
    let monkeys = input();

    println!("Part 1: {}", rounds(&monkeys, 3, 20));
    println!("Part 2: {}", rounds(&monkeys, 1, 10000));
}
