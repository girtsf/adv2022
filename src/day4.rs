mod lib;

fn parse_range(s: &str) -> (u32, u32) {
    let (start, end) = s.split_once("-").unwrap();
    (start.parse::<u32>().unwrap(), end.parse::<u32>().unwrap())
}

fn parse_line(line: &str) -> ((u32, u32), (u32, u32)) {
    let (a, b) = line.split_once(",").unwrap();
    (parse_range(a), parse_range(b))
}

fn main() {
    let input = lib::read_input();
    let ranges = input.lines().map(parse_line).collect::<Vec<_>>();
    let part1 = ranges
        .iter()
        .map(|((a_start, a_end), (b_start, b_end))| {
            if (a_start <= b_start && a_end >= b_end) || (b_start <= a_start && b_end >= a_end) {
                1
            } else {
                0
            }
        })
        .sum::<u32>();
    dbg!(part1);

    let part2 = ranges
        .iter()
        .map(|((a_start, a_end), (b_start, b_end))| {
            if b_start <= a_end && a_start <= b_end {
                1
            } else {
                0
            }
        })
        .sum::<u32>();
    dbg!(part2);
}
