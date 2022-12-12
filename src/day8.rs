use std::iter::repeat_with;

mod lib;

#[derive(Debug)]
struct Grid {
    rows: usize,
    cols: usize,
    map: Vec<Vec<u8>>,
    visible: Vec<Vec<bool>>,
}

impl Grid {
    fn parse(s: &str) -> Grid {
        let map: Vec<Vec<u8>> = s
            .lines()
            .map(|line| line.trim().bytes().map(|c| c - b'0').collect())
            .collect();
        let rows = map.len();
        let cols = map[0].len();
        Grid {
            rows,
            cols,
            map,
            visible: repeat_with(|| std::iter::repeat(false).take(cols).collect())
                .take(rows)
                .collect(),
        }
    }

    fn calculate_visible(&mut self) {
        for row in 0..self.rows {
            // Left to right.
            self.visible[row][0] = true;
            let mut prev_height = self.map[row][0];
            for col in 1..self.cols {
                let height = self.map[row][col];
                if height > prev_height {
                    self.visible[row][col] = true;
                }
                prev_height = prev_height.max(height);
            }
            // Right to left.
            self.visible[row][self.cols - 1] = true;
            prev_height = self.map[row][self.cols - 1];
            for col in (0..self.cols - 1).rev() {
                let height = self.map[row][col];
                if height > prev_height {
                    self.visible[row][col] = true;
                }
                prev_height = prev_height.max(height);
            }
        }
        for col in 0..self.cols {
            // Top down.
            self.visible[0][col] = true;
            let mut prev_height = self.map[0][col];
            for row in 1..self.rows {
                let height = self.map[row][col];
                if height > prev_height {
                    self.visible[row][col] = true;
                }
                prev_height = prev_height.max(height);
            }
            // Bottom up.
            self.visible[self.rows - 1][col] = true;
            let mut prev_height = self.map[self.rows - 1][col];
            for row in (0..self.rows - 1).rev() {
                let height = self.map[row][col];
                if height > prev_height {
                    self.visible[row][col] = true;
                }
                prev_height = prev_height.max(height);
            }
        }
    }

    fn count_visible(&self) -> usize {
        self.visible
            .iter()
            .map(|row| row.iter().filter(|x| **x).count())
            .sum()
    }

    fn calculate_scenic_score(&self, row: usize, col: usize) -> usize {
        let mut scenic_score = 1;
        for (d_row, d_col) in [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)] {
            let mut row_ = row as isize;
            let mut col_ = col as isize;
            let house_height = self.map[row][col];
            let mut distance = 0;
            while row_ != 0
                && row_ != (self.rows - 1) as isize
                && col_ != 0
                && col_ != (self.cols - 1) as isize
            {
                distance += 1;
                row_ += d_row;
                col_ += d_col;
                let tree_height = self.map[row_ as usize][col_ as usize];
                if tree_height >= house_height {
                    break;
                }
            }
            scenic_score *= distance;
        }
        scenic_score
    }

    fn find_highest_scenic_score(&self) -> usize {
        let mut max_score = 0usize;
        for row in 0..self.rows {
            for col in 0..self.cols {
                max_score = max_score.max(self.calculate_scenic_score(row, col));
            }
        }
        max_score
    }
}

fn main() {
    let input = lib::read_input();
    let mut grid = Grid::parse(&input);
    grid.calculate_visible();
    // dbg!(&grid);
    dbg!(grid.count_visible());
    dbg!(grid.find_highest_scenic_score());
}
