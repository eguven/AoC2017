use std::env;

fn inverse_captcha(input: &str) -> usize {
    let chars: Vec<_> = input.chars().collect();
    let mut sum: usize = 0;

    for idx in 0..input.len() {
        let next_idx = (idx + input.len() / 2) % input.len();  // no even checking here
        if chars[idx] == chars[next_idx] {
            sum += chars[idx].to_digit(10).unwrap() as usize;  // no err checking here
        }
    }

    sum
}

fn main() {
    let args: Vec<_> = env::args().collect();
    println!("{}", inverse_captcha(&args[1]));
}
