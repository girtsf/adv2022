mod lib;

type CrateStack = Vec<Vec<char>>;

// [T]             [P]     [J]
// [F]     [S]     [T]     [R]     [B]
// [V]     [M] [H] [S]     [F]     [R]
// [Z]     [P] [Q] [B]     [S] [W] [P]
// [C]     [Q] [R] [D] [Z] [N] [H] [Q]
// [W] [B] [T] [F] [L] [T] [M] [F] [T]
// [S] [R] [Z] [V] [G] [R] [Q] [N] [Z]
// [Q] [Q] [B] [D] [J] [W] [H] [R] [J]
//  1   2   3   4   5   6   7   8   9
fn parse_stack(text: &str) -> CrateStack {
    // Ignore the last line and assume it's 1, 2, 3...
    let lines: Vec<&str> = text.lines().rev().skip(1).collect();
    let count = (lines[0].len() + 1) / 4;

    let mut out = Vec::new();
    out.resize_with(count, || Vec::new());
    for l in lines.iter() {
        for i in 0..count {
            let c = l.chars().nth(4 * i + 1).unwrap();
            if c == ' ' {
                continue;
            }
            out[i].push(c);
        }
    }
    out
}

// move 3 from 8 to 2
fn parse_move(line: &str) -> (usize, usize, usize) {
    let items: Vec<&str> = line.split(" ").collect();
    let count = items[1].parse::<usize>().unwrap();
    let from = items[3].parse::<usize>().unwrap();
    let to = items[5].parse::<usize>().unwrap();
    (count, from, to)
}

fn parse_and_apply_moves_part1(crate_stack: &mut CrateStack, moves_text: &str) {
    for move_line in moves_text.lines() {
        let (mut count, from, to) = parse_move(move_line);
        while count > 0 {
            let elem = crate_stack[from - 1].pop();
            match elem {
                Some(c) => crate_stack[to - 1].push(c),
                None => break,
            }
            count -= 1;
        }
    }
}

fn parse_and_apply_moves_part2(crate_stack: &mut CrateStack, moves_text: &str) {
    for move_line in moves_text.lines() {
        let (count, from, to) = parse_move(move_line);
        let from_pos = crate_stack[from - 1].len() - count;
        let to_move: Vec<char> = crate_stack[from - 1].drain(from_pos..).collect();
        crate_stack[to - 1].extend(to_move);
    }
}

fn answer(crate_stack: &CrateStack) -> String {
    let mut out = String::new();
    for stack in crate_stack.iter() {
        out.push(*stack.last().unwrap());
    }
    out
}

fn main() {
    let input = lib::read_input();
    let (initial_state, moves) = input.split_once("\n\n").unwrap();
    let crate_stack = parse_stack(initial_state);

    let mut part1_stack = crate_stack.clone();
    parse_and_apply_moves_part1(&mut part1_stack, moves);
    dbg!(answer(&part1_stack));

    let mut part2_stack = crate_stack.clone();
    parse_and_apply_moves_part2(&mut part2_stack, moves);
    dbg!(answer(&part2_stack));
}
