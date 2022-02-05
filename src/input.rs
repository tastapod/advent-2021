use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn for_day(day: i32) -> Vec<String> {
    read_lines(format!("src/day{}_input.txt", day))
        .unwrap()
        .map(|s| s.unwrap())
        .collect::<Vec<String>>()
}

/// from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
