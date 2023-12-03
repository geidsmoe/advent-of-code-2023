use std::collections::HashMap;
use std::cmp::max;

fn main() {
    part1();
    part2();
}

fn part2() {
    let lines = include_str!("input.txt").lines();

    let mut game_powers: Vec<_> = Vec::new();

    for line in lines {
        let turns = generate_game_turns(line);

        let mut color_minimums = HashMap::new();
        color_minimums.insert("red", 0);
        color_minimums.insert("green", 0);
        color_minimums.insert("blue", 0);

        for turn in turns.iter() {
            // iterate over each grab of cubes from a turn
            for (count, color) in turn.iter() {
                color_minimums.insert(*color, max(*count, *color_minimums.get(*color).unwrap()));
            }
        }
        game_powers.push(color_minimums.values().fold(1, |acc, &x| acc * x));
    }

    let answer = game_powers.iter().sum::<i32>();
    println!("{}", answer);
}

fn part1() {
    let mut invalid_game_ids: Vec<_> = Vec::new();
    let mut color_limits = HashMap::new();
    color_limits.insert("red", 12);
    color_limits.insert("green", 13);
    color_limits.insert("blue", 14);
    let lines = include_str!("input.txt").lines();
    let mut num_games = 0;
    for (game_num, line) in lines.enumerate() {
        let turns = generate_game_turns(line);

        'outer: for turn in turns.iter() {
            for (count, color) in turn.iter() {
                if *count > *color_limits.get(*color).unwrap() {
                    invalid_game_ids.push(game_num + 1);
                    break 'outer;
                }
            }
        }

        num_games = game_num + 1;
    }

    let answer: i32 = ((num_games * (num_games + 1) / 2) - invalid_game_ids.iter().sum::<usize>()) as i32;
    println!("{}", answer);
}

fn generate_game_turns(line: &str) -> Vec<Vec<(i32, &str)>> {
    let parts: Vec<&str> = line.split(":").collect();

    let unparsed_turns: Vec<&str> = parts[1].split(";").collect::<Vec<_>>();
    let turns: Vec<Vec<(i32, &str)>> = unparsed_turns
        .iter()
        .map(|&grab| {
            grab.split(',')
                .map(|item| {
                    let grab_parts: Vec<&str> = item.trim().split_whitespace().collect();
                    let count: i32 = grab_parts[0].parse().unwrap_or(0);
                    let color = grab_parts[1];
                    (count, color)
                })
                .collect()
        })
        .collect();

    return turns;
}
