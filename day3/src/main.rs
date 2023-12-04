fn main() {
    part1();
}

fn part1() {
    let lines: Vec<&str> = include_str!("test_input.txt").lines().collect();
    const INPUT_WIDTH: usize = lines.get(0).unwrap().len();
    const INPUT_HEIGHT: usize = lines.len();
    println!("{} char per line, {} lines", INPUT_WIDTH, INPUT_HEIGHT);

    let answer = 0;
    for (i, line) in lines.iter().enumerate() {
        let mut current_number = 0;
        let mut adjacent_to_symbol = false;
        for (j, c) in line.chars().enumerate() {
            if c.is_numeric() {
                current_number = (current_number * 10) + c.to_digit(10);
                
            }
        }
    }
}
