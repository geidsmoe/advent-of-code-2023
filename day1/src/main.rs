fn main() {
    let num_str_array = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut sum_nums = 0;
    for line in include_str!("input.txt").lines() {
        println!("{line}");
        let mut first_num: i32 = -1;
        let mut last_num: i32 = -1;
        // if they were really mean we'd have to use chars().count() because unicode is hard
        let line_len = line.len();
        let mut i = 0;
        while i < line_len  {
            let c = line.as_bytes()[i] as char;
            let curr_substr = line.get(i..).unwrap();
            if c.is_numeric() {
                if first_num == -1 {
                    first_num = c.to_digit(10).unwrap() as i32;
                }
                last_num = c.to_digit(10).unwrap() as i32;
                i += 1;
            } else if let Some((index, num_str)) = num_str_array
                        .iter()
                        .position(|&substring| curr_substr.starts_with(substring))
                        .map(|index| (index, &num_str_array[index]))
            {
                if first_num == -1 {
                    first_num = (index + 1) as i32;
                }
                last_num = (index + 1) as i32;
                i += 1;
            } else {
                i += 1;
            }
        }
        println!("{first_num}, {last_num}");

        sum_nums += 10*first_num + last_num;
    }

    println!("{sum_nums}");

    //part1();
}



fn part1() {
    let mut sum_nums = 0;
    for line in include_str!("input.txt").lines() {
        //println!("{line}");
        let mut first_num: i32 = -1;
        let mut last_num: i32 = -1;
        for c in line.chars() {
            if c.is_numeric() {
                if first_num == -1 {
                    first_num = c.to_digit(10).unwrap() as i32;
                }
                last_num = c.to_digit(10).unwrap() as i32;
            }
        }

        //println!("{first_num}, {last_num}");
        sum_nums += 10*first_num + last_num;
    }

    println!("{sum_nums}");
}