fn main() {
    let input = include_str!("input_test.txt");
    let schematic = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let row_max: usize = schematic.len();
    let col_max: usize = schematic[0].len();

    for row in 0..row_max {
        for col in 0..col_max {
            if schematic[row][col].is_numeric() == false && schematic[row][col] != '.' {
                // get surrounding characters on all eight points on the 2 dimensional array
                let mut surrounding_chars: Vec<char> = Vec::new();
                // TL, TM, TR
                if row > 0 {
                    if col > 0 {
                        surrounding_chars.push(schematic[row-1][col - 1]);
                    }
                    surrounding_chars.push(schematic[row - 1][col]);
                    if col < col_max - 1 {
                        surrounding_chars.push(schematic[row-1][col + 1]);
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
                        surrounding_chars.push(schematic[row+1][col - 1]);
                    }
                    surrounding_chars.push(schematic[row + 1][col]);
                    if col < col_max - 1 {
                        surrounding_chars.push(schematic[row+1][col + 1]);
                    }
                } else {
                    surrounding_chars.push('.');
                    surrounding_chars.push('.');
                    surrounding_chars.push('.');
                }
                println!("{:?}", surrounding_chars);
                for i in 0..surrounding_chars.len() {
                    // TODO: Go through the surrounding chars and extract the
                    // Numeric values and waal the rest of the applicable array to resolve the number
                }
            }
        }
    }
}
