pub fn read_input() -> String {
    let path = std::env::args().nth(1).expect("pls provide input file");
    std::fs::read_to_string(path).expect("read failed")
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Pos(pub isize, pub isize);

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}
