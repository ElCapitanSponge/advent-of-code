fn main() {
    let input = include_str!("./input_test.txt");
    let lines = input.split("\r\n");
    let mut calibration_sum: i32 = 0;
    for line in lines {
        if line.is_empty() == false {
            calibration_sum += number_get(line);
        }
    }
    println!("Sum: {:?}", calibration_sum);
}

fn number_get(line: &str) -> i32 {
    let mut numbers: Vec<String> = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    for c in chars {
        if c.is_numeric() == true {
            let t: String = c.to_string();
            numbers.push(t);
        }
    }
    let size = numbers.len().to_string().parse::<i32>().unwrap();
    if size > 0 {
        if size == 1 {
            return [numbers[0].to_string(), numbers[0].to_string()]
                .join("")
                .parse::<i32>()
                .unwrap();
        } else {
            return [
                numbers[0].to_string(),
                numbers[numbers.len() - 1].to_string(),
            ]
            .join("")
            .parse::<i32>()
            .unwrap();
        }
    }
    0
}
