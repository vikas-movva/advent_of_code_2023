mod day_1;
mod utils;
#[allow(unused_imports)]
use crate::day_1::{calculate_code_1, calculate_code_2};
use crate::utils::get_file_contents;
fn main() {
    let contents = get_file_contents("src/input-1.txt");
    let words = contents.split_whitespace().collect();
    // let words = vec![
        //     "two1nine",
        //     "eightwothree",
        //     "abcone2threexyz",
        //     "xtwone3four",
        //     "33gfdhasjkgfasdhjk",
        //     "1werweq12rewq233rewq44556eqr6778rewq8rewq99rewq00",
        // "v6",
        // "xmfbn34",
    // ];
    let sum = calculate_code_2(words);
    println!("Sum: {}", sum);
}
