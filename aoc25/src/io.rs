pub fn clear_screen() {
    print!("\x1B[3J\x1B[H\x1B[2J");
}
