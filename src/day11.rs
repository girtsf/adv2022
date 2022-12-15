use std::collections::VecDeque;

use itertools::{self, Itertools};

type MonkeyFn = Box<dyn Fn(usize) -> usize>;

struct Monkey {
    items: VecDeque<usize>,
    // new = fn(old)
    operation: MonkeyFn,
    divisor: usize,
    monkey_if_true: usize,
    monkey_if_false: usize,

    inspect_count: usize,
}

impl Monkey {
    fn new(
        items: impl IntoIterator<Item = usize>,
        operation: MonkeyFn,
        divisor: usize,
        monkey_if_true: usize,
        monkey_if_false: usize,
    ) -> Monkey {
        Monkey {
            items: VecDeque::from_iter(items),
            operation,
            divisor,
            monkey_if_true,
            monkey_if_false,
            inspect_count: 0,
        }
    }
}

struct State {
    monkeys: Vec<Monkey>,
    divisor_product: usize,
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
            Monkey::new([79, 98], Box::new(|old| old * 19), 23, 2, 3),
            // Monkey 1:
            //   Starting items: 54, 65, 75, 74
            //   Operation: new = old + 6
            //   Test: divisible by 19
            //     If true: throw to monkey 2
            //     If false: throw to monkey 0
            Monkey::new([54, 65, 75, 74], Box::new(|old| old + 6), 19, 2, 0),
            // Monkey 2:
            //   Starting items: 79, 60, 97
            //   Operation: new = old * old
            //   Test: divisible by 13
            //     If true: throw to monkey 1
            //     If false: throw to monkey 3
            Monkey::new([79, 60, 97], Box::new(|old| old * old), 13, 1, 3),
            // Monkey 3:
            //   Starting items: 74
            //   Operation: new = old + 3
            //   Test: divisible by 17
            //     If true: throw to monkey 0
            //     If false: throw to monkey 1
            Monkey::new([74], Box::new(|old| old + 3), 17, 0, 1),
        ];
        let divisor_product = monkeys.iter().map(|m| m.divisor).product();
        State {
            monkeys,
            divisor_product,
        }
    }

    fn build_input() -> State {
        let monkeys = vec![
            // Monkey 0:
            //   Starting items: 66, 71, 94
            //   Operation: new = old * 5
            //   Test: divisible by 3
            //     If true: throw to monkey 7
            //     If false: throw to monkey 4
            Monkey::new([66, 71, 94], Box::new(|old| old * 5), 3, 7, 4),
            // Monkey 1:
            //   Starting items: 70
            //   Operation: new = old + 6
            //   Test: divisible by 17
            //     If true: throw to monkey 3
            //     If false: throw to monkey 0
            Monkey::new([70], Box::new(|old| old + 6), 17, 3, 0),
            // Monkey 2:
            //   Starting items: 62, 68, 56, 65, 94, 78
            //   Operation: new = old + 5
            //   Test: divisible by 2
            //     If true: throw to monkey 3
            //     If false: throw to monkey 1
            Monkey::new([62, 68, 56, 65, 94, 78], Box::new(|old| old + 5), 2, 3, 1),
            // Monkey 3:
            //   Starting items: 89, 94, 94, 67
            //   Operation: new = old + 2
            //   Test: divisible by 19
            //     If true: throw to monkey 7
            //     If false: throw to monkey 0
            Monkey::new([89, 94, 94, 67], Box::new(|old| old + 2), 19, 7, 0),
            // Monkey 4:
            //   Starting items: 71, 61, 73, 65, 98, 98, 63
            //   Operation: new = old * 7
            //   Test: divisible by 11
            //     If true: throw to monkey 5
            //     If false: throw to monkey 6
            Monkey::new(
                [71, 61, 73, 65, 98, 98, 63],
                Box::new(|old| old * 7),
                11,
                5,
                6,
            ),
            // Monkey 5:
            //   Starting items: 55, 62, 68, 61, 60
            //   Operation: new = old + 7
            //   Test: divisible by 5
            //     If true: throw to monkey 2
            //     If false: throw to monkey 1
            Monkey::new([55, 62, 68, 61, 60], Box::new(|old| old + 7), 5, 2, 1),
            // Monkey 6:
            //   Starting items: 93, 91, 69, 64, 72, 89, 50, 71
            //   Operation: new = old + 1
            //   Test: divisible by 13
            //     If true: throw to monkey 5
            //     If false: throw to monkey 2
            Monkey::new(
                [93, 91, 69, 64, 72, 89, 50, 71],
                Box::new(|old| old + 1),
                13,
                5,
                2,
            ),
            // Monkey 7:
            //   Starting items: 76, 50
            //   Operation: new = old * old
            //   Test: divisible by 7
            //     If true: throw to monkey 4
            //     If false: throw to monkey 6
            Monkey::new([76, 50], Box::new(|old| old * old), 7, 4, 6),
        ];
        let divisor_product = monkeys.iter().map(|m| m.divisor).product();
        State {
            monkeys,
            divisor_product,
        }
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
                    // GCD would be more efficient, but I was lazy.
                    item %= self.divisor_product;
                    let to_monkey = if item % monkey.divisor == 0 {
                        monkey.monkey_if_true
                    } else {
                        monkey.monkey_if_false
                    };

                    (item, to_monkey)
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
    {
        let mut state = State::build_input();
        for _ in 0..10000 {
            state.do_round(false);
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
}
