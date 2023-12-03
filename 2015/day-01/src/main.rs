fn main() {
    let input = include_str!("./input.txt");
    let chars: Vec<char> = input.chars().collect();
    let mut result: i32 = 0;
    let mut basement: Option<usize> = None;
    for (i, c) in chars.iter().enumerate() {
        match c {
            '(' => result += 1,
            ')' => result -= 1,
            _ => result = result
        }
        if basement.is_none() && result == -1 {
            basement = Some(i + 1);
        }
    }
    println!("Floor Number: {:?}", result);
    println!("Basement Idx: {:?}", basement);
}
