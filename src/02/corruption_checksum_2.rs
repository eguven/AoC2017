use std::env;

/// quotient of the only two evenly divisible numbers
fn line_checksum(line: &str) -> usize {
    let mut numbers: Vec<_> = line.split("\t")
        .map(|i| usize::from_str_radix(i, 10).unwrap()).collect();
    numbers.sort_unstable();
    numbers.reverse();

    for numerator_idx in 0..numbers.len() {
        for denominator_idx in (numerator_idx + 1)..numbers.len() {
            if numbers[numerator_idx] % numbers[denominator_idx] == 0 {
                return numbers[numerator_idx] / numbers[denominator_idx];
            }
        }
    }
    panic!("Could not find evenly divisible numbers in {:?}", numbers);
}

fn spreadsheet_checksum(input: &str) -> usize {
    input.split("\n").map(|line| line_checksum(line)).sum()
}

fn main() {
    let args: Vec<_> = env::args().collect();
    println!("{}", spreadsheet_checksum(&args[1]));
}
