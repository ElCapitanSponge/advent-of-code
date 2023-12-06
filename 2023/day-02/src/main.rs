use std::collections::HashMap;

fn main() {
    let input = include_str!("./input_test.txt");
    let loaded_cubes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut min_sum: Vec<i32> = Vec::new();
    let good_games: i32 = input.lines().map(|line| {
        let game_split = line.split(": ").collect::<Vec<&str>>();
        let game_num: i32 = game_split[0].replace("Game ", "").parse::<i32>().unwrap();
        let game_vals = game_split[1].replace(";", ",");
        let game_shown = game_vals.split(", ").collect::<Vec<&str>>();
        let mut r_vals = get_color_vals(" red", game_shown.clone());
        r_vals.sort_by(|a, b| b.cmp(a));
        let mut g_vals = get_color_vals(" green", game_shown.clone());
        g_vals.sort_by(|a, b| b.cmp(a));
        let mut b_vals = get_color_vals(" blue", game_shown.clone());
        b_vals.sort_by(|a, b| b.cmp(a));
        min_sum.push(r_vals[0] * g_vals[0] * b_vals[0]);
        if valid_vals(r_vals, loaded_cubes.get("red")) == true
            && valid_vals(g_vals, loaded_cubes.get("green")) == true
            && valid_vals(b_vals, loaded_cubes.get("blue")) == true
        {
            return game_num;
        }
        0
    }).sum();
    println!("Game Sum: {:?}", good_games);
    println!("Power Sum: {:?}", min_sum.iter().sum::<i32>());
}

fn valid_vals(values: Vec<i32>, max: Option<&i32>) -> bool {
    let mut t_max: i32 = 0;
    match max.ok_or(0) {
        Ok(v) => {
            t_max = *v;
        },
        Err(v) => (),
    }
    let mut invalid: Vec<i32> = Vec::new();
    for val in values.iter() {
        if *val > t_max {
            invalid.push(*val);
        }
    }
    if invalid.len() > 0 {
        return false;
    }
    true
}

fn get_color_vals(color: &str, game: Vec<&str>) -> Vec<i32> {
    game.iter()
        .filter_map(|val| {
            if val.contains(color) == false {
                return None;
            }
            Some(val.replace(color, "").parse::<i32>().unwrap())
        })
        .collect::<Vec<i32>>()
}
