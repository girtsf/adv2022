pub fn read_input() -> String {
    let path = std::env::args().nth(1).expect("pls provide input file");
    std::fs::read_to_string(path).expect("read failed")
}
