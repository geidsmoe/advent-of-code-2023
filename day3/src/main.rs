fn main() {
    part1();
}

fn part1() {
    let lines: Vec<_> = include_str!("input.txt").lines().collect();
    let input_width: usize = lines.get(0).unwrap().len();
    let input_height: usize = lines.len();
    println!("{} char per line, {} lines", input_width, input_height);

    let mut answer = 0;
    for (i, &line) in lines.iter().enumerate() {
        let mut current_number = 0;
        let mut adjacent_to_symbol = false;
        for (j, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                current_number = (current_number * 10) + c.to_digit(10).unwrap();
                if adjacent_to_symbol {
                    continue;
                }
                if j > 0 {
                    let prev_char = line.as_bytes()[j-1] as char;
                    if prev_char != '.' && !prev_char.is_ascii_digit() {
                        adjacent_to_symbol = true;
                        continue;
                    }
                }
                if j < input_width - 1 {
                    let next_char = line.as_bytes()[j+1] as char;
                    if next_char != '.' && !next_char.is_ascii_digit() {
                        adjacent_to_symbol = true;
                        continue;
                    }
                }
                if i < input_height - 1 {
                    let below_char = lines.get(i + 1).unwrap().as_bytes()[j] as char;
                    if below_char != '.' && !below_char.is_ascii_digit() {
                        adjacent_to_symbol = true;
                        continue;
                    }
                }
                if i > 0 {
                    let above_char = lines.get(i - 1).unwrap().as_bytes()[j] as char;
                    if above_char != '.' && !above_char.is_ascii_digit() {
                        adjacent_to_symbol = true;
                        continue;
                    }
                }
                if i > 0 && j > 0 {
                    let above_left_char = lines.get(i - 1).unwrap().as_bytes()[j - 1] as char;
                    if above_left_char != '.' && !above_left_char.is_ascii_digit() {
                        adjacent_to_symbol = true;
                        continue;
                    }
                }
                if i > 0 && j < input_width - 1 {
                    let above_right_char = lines.get(i - 1).unwrap().as_bytes()[j + 1] as char;
                    if above_right_char != '.' && !above_right_char.is_ascii_digit() {
                        adjacent_to_symbol = true;
                        continue;
                    }
                }
                if i < input_height - 1 && j > 0 {
                    let below_left_char = lines.get(i + 1).unwrap().as_bytes()[j - 1] as char;
                    if below_left_char != '.' && !below_left_char.is_ascii_digit() {
                        adjacent_to_symbol = true;
                        continue;
                    }
                }
                if i < input_height - 1 && j < input_width - 1 {
                    let below_right_char = lines.get(i + 1).unwrap().as_bytes()[j + 1] as char;
                    if below_right_char != '.' && !below_right_char.is_ascii_digit() {
                        adjacent_to_symbol = true;
                        continue;
                    }
                }
            } else {
                if current_number > 0 && !adjacent_to_symbol {
                    println!("{current_number}, {adjacent_to_symbol}");
                }
                if adjacent_to_symbol {
                    answer += current_number;
                }
                current_number = 0;
                adjacent_to_symbol = false;
            }
        }
    }
    println!("{answer}");
}
