fn main() {
    let input = include_str!("./input.txt");
    println!("Total ammount of paper: {:?}", get_paper(input));
    println!("Total ammount of ribbon: {:?}", get_ribbon(input));
}

fn get_paper(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let itms: Vec<&str> = line.split("x").collect();
            let l = itms[0].parse::<i32>().unwrap();
            let w = itms[1].parse::<i32>().unwrap();
            let h = itms[2].parse::<i32>().unwrap();
            let lw = l * w;
            let wh = w * h;
            let hl = h * l;
            let mut small = lw;
            if wh < small {
                small = wh;
            }
            if hl < small {
                small = hl;
            }
            let ammount: i32 = 2 * lw + 2 * wh + 2 * hl + small;
            ammount as usize
        })
        .sum()
}

fn get_ribbon(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut itms: Vec<&str> = line.split("x").collect();
            itms.sort();
            let l = itms[0].parse::<i32>().unwrap();
            let w = itms[1].parse::<i32>().unwrap();
            let h = itms[2].parse::<i32>().unwrap();
            let bow = l+l+w+w+(l*w*h);
            bow as usize
        })
        .sum()
}
