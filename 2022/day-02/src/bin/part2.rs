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
    ]);
    m
}

fn build_action_results_map<'a>() -> HashMap<&'a str, Result> {
    let m = HashMap::from([
        ("A Z", Result::Win), // Rock, Scissors
        ("A X", Result::Loss), // Rock, Rock
        ("A Y", Result::Draw),  // Rock, Paper
        ("B Z", Result::Win),  // Paper, Scissors
        ("B X", Result::Loss), // Paper, Rock
        ("B Y", Result::Draw), // Paper, Paper
        ("C Z", Result::Win), // Scissors, Scissors
        ("C X", Result::Loss),  // Scissors, Rock
        ("C Y", Result::Draw), // Scissors, Paper
    ]);
    m
}

fn round_result(outcome: Option<&Result>, thier_action: Option<&Action>) -> i32 {
    let mut res: i32 = 0;
    match outcome {
        Some(Result::Win) => {
            res += 6;
            match thier_action {
                Some(Action::Rock) => res += 2,
                Some(Action::Paper) => res += 3,
                Some(Action::Scissors) => res += 1,
                _ => ()
            }
        },
        Some(Result::Draw) => {
            res += 3;
            match thier_action {
                Some(Action::Rock) => res += 1,
                Some(Action::Paper) => res += 2,
                Some(Action::Scissors) => res += 3,
                _ => ()
            }
        },
        Some(Result::Loss) => {
            res += 0;
            match thier_action {
                Some(Action::Rock) => res += 3,
                Some(Action::Paper) => res += 1,
                Some(Action::Scissors) => res += 2,
                _ => ()
            }
        },
        _ => ()
    }

    res
}

fn calc_total(input: &str, actions: HashMap<&str, Action>, results: HashMap<&str, Result>) -> i32 {
    let mut t: i32 = 0;
    let lines = input.split("\r\n");
    for line in lines {
        if line.is_empty() == false {
            let res = results.get(line);
            let tmp: Vec<char> = line.chars().collect();
            let act_val = tmp[0].to_string();
            let act = actions.get(act_val.as_str());
            t += round_result(res, act);
        }
    }
    t
}
