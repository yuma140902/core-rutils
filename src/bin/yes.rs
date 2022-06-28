use std::{
    ffi::OsString,
    io::{self, BufWriter, Write},
};

const BUFSIZE: usize = 1024 * 16;

fn main() {
    let expletive = std::env::args().nth(1).unwrap_or("y".to_string());
    let mut writer = BufWriter::with_capacity(BUFSIZE, io::stdout());
    loop {
        writeln!(writer, "{}", expletive).unwrap();
    }
}
