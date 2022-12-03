use std::collections::HashSet;

mod lib;

fn priority(item: u8) -> u8 {
    match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => panic!("invalid char: {}", item),
    }
}

fn str_to_u8_set(s: &str) -> HashSet<u8> {
    HashSet::<u8>::from_iter(s.as_bytes().to_owned())
}

/// Finds a common element among the compartments/backpacks.
fn common(cont: &[&str]) -> u8 {
    let mut set_common = str_to_u8_set(cont[0]);
    for x in cont.iter().skip(1) {
        set_common = set_common
            .intersection(&str_to_u8_set(x))
            .cloned()
            .collect();
    }
    if set_common.len() != 1 {
        panic!("expected one element in common, but got: {:?}", set_common);
    }
    set_common.into_iter().next().unwrap()
}

fn main() {
    let input = lib::read_input();
    let lines: Vec<&str> = input.lines().collect();
    let part1 = lines
        .iter()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            priority(common(&[a, b])) as u32
        })
        .sum::<u32>();
    dbg!(part1);

    let part2 = lines
        .chunks_exact(3)
        .map(|three| priority(common(three)) as u32)
        .sum::<u32>();
    dbg!(part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(priority(b'b'), 2);
        assert_eq!(priority(b'B'), 28);
    }
}
