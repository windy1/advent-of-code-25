use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
};

pub fn clear_screen() {
    print!("\x1B[3J\x1B[H\x1B[2J");
}

pub fn hide_cursor() {
    print!("\x1B[?25l");
}

pub fn show_cursor() {
    print!("\x1B[?25h");
}

pub struct ReadProgress {
    pub bytes_read: u64,
    pub total_bytes: u64,
}

pub fn read_to_string_with_progress<F>(path: &Path, mut progress_fn: F) -> io::Result<String>
where
    F: FnMut(ReadProgress),
{
    let file = File::open(path)?;
    let total_bytes = file.metadata()?.len();
    let mut reader = BufReader::new(file);

    let mut buffer = String::new();
    let mut chunk = vec![0u8; 8192]; // 8KB chunks
    let mut bytes_read = 0u64;

    loop {
        let n = reader.read(&mut chunk)?;
        if n == 0 {
            break;
        }

        buffer.push_str(&String::from_utf8_lossy(&chunk[..n]));
        bytes_read += n as u64;

        progress_fn(ReadProgress {
            bytes_read,
            total_bytes,
        });
    }

    Ok(buffer)
}
