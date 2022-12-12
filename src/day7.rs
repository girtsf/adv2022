use std::{cell::RefCell, collections::HashMap, rc::Rc};

mod lib;

#[derive(Debug, Default)]
struct Dir {
    dirs: HashMap<String, Rc<RefCell<Dir>>>,
    files: HashMap<String, usize>,
    /// Size of files in this directory + total sizes of all subdirectories. None if it hasn't been
    /// calculated yet.
    total_size: Option<usize>,
}

impl Dir {
    fn parse_ls<'a>(&mut self, lines: impl Iterator<Item = &'a str>) {
        for line in lines {
            dbg!(&line);
            match line.split_once(" ").unwrap() {
                ("dir", _) => {
                    // we don't actually need this, as we create the dirs when we "cd" into them.
                }
                (size, filename) => {
                    self.files
                        .insert(filename.to_string(), size.parse::<usize>().unwrap());
                }
            }
        }
    }

    fn recalculate_total_sizes(&mut self) {
        let mut total_size = 0usize;
        for dir in self.dirs.values_mut() {
            dir.borrow_mut().recalculate_total_sizes();
            total_size += dir.borrow().total_size.unwrap();
        }
        for file_size in self.files.values_mut() {
            total_size += *file_size;
        }
        self.total_size = Some(total_size);
    }

    /// Finds the sum of all total_sizes that is below 100K.
    fn part1(&self) -> usize {
        let dir_100k_sizes = self
            .dirs
            .values()
            .map(|dir| dir.borrow().part1())
            .sum::<usize>();
        let our_size = self.total_size.unwrap();
        if our_size <= 100_000 {
            our_size + dir_100k_sizes
        } else {
            dir_100k_sizes
        }
    }

    /// Finds the smallest total size >= at_least.
    fn part2(&self, at_least: usize) -> usize {
        let mut min_dir = self
            .dirs
            .values()
            .fold(usize::MAX, |acc, dir| acc.min(dir.borrow().part2(at_least)));

        let our_size = self.total_size.unwrap();
        if our_size >= at_least {
            min_dir = min_dir.min(our_size)
        }
        min_dir
    }
}

fn split_into_commands(input: &str) -> Vec<&str> {
    input.trim_start_matches("$ ").split("\n$ ").collect()
}

fn simulate_commands(commands: &[&str]) -> Dir {
    let root = Rc::new(RefCell::new(Dir::default()));

    let mut dir_stack: Vec<Rc<RefCell<Dir>>> = Vec::new();
    dir_stack.push(root.clone());

    for cmd in commands {
        let mut lines = cmd.lines();
        let first_line = lines.next().unwrap();
        match first_line {
            "cd /" => {
                dir_stack.clear();
                dir_stack.push(root.clone());
            }
            "cd .." => {
                if dir_stack.len() > 1 {
                    dir_stack.pop();
                }
            }
            "ls" => {
                dir_stack.last().unwrap().borrow_mut().parse_ls(lines);
            }
            // Otherwise, it must be "cd foo".
            _ => {
                assert!(first_line.starts_with("cd "));
                let path: String = first_line.chars().skip(3).collect();
                let new_dir = {
                    let mut cur_dir = dir_stack.last().unwrap().borrow_mut();
                    cur_dir.dirs.entry(path).or_default().clone()
                };
                dir_stack.push(new_dir);
            }
        }
    }
    root.take()
}

fn main() {
    let input = lib::read_input();
    let commands = split_into_commands(&input);
    dbg!(&commands);
    let mut root = simulate_commands(&commands);
    dbg!(&root);
    root.recalculate_total_sizes();
    dbg!(&root);
    dbg!(root.part1());

    // Part 2:
    const TOTAL_SPACE: usize = 70000000;
    const SPACE_NEEDED: usize = 30000000;
    let max_allowed_used = TOTAL_SPACE - SPACE_NEEDED;
    let must_delete_at_least = root.total_size.unwrap() - max_allowed_used;
    dbg!(&must_delete_at_least);

    dbg!(root.part2(must_delete_at_least));
}
