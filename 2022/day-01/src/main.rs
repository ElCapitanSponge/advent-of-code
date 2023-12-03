fn main() {
    let input = include_str!("./input1.txt");
    let elf_calories: Vec<i32> = calories_list(input);
    let large = largest_cal(&elf_calories);
    let top = top_three(&elf_calories);
    println!("Largest: {:?}", large);
    println!("Top three: {:?}", top);
}

fn calories_list(input: &str) -> Vec<i32> {
    let mut tmp_vec: Vec<i32> = Vec::new();
    let groups = input.split("\r\n\r\n");
    for group in groups {
        let items = group.split("\r\n");
        let mut tmp_val: i32 = 0;
        for item in items {
            if item.is_empty() == false {
                tmp_val += item.parse::<i32>().unwrap();
            }
        }
        tmp_vec.push(tmp_val)
    }
    tmp_vec.sort();
    tmp_vec
}

fn largest_cal(input: &Vec<i32>) -> i32 {
    input.iter().rev().take(1).sum()
}

fn top_three(input: &Vec<i32>) -> i32 {
    input.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calories_list_test() {
        let input = include_str!("./input1_test.txt");
        let result = calories_list(input);
        assert_eq!(result, [4000, 6000, 10000, 11000, 24000]);
    }

    #[test]
    fn largets_cal_test() {
        let input = include_str!("./input1_test.txt");
        let cal_vec = calories_list(input);
        let result = largest_cal(&cal_vec);
        assert_eq!(result, 24000);
    }

    #[test]
    fn top_three_test() {
        let input = include_str!("./input1_test.txt");
        let cal_vec = calories_list(input);
        let result = top_three(&cal_vec);
        assert_eq!(result, 45000);
    }
}
