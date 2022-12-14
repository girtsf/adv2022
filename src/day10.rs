mod lib;

#[derive(Debug)]
enum Instruction {
    Addx(isize),
    Noop,
}

impl Instruction {
    fn parse(line: &str) -> Instruction {
        match (line, line.split_once(" ")) {
            ("noop", None) => Instruction::Noop,
            (_, Some(("addx", val))) => Instruction::Addx(val.parse::<isize>().unwrap()),
            _ => panic!("unexpected input {}", line),
        }
    }
}

#[derive(Debug)]
struct Cpu {
    cycle: usize,
    x: isize,
    signal_strength_sum: isize,
    scan_line: [char; 40],
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            cycle: 1,
            x: 1,
            signal_strength_sum: 0,
            scan_line: ['.'; 40],
        }
    }
    fn maybe_record_signal_strength(&mut self) {
        if (self.cycle + 20) % 40 == 0 {
            let signal_strength = self.cycle as isize * self.x;
            // dbg!(self.cycle, signal_strength);
            self.signal_strength_sum += signal_strength;
        }
    }
    fn handle_cycle(&mut self) {
        self.maybe_record_signal_strength();
        let pixel_pos = (self.cycle - 1) % 40;
        self.scan_line[pixel_pos] = if (pixel_pos as isize - self.x).abs() <= 1 {
            '#'
        } else {
            '.'
        };
        if pixel_pos == 39 {
            let line = self.scan_line.iter().collect::<String>();
            dbg!(line);
        }
    }
    fn execute(&mut self, inst: Instruction) {
        // dbg!(&self, &inst);
        self.handle_cycle();

        match inst {
            Instruction::Addx(v) => {
                self.cycle += 1;
                self.handle_cycle();
                self.x += v;
            }
            Instruction::Noop => {}
        }
        self.cycle += 1;
    }
}

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = Instruction> + 'a {
    input.lines().map(|line| Instruction::parse(line.trim()))
}

fn main() {
    let input = lib::read_input();
    let cmd_iter = parse_input(&input);
    let mut cpu = Cpu::new();
    cmd_iter.for_each(|inst| cpu.execute(inst));
    dbg!(&cpu.signal_strength_sum);
}
