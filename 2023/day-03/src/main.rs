fn main() {
    let input = include_str!("input.txt");
    let schematic = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let row_max: usize = schematic.len();
    let col_max: usize = schematic[0].len();
    let mut values: Vec<usize> = Vec::new();
    let mut values_idx: usize = 0;

    for row in 0..row_max {
        for col in 0..col_max {
            if schematic[row][col].is_numeric() == false && schematic[row][col] != '.' {
                // get surrounding characters on all eight points on the 2 dimensional array
                let mut surrounding_chars: Vec<char> = Vec::new();
                // TL, TM, TR
                if row > 0 {
                    if col > 0 {
                        surrounding_chars.push(schematic[row - 1][col - 1]);
                    }
                    surrounding_chars.push(schematic[row - 1][col]);
                    if col < col_max - 1 {
                        surrounding_chars.push(schematic[row - 1][col + 1]);
                    }
                } else {
                    surrounding_chars.push('.');
                    surrounding_chars.push('.');
                    surrounding_chars.push('.');
                }
                // CL, CR
                if col > 0 {
                    surrounding_chars.push(schematic[row][col - 1]);
                } else {
                    surrounding_chars.push('.');
                }
                if col < col_max - 1 {
                    surrounding_chars.push(schematic[row][col + 1]);
                } else {
                    surrounding_chars.push('.');
                }
                // BL, BM, BR
                if row < row_max - 1 {
                    if col > 0 {
                        surrounding_chars.push(schematic[row + 1][col - 1]);
                    }
                    surrounding_chars.push(schematic[row + 1][col]);
                    if col < col_max - 1 {
                        surrounding_chars.push(schematic[row + 1][col + 1]);
                    }
                } else {
                    surrounding_chars.push('.');
                    surrounding_chars.push('.');
                    surrounding_chars.push('.');
                }
                // TODO: Go through the surrounding chars and extract the
                // Numeric values and walk the rest of the applicable array to resolve the number
                for i in 0..7 {
                    match i {
                        0 => {
                            // INFO: Top left
                            if surrounding_chars[i].is_numeric() {
                                let result = process_num(
                                    surrounding_chars[i].to_digit(10).unwrap() as usize,
                                    walk(true, row - 1, col - 1, schematic.clone()),
                                    walk(false, row - 1, col - 1, schematic.clone()),
                                );
                                values.push(result);
                                values_idx += 1;
                            }
                        }
                        1 => {
                            // INFO: Top middle
                            if surrounding_chars[i].is_numeric() {
                                let result = process_num(
                                    surrounding_chars[i].to_digit(10).unwrap() as usize,
                                    walk(true, row - 1, col, schematic.clone()),
                                    walk(false, row - 1, col, schematic.clone()),
                                );
                                if values_idx == 0 || values[values_idx - 1] != result {
                                    values.push(result);
                                    values_idx += 1;
                                }
                            }
                        }
                        2 => {
                            // INFO: Top right
                            if surrounding_chars[i].is_numeric() {
                                let result = process_num(
                                    surrounding_chars[i].to_digit(10).unwrap() as usize,
                                    walk(true, row - 1, col + 1, schematic.clone()),
                                    walk(false, row - 1, col + 1, schematic.clone()),
                                );
                                if values_idx == 0 || values[values_idx - 1] != result {
                                    values.push(result);
                                    values_idx += 1;
                                }
                            }
                        }
                        3 => {
                            // INFO: Middle left
                            if surrounding_chars[i].is_numeric() {
                                let result = process_num(
                                    surrounding_chars[i].to_digit(10).unwrap() as usize,
                                    walk(true, row, col - 1, schematic.clone()),
                                    Vec::new() as Vec<char>,
                                );
                                values.push(result);
                                values_idx += 1;
                            }
                        }
                        4 => {
                            // INFO: Middle Right
                            if surrounding_chars[i].is_numeric() {
                                let result = process_num(
                                    surrounding_chars[i].to_digit(10).unwrap() as usize,
                                    Vec::new() as Vec<char>,
                                    walk(false, row, col + 1, schematic.clone()),
                                );
                                values.push(result);
                                values_idx += 1;
                            }
                        }
                        5 => {
                            // INFO: Bottom left
                            if surrounding_chars[i].is_numeric() {
                                let result = process_num(
                                    surrounding_chars[i].to_digit(10).unwrap() as usize,
                                    walk(true, row + 1, col - 1, schematic.clone()),
                                    walk(false, row + 1, col - 1, schematic.clone()),
                                );
                                values.push(result);
                                values_idx += 1;
                            }
                        }
                        6 => {
                            // INFO: Bottom middle
                            if surrounding_chars[i].is_numeric() {
                                let result = process_num(
                                    surrounding_chars[i].to_digit(10).unwrap() as usize,
                                    walk(true, row + 1, col, schematic.clone()),
                                    walk(false, row + 1, col, schematic.clone()),
                                );
                                if values_idx == 0 || values[values_idx - 1] != result {
                                    values.push(result);
                                    values_idx += 1;
                                }
                            }
                        }
                        7 => {
                            // INFO: Bottom right
                            if surrounding_chars[i].is_numeric() {
                                let result = process_num(
                                    surrounding_chars[i].to_digit(10).unwrap() as usize,
                                    walk(true, row + 1, col + 1, schematic.clone()),
                                    walk(false, row + 1, col + 1, schematic.clone()),
                                );
                                if values_idx == 0 || values[values_idx - 1] != result {
                                    values.push(result);
                                    values_idx += 1;
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }
    println!("Parts sum: {:?}", values.iter().sum::<usize>());
}

fn process_num(start: usize, left: Vec<char>, right: Vec<char>) -> usize {
    let num_str: String = left.iter().rev().collect::<String>()
        + start.to_string().as_str()
        + right.iter().collect::<String>().as_str();
    num_str.parse::<usize>().unwrap()
}

fn walk(walk_left: bool, row: usize, mut col: usize, schema: Vec<Vec<char>>) -> Vec<char> {
    let mut values: Vec<char> = Vec::new();
    let mut walking: bool = true;
    while walking == true {
        if walk_left == true {
            if col > 0 {
                col -= 1;
            } else {
                walking = false;
            }
        } else {
            if col < schema[row].len() - 1 {
                col += 1;
            } else {
                walking = false;
            }
        }
        if walking == true {
            if schema[row][col].is_numeric() == true {
                values.push(schema[row][col]);
            } else {
                walking = false;
            }
        }
    }
    values
}
