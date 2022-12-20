pub type Pos = (i8, i8);

fn move_cursor(x: u8, y: u8) {
    print!("\x1b[{};{}H", x, y);
}

pub fn clear_screen() {
    print!("\x1b[2J");
    move_cursor(0, 0);
}

