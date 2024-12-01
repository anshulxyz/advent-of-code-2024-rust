use std::fs;
use std::io::{self, BufRead};

pub fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = fs::File::open(filename)?;
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    Ok(lines)
}
