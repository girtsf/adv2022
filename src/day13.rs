mod lib;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Number(usize),
    List(Vec<Packet>),
}

impl Packet {
    fn parse(mut line: &str) -> (Packet, &str) {
        // dbg!(line);
        assert_eq!(line.chars().next().unwrap(), '[');
        line = &line[1..];

        let mut list = Vec::<Packet>::new();
        // If we are currently parsing a number.
        let mut current_number: Option<usize> = None;

        loop {
            let first = line.chars().next().unwrap();
            // dbg!(line, first);

            match first {
                ']' => {
                    // grab number if it exists
                    if let Some(x) = current_number {
                        list.push(Packet::Number(x));
                    }
                    return (Packet::List(list), &line[1..]);
                }
                '[' => {
                    assert!(current_number.is_none());
                    let sub_list;
                    (sub_list, line) = Packet::parse(line);
                    list.push(sub_list);
                    continue;
                }
                ',' => {
                    if let Some(x) = current_number {
                        list.push(Packet::Number(x));
                        current_number = None;
                    }
                }
                '0'..='9' => {
                    current_number =
                        Some(current_number.unwrap_or(0) * 10 + (first as usize - '0' as usize));
                }
                _ => panic!(),
            }
            line = &line[1..];
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Number(n_a), Packet::Number(n_b)) => Some(n_a.cmp(n_b)),
            (Packet::Number(n_a), Packet::List(_)) => {
                Packet::List(vec![Packet::Number(n_a.clone())]).partial_cmp(other)
            }
            (Packet::List(_), Packet::Number(n_b)) => {
                self.partial_cmp(&Packet::List(vec![Packet::Number(n_b.clone())]))
            }
            (Packet::List(l_a), Packet::List(l_b)) => {
                // XXX: is there some kind of zip(l_a.iter(), l_b.iter()) that yields (Option(Item1),
                // Option(Item2)) pairs?
                let mut iter_a = l_a.iter();
                let mut iter_b = l_b.iter();
                loop {
                    match (iter_a.next(), iter_b.next()) {
                        // Both run out of outputs.
                        (None, None) => return Some(std::cmp::Ordering::Equal),
                        // Left runs out of items first.
                        (None, _) => return Some(std::cmp::Ordering::Less),
                        // Right runs out of items first.
                        (Some(_), None) => return Some(std::cmp::Ordering::Greater),
                        // Both have elements, compare those.
                        (Some(el_a), Some(el_b)) => {
                            let order = el_a.partial_cmp(el_b);
                            if !matches!(order, Some(std::cmp::Ordering::Equal)) {
                                return order;
                            }
                            // Otherwise, keep going.
                        }
                    }
                }
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_pair(lines: &str) -> (Packet, Packet) {
    let (l1, l2) = lines.split_once("\n").unwrap();
    (Packet::parse(l1).0, Packet::parse(l2).0)
}

fn main() {
    let input = lib::read_input();
    let part1 = input
        .split("\n\n")
        .enumerate()
        .map(|(i, lines)| {
            let (p1, p2) = parse_pair(lines);
            match p1.partial_cmp(&p2) {
                Some(std::cmp::Ordering::Less) => i + 1,
                Some(std::cmp::Ordering::Greater) => 0,
                Some(std::cmp::Ordering::Equal) | None => panic!("should not happen"),
            }
        })
        .sum::<usize>();
    dbg!(&part1);

    {
        // Part 2.
        let mut packets: Vec<_> = input
            .lines()
            .filter_map(|line| {
                if line.is_empty() {
                    None
                } else {
                    Some(Packet::parse(line))
                }
            })
            .collect();
        let divider2 = Packet::parse(&"[[2]]");
        let divider6 = Packet::parse(&"[[6]]");
        packets.push(divider2.clone());
        packets.push(divider6.clone());
        packets.sort();
        // dbg!(&packets);
        let divider2_pos = packets.iter().position(|x| x.eq(&divider2)).unwrap() + 1;
        let divider6_pos = packets.iter().position(|x| x.eq(&divider6)).unwrap() + 1;
        // dbg!(&divider2_pos, &divider6_pos);
        let decoder_key = divider2_pos * divider6_pos;
        dbg!(&decoder_key);
    }
}
