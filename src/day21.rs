use std::{cmp::Ordering, collections::HashMap};

use adv2022::read_input;

#[derive(Debug)]
struct Math {
    lhs: String,
    op: char,
    rhs: String,
}

impl Math {
    fn parse(input: &str) -> Math {
        assert_eq!(input.len(), 11);
        let lhs: String = input.chars().take(4).collect();
        let op = input.chars().nth(5).unwrap();
        let rhs: String = input.chars().skip(7).collect();
        Math { lhs, op, rhs }
    }
}

#[derive(Debug)]
struct Monkey {
    value: Option<isize>,
    job: Option<Math>,
}

impl Monkey {
    fn parse(input: &str) -> Monkey {
        if let Some(number) = input.parse::<isize>().ok() {
            Monkey {
                value: Some(number),
                job: None,
            }
        } else {
            Monkey {
                value: None,
                job: Some(Math::parse(input)),
            }
        }
    }
}

#[derive(Debug)]
struct Maths {
    monkeys: HashMap<String, Monkey>,
}

impl Maths {
    fn parse(input: &str) -> Maths {
        let monkeys = input
            .lines()
            .map(|line| {
                let (name, rest) = line.trim().split_once(": ").unwrap();
                (name.to_owned(), Monkey::parse(rest))
            })
            .collect();
        Maths { monkeys }
    }

    /// Returns calculated number and whether any divisions resulted in rounding.
    fn calculate(&self, who: &str) -> (isize, bool) {
        let m = self.monkeys.get(who).unwrap();
        if let Some(value) = m.value {
            return (value, false);
        }
        let math = m.job.as_ref().unwrap();
        let lhs = math.lhs.clone();
        let op = math.op.clone();
        let rhs = math.rhs.clone();
        let (lhs_value, lhs_rounding) = self.calculate(&lhs);
        let (rhs_value, rhs_rounding) = self.calculate(&rhs);
        let value = match op {
            '+' => lhs_value + rhs_value,
            '-' => lhs_value - rhs_value,
            '/' => {
                if lhs_value % rhs_value != 0 {
                    return (lhs_value / rhs_value, true);
                }
                lhs_value / rhs_value
            }
            '*' => lhs_value * rhs_value,
            _ => panic!("invalid op"),
        };
        (value, lhs_rounding || rhs_rounding)
    }

    fn compare_sides(&self, who: &str) -> (Ordering, bool) {
        let m = self.monkeys.get(who).unwrap();
        let math = m.job.as_ref().unwrap();
        let lhs = math.lhs.clone();
        let rhs = math.rhs.clone();
        let (lhs_value, lhs_rounding) = self.calculate(&lhs);
        let (rhs_value, rhs_rounding) = self.calculate(&rhs);
        dbg!(lhs_value, rhs_value, lhs_rounding, rhs_rounding);
        dbg!((lhs_value.cmp(&rhs_value), lhs_rounding || rhs_rounding))
    }
}

fn main() {
    let mut maths = Maths::parse(&read_input());
    dbg!(&maths);
    let part1 = maths.calculate("root");
    dbg!(part1);

    // Part2: the electric cheeezeroo
    //
    // From manual poking, I knew that the value was positive and on lhs, and I manually tweaked
    // the search direction. This could be made smarter, but /shrug.
    let mut low = 0isize;
    let mut high = 2isize.pow(50);
    while low <= high {
        let middle = (high + low) / 2;
        dbg!(low, high, middle);
        maths.monkeys.get_mut("humn").unwrap().value = Some(middle);
        let (ord, _) = maths.compare_sides("root");
        match ord {
            Ordering::Equal => break,
            Ordering::Less => {
                high = middle - 1;
            }
            Ordering::Greater => {
                low = middle + 1;
            }
        }
    }
    for humn in low..=high {
        dbg!(&humn);
        maths.monkeys.get_mut("humn").unwrap().value = Some(humn);
        match maths.compare_sides("root") {
            (Ordering::Equal, false) => break,
            _ => {}
        }
    }
}
