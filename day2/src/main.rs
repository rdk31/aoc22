use std::str::FromStr;

struct Round {
    theirs: Move,
    ours: Move,
}

impl Round {
    fn our_score(&self) -> usize {
        self.ours.outcome(&self.theirs).points() + self.ours.points()
    }

    fn part1(s: &str) -> Round {
        let mut parts = s.split_whitespace();
        let theirs = parts.next().unwrap().parse::<Move>().unwrap();
        let ours = parts.next().unwrap().parse::<Move>().unwrap();

        Round { theirs, ours }
    }

    fn part2(s: &str) -> Round {
        let mut parts = s.split_whitespace();
        let theirs = parts.next().unwrap().parse::<Move>().unwrap();
        let ours = parts
            .next()
            .unwrap()
            .parse::<Outcome>()
            .unwrap()
            .matching_move(theirs);

        Round { theirs, ours }
    }
}

#[derive(Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn beats(&self, other: &Move) -> bool {
        matches!(
            (self, other),
            (Move::Rock, Move::Scissors)
                | (Move::Paper, Move::Rock)
                | (Move::Scissors, Move::Paper)
        )
    }

    fn outcome(&self, other: &Move) -> Outcome {
        if self.beats(other) {
            Outcome::Win
        } else if other.beats(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }

    fn points(&self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    const ALL_MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

    fn winning_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|m| m.beats(&self))
            .expect("at least one move beats us")
    }

    fn losing_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|&m| self.beats(&m))
            .expect("at least one move beats us")
    }

    fn drawing_move(self) -> Self {
        self
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Move, ()> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(()),
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn points(&self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }

    fn matching_move(self, theirs: Move) -> Move {
        match self {
            Outcome::Win => theirs.winning_move(),
            Outcome::Draw => theirs.drawing_move(),
            Outcome::Loss => theirs.losing_move(),
        }
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Outcome, ()> {
        match s {
            "Z" => Ok(Outcome::Win),
            "Y" => Ok(Outcome::Draw),
            "X" => Ok(Outcome::Loss),
            _ => Err(()),
        }
    }
}

fn main() {
    let input = include_str!("input.txt");

    let part1: usize = input
        .lines()
        .map(Round::part1)
        .map(|round| round.our_score())
        .sum();

    let part2: usize = input
        .lines()
        .map(Round::part2)
        .map(|round| round.our_score())
        .sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
