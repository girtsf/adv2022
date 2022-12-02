mod lib;

fn part1_score(opponent: char, you: char) -> u32 {
    // A: Rock,     B: Paper,     C: Scissors
    // X: Rock (1), Y: Paper (2), Z: Scissors (3)
    // Lost: 0, Draw: 3, Win: 6
    match (opponent, you) {
        ('A', 'X') => 1 + 3,
        ('A', 'Y') => 2 + 6,
        ('A', 'Z') => 3 + 0,
        ('B', 'X') => 1 + 0,
        ('B', 'Y') => 2 + 3,
        ('B', 'Z') => 3 + 6,
        ('C', 'X') => 1 + 6,
        ('C', 'Y') => 2 + 0,
        ('C', 'Z') => 3 + 3,
        _ => panic!("unexpected input"),
    }
}

fn part2_score(opponent: char, you: char) -> u32 {
    // A: Rock, B: Paper, C: Scissors
    // X: Lose, Y: Draw,  Z: Win
    // Lost: 0, Draw: 3, Win: 6
    match (opponent, you) {
        ('A', 'X') => 0 + 3,
        ('A', 'Y') => 3 + 1,
        ('A', 'Z') => 6 + 2,
        ('B', 'X') => 0 + 1,
        ('B', 'Y') => 3 + 2,
        ('B', 'Z') => 6 + 3,
        ('C', 'X') => 0 + 2,
        ('C', 'Y') => 3 + 3,
        ('C', 'Z') => 6 + 1,
        _ => panic!("unexpected input"),
    }
}

fn parse_line(line: &str) -> (char, char) {
    let mut chars = line.chars();
    (chars.next().unwrap(), chars.skip(1).next().unwrap())
}

fn part1_score_line(line: &str) -> u32 {
    let (opponent, you) = parse_line(line);
    part1_score(opponent, you)
}

fn part2_score_line(line: &str) -> u32 {
    let (opponent, you) = parse_line(line);
    part2_score(opponent, you)
}

fn main() {
    let input = lib::read_input();
    let part1 = input
        .lines()
        .map(|line| part1_score_line(line))
        .sum::<u32>();
    dbg!(part1);

    let part2 = input
        .lines()
        .map(|line| part2_score_line(line))
        .sum::<u32>();
    dbg!(part2);
}
