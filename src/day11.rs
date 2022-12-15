use std::collections::VecDeque;

use itertools::{self, Itertools};

type MonkeyFn = Box<dyn Fn(usize) -> usize>;

struct Monkey {
    items: VecDeque<usize>,
    // new = fn(old)
    operation: MonkeyFn,
    // to_monkey = fn(value)
    test: MonkeyFn,

    inspect_count: usize,
}

impl Monkey {
    fn new(items: impl IntoIterator<Item = usize>, operation: MonkeyFn, test: MonkeyFn) -> Monkey {
        Monkey {
            items: VecDeque::from_iter(items),
            operation,
            test,
            inspect_count: 0,
        }
    }
}

struct State {
    monkeys: Vec<Monkey>,
}

impl State {
    fn build_sample() -> State {
        let monkeys = vec![
            // Monkey 0:
            //   Starting items: 79, 98
            //   Operation: new = old * 19
            //   Test: divisible by 23
            //     If true: throw to monkey 2
            //     If false: throw to monkey 3
            Monkey::new(
                [79, 98],
                Box::new(|old| old * 19),
                Box::new(|value| if value % 23 == 0 { 2 } else { 3 }),
            ),
            // Monkey 1:
            //   Starting items: 54, 65, 75, 74
            //   Operation: new = old + 6
            //   Test: divisible by 19
            //     If true: throw to monkey 2
            //     If false: throw to monkey 0
            Monkey::new(
                [54, 65, 75, 74],
                Box::new(|old| old + 6),
                Box::new(|value| if value % 19 == 0 { 2 } else { 0 }),
            ),
            // Monkey 2:
            //   Starting items: 79, 60, 97
            //   Operation: new = old * old
            //   Test: divisible by 13
            //     If true: throw to monkey 1
            //     If false: throw to monkey 3
            Monkey::new(
                [79, 60, 97],
                Box::new(|old| old * old),
                Box::new(|value| if value % 13 == 0 { 1 } else { 3 }),
            ),
            // Monkey 3:
            //   Starting items: 74
            //   Operation: new = old + 3
            //   Test: divisible by 17
            //     If true: throw to monkey 0
            //     If false: throw to monkey 1
            Monkey::new(
                [74],
                Box::new(|old| old + 3),
                Box::new(|value| if value % 17 == 0 { 0 } else { 1 }),
            ),
        ];
        State { monkeys }
    }

    fn build_input() -> State {
        let monkeys = vec![
            // Monkey 0:
            //   Starting items: 66, 71, 94
            //   Operation: new = old * 5
            //   Test: divisible by 3
            //     If true: throw to monkey 7
            //     If false: throw to monkey 4
            Monkey::new(
                [66, 71, 94],
                Box::new(|old| old * 5),
                Box::new(|value| if value % 3 == 0 { 7 } else { 4 }),
            ),
            // Monkey 1:
            //   Starting items: 70
            //   Operation: new = old + 6
            //   Test: divisible by 17
            //     If true: throw to monkey 3
            //     If false: throw to monkey 0
            Monkey::new(
                [70],
                Box::new(|old| old + 6),
                Box::new(|value| if value % 17 == 0 { 3 } else { 0 }),
            ),
            // Monkey 2:
            //   Starting items: 62, 68, 56, 65, 94, 78
            //   Operation: new = old + 5
            //   Test: divisible by 2
            //     If true: throw to monkey 3
            //     If false: throw to monkey 1
            Monkey::new(
                [62, 68, 56, 65, 94, 78],
                Box::new(|old| old + 5),
                Box::new(|value| if value % 2 == 0 { 3 } else { 1 }),
            ),
            // Monkey 3:
            //   Starting items: 89, 94, 94, 67
            //   Operation: new = old + 2
            //   Test: divisible by 19
            //     If true: throw to monkey 7
            //     If false: throw to monkey 0
            Monkey::new(
                [89, 94, 94, 67],
                Box::new(|old| old + 2),
                Box::new(|value| if value % 19 == 0 { 7 } else { 0 }),
            ),
            // Monkey 4:
            //   Starting items: 71, 61, 73, 65, 98, 98, 63
            //   Operation: new = old * 7
            //   Test: divisible by 11
            //     If true: throw to monkey 5
            //     If false: throw to monkey 6
            Monkey::new(
                [71, 61, 73, 65, 98, 98, 63],
                Box::new(|old| old * 7),
                Box::new(|value| if value % 11 == 0 { 5 } else { 6 }),
            ),
            // Monkey 5:
            //   Starting items: 55, 62, 68, 61, 60
            //   Operation: new = old + 7
            //   Test: divisible by 5
            //     If true: throw to monkey 2
            //     If false: throw to monkey 1
            Monkey::new(
                [55, 62, 68, 61, 60],
                Box::new(|old| old + 7),
                Box::new(|value| if value % 5 == 0 { 2 } else { 1 }),
            ),
            // Monkey 6:
            //   Starting items: 93, 91, 69, 64, 72, 89, 50, 71
            //   Operation: new = old + 1
            //   Test: divisible by 13
            //     If true: throw to monkey 5
            //     If false: throw to monkey 2
            Monkey::new(
                [93, 91, 69, 64, 72, 89, 50, 71],
                Box::new(|old| old + 1),
                Box::new(|value| if value % 13 == 0 { 5 } else { 2 }),
            ),
            // Monkey 7:
            //   Starting items: 76, 50
            //   Operation: new = old * old
            //   Test: divisible by 7
            //     If true: throw to monkey 4
            //     If false: throw to monkey 6
            Monkey::new(
                [76, 50],
                Box::new(|old| old * old),
                Box::new(|value| if value % 7 == 0 { 4 } else { 6 }),
            ),
        ];
        State { monkeys }
    }

    fn do_round(&mut self, do_div_by_three: bool) {
        for i in 0..self.monkeys.len() {
            while let Some(mut item) = self.monkeys[i].items.pop_front() {
                let (new_item, to_monkey) = {
                    let monkey = &mut self.monkeys[i];
                    monkey.inspect_count += 1;
                    // Inspect.
                    item = (monkey.operation)(item);
                    if do_div_by_three {
                        item /= 3;
                    }
                    (item, (monkey.test)(item))
                };
                self.monkeys[to_monkey].items.push_back(new_item);
            }
        }
    }
}

fn main() {
    {
        // Part 1:
        let mut state = State::build_input();
        for _ in 0..20 {
            state.do_round(true);
        }
        let monkey_business: usize = state
            .monkeys
            .iter()
            .map(|monkey| monkey.inspect_count)
            .sorted()
            .rev()
            .take(2)
            .product();
        dbg!(monkey_business);
    }
    // Part 2:
    // {
    //     let mut state = State::build_input();
    //     for _ in 0..20 {
    //         state.do_round(false);
    //     }
    //     let monkey_business: usize = state
    //         .monkeys
    //         .iter()
    //         .map(|monkey| monkey.inspect_count)
    //         .sorted()
    //         .rev()
    //         .take(2)
    //         .product();
    //     dbg!(monkey_business);
    // }
}
