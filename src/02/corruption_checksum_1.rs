use std::env;

/// difference of maximum and minimum
fn line_checksum(line: &str) -> usize {
    let numbers: Vec<_> = line.split_whitespace()
        .map(|i| usize::from_str_radix(i, 10).unwrap()).collect();
    let max = *numbers.iter().max().unwrap();
    let min = *numbers.iter().min().unwrap();
    max - min
}

fn spreadsheet_checksum(input: &str) -> usize {
    input.split("\n").map(|line| line_checksum(line)).sum()
}

fn main() {
    let args: Vec<_> = env::args().collect();
    println!("{}", spreadsheet_checksum(&args[1]));
}
