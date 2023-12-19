mod day_01;
mod day_02;
mod utils;

#[allow(unused_imports)]
use crate::day_01::{calculate_code_1, calculate_code_2};
#[allow(unused_imports)]
use crate::day_02::{calculate_id_sum_1, calculate_id_sum_2};
use crate::utils::get_file_contents;
fn main() {
    let contents = get_file_contents("src/input-2.txt");
    let words: Vec<&str> = contents.lines().collect();
    let sum = calculate_id_sum_2(words);
    println!("Sum: {}", sum);
}
