use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{fs, process::Output};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cube {
    amount: u32,
    color: Colour,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    id: u32,
    cubes: Vec<Cube>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Colour {
    Blue,
    Red,
    Green,
}

#[allow(dead_code)]
pub fn calculate_id_sum_1(contents: Vec<&str>, r: u32, g: u32, b: u32) -> u32 {
    let mut games: Vec<Game> = vec![];
    for line in contents {
        games.push(parse_game(line));
    }
    let mut sum: u32 = 0;
    for game in games {
        let mut possible = true;
        for cube in game.cubes {
            match cube.color {
                Colour::Red => {
                    if cube.amount > r {
                        possible = false;
                        break;
                    }
                }
                Colour::Green => {
                    if cube.amount > g {
                        possible = false;
                        break;
                    }
                }
                Colour::Blue => {
                    if cube.amount > b {
                        possible = false;
                        break;
                    }
                }
            }
        }
        if possible {
            sum += game.id;
        }
    }
    sum
}

pub fn calculate_id_sum_2(contents: Vec<&str>) -> u32 {
    let mut games: Vec<Game> = vec![];
    for line in contents {
        games.push(parse_game(line));
    }
    let mut sum: u32 = 0;
    for game in games {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for cube in &game.cubes {
            match cube.color {
                Colour::Red => {
                    if cube.amount > red {
                        red = cube.amount
                    }
                }
                Colour::Green => {
                    if cube.amount > green {
                        green = cube.amount
                    }
                }
                Colour::Blue => {
                    if cube.amount > blue {
                        blue = cube.amount
                    }
                }
            }
        }
        sum += red * green * blue;
    }
    sum
}

//Game 1: 10 green, 9 blue, 1 red; 1 red, 7 green; 11 green, 6 blue; 8 blue, 12 green
pub fn parse_game(line: &str) -> Game {
    // println!("{:?}", line.split(":").collect::<Vec<&str>>());
    let line_vec = line.split(":").collect::<Vec<&str>>();
    let game = Game {
        id: line_vec[0].replace("Game ", "").parse::<u32>().unwrap(),
        cubes: parse_cubes(line_vec[1]),
    };
    game
}

//[10 green, 9 blue, 1 red; 1 red, 7 green; 11 green, 6 blue; 8 blue, 12 green]
pub fn parse_cubes(line: &str) -> Vec<Cube> {
    let mut cubes_vec: Vec<Cube> = vec![];
    let line_vec = line.split(";").collect::<Vec<&str>>();
    //[10 green, 9 blue, 1 red, 1 red, 7 green, 11 green, 6 blue, 8 blue, 12 green]
    for cubes in line_vec {
        let cube_vec = cubes.split(",").collect::<Vec<&str>>();
        //[10 green]
        for cube in cube_vec {
            let single_cube = cube.trim().split(" ").collect::<Vec<&str>>();
            //[10, green]
            let amount = single_cube[0].parse::<u32>().unwrap();
            let color = match single_cube[1] {
                "green" => Colour::Green,
                "blue" => Colour::Blue,
                "red" => Colour::Red,
                _ => panic!("Invalid colour"),
            };
            cubes_vec.push(Cube { amount, color });
        }
    }
    cubes_vec
}
