const ESC: char = 0x1B as char;

pub fn width() -> usize {
    80
}

pub fn move_up(n: usize) {
    print!("{}[{}A", ESC, n);
}

pub fn clear() {
    print!("{}[2", ESC);
}
