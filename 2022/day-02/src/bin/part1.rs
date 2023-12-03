use std::collections::HashMap;

#[derive(Debug)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Result {
    Win,
    Loss,
    Draw,
}

fn main() {
    let char_action = build_action_map();
    let action_results = build_action_results_map();
    let input = include_str!("./input.txt");
    let total = calc_total(input, char_action, action_results);
    println!("total: {:?}", total);
}

fn build_action_map<'a>() -> HashMap<&'a str, Action> {
    let m = HashMap::from([
        ("A", Action::Rock),
        ("B", Action::Paper),
        ("C", Action::Scissors),
        ("X", Action::Rock),
        ("Y", Action::Paper),
        ("Z", Action::Scissors),
    ]);
    m
}

fn build_action_results_map<'a>() -> HashMap<&'a str, Result> {
    let m = HashMap::from([
        ("A Z", Result::Loss), // Rock, Scissors
        ("A X", Result::Draw), // Rock, Rock
        ("A Y", Result::Win),  // Rock, Paper
        ("B Z", Result::Win),  // Paper, Scissors
        ("B X", Result::Loss), // Paper, Rock
        ("B Y", Result::Draw), // Paper, Paper
        ("C Z", Result::Draw), // Scissors, Scissors
        ("C X", Result::Win),  // Scissors, Rock
        ("C Y", Result::Loss), // Scissors, Paper
    ]);
    m
}

fn round_result(outcome: Option<&Result>, my_action: Option<&Action>) -> i32 {
    let mut out_val: i32 = 0;
    match outcome {
        Some(Result::Win) => out_val = 6,
        Some(Result::Draw) => out_val = 3,
        Some(Result::Loss) => out_val = 0,
        _ => out_val = 0
    }

    let mut act_val: i32 = 0;
    match my_action {
        Some(Action::Rock) => act_val = 1,
        Some(Action::Paper) => act_val = 2,
        Some(Action::Scissors) => act_val = 3,
        _ => act_val = 0
    }

    out_val + act_val
}

fn calc_total(input: &str, actions: HashMap<&str, Action>, results: HashMap<&str, Result>) -> i32 {
    let mut t: i32 = 0;
    let lines = input.split("\r\n");
    for line in lines {
        if line.is_empty() == false {
            let res = results.get(line);
            let tmp: Vec<char> = line.chars().collect();
            let act_val = tmp[tmp.len() - 1].to_string();
            let act = actions.get(act_val.as_str());
            t += round_result(res, act);
        }
    }
    t
}
