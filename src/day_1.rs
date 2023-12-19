use std::collections::HashMap;

pub fn calculate_code_1(contents: Vec<&str>) -> i32 {
    let mut sum = 0;
    for line in contents {
        let mut nums: Vec<i8> = vec![];
        for char in line.chars() {
            if char.is_numeric() {
                nums.push(char.to_digit(10).unwrap() as i8);
            }
        }
        let code: i8;
        match nums.len() > 1 {
            false => code = nums[0] * 10 + nums[0],
            true => code = nums[0] * 10 + nums.last().unwrap(),
        }
        sum += code as i32;
    }
    sum
}

// this code is disgusting, but it works
pub fn calculate_code_2(contents: Vec<&str>) -> i32 {
    let mut sum = 0;
    let number_map: HashMap<&str, i8> = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect();
    let valid_leters: Vec<char> = "zotfsen".chars().collect();
    for line in contents {
        let mut nums: Vec<i8> = vec![];
        let mut p1 = 0;
        let mut p2 = 1;
        let chars: Vec<char> = line.chars().collect();
        println!("Line: {}", line);
        while p2 <= chars.len() {
            if chars[p1].is_numeric() {
                nums.push(chars[p1].to_digit(10).unwrap() as i8);
                p1 += 1;
                p2 = p1 + 1;
                continue;
            }
            let slice = &chars[p1..p2].iter().collect::<String>();
            let slice_str: &str = &*slice;
            // println!("Slice: {}, ", &slice_str);
            if number_map.contains_key(slice_str) {
                nums.push(*number_map.get(slice_str).unwrap());
                p1 += 1;
                p2 = p1 + 1;
            } else {
                if valid_leters.contains(&chars[p1]) && p2 - p1 < 5 && p2 + 1 <= chars.len() {
                    p2 += 1;
                } else {
                    p1 += 1;
                    p2 = p1 + 1;
                }
            }
        }
        let code: i8;
        match nums.len() > 1 {
            false => code = nums[0] * 10 + nums[0],
            true => code = nums[0] * 10 + nums.last().unwrap(),
        }
        println!("Nums: {:?} Code: {}", &nums, &code);
        sum += code as i32;
    }
    sum
}
