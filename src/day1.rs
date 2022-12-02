mod lib;

fn main() {
    let input = lib::read_input();
    let mut calories_by_elf = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    calories_by_elf.sort();

    // Part 1:
    let top_elf = calories_by_elf.last().unwrap();
    dbg!(top_elf);

    // Part 2:
    let top_three_elves = calories_by_elf.iter().rev().take(3).sum::<u32>();
    dbg!(top_three_elves);
}
