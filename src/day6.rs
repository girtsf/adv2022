use std::collections::HashSet;

mod lib;

fn find_first_unique_sequence(input: &str, window: usize) -> usize {
    let chars: Vec<_> = input.chars().collect();
    for i in window..chars.len() {
        let x = &chars[i - window..i];
        let h = HashSet::<char>::from_iter(x.iter().cloned());
        if h.len() == window {
            return i;
        }
    }
    panic!("shouldn't get here");
}

fn main() {
    let input = lib::read_input();

    dbg!(find_first_unique_sequence(&input, 4));
    dbg!(find_first_unique_sequence(&input, 14));
}
