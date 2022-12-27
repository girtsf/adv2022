use adv2022::read_input;

fn parse_snafu(s: &str) -> isize {
    let mut acc = 0isize;
    for c in s.trim().chars() {
        acc = acc * 5
            + match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!("invalid char"),
            }
    }
    acc
}

fn to_snafu(mut i: isize) -> String {
    assert!(i > 0);
    let mut out: Vec<char> = Vec::new();
    while i > 0 {
        let digit = i % 5;
        out.push(match digit {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => {
                i += 5;
                '='
            }
            4 => {
                i += 5;
                '-'
            }
            _ => panic!(),
        });
        i /= 5;
    }
    out.iter().rev().collect()
}

fn main() {
    let sum = read_input()
        .lines()
        .map(|line| parse_snafu(line))
        .sum::<isize>();
    dbg!(&sum);
    dbg!(to_snafu(sum));
}
