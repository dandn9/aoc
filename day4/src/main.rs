use std::collections::HashMap;

fn main() {
    let input_1 = include_str!("input_1.txt");

    println!("---RESULT_1---");
    let result_1 = solve_1(input_1);
    println!("{result_1}");
    println!("---RESULT_2---");
    let result_2 = solve_2(input_1);
    println!("{result_2}");
}

fn solve_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut splitted = line.split(':');

            let game_id: u32 = splitted.next().unwrap()[5..].trim().parse().unwrap();
            let mut game_results = splitted.next().unwrap().split('|');

            let winning_numbers: Vec<_> = game_results
                .next()
                .unwrap()
                .split(' ')
                .filter(|k| *k != "")
                .collect();
            let played_numbers: Vec<_> = game_results
                .next()
                .unwrap()
                .split(' ')
                .filter(|v| *v != "")
                .collect();

            played_numbers.iter().fold(0, |acc, el| {
                if winning_numbers.contains(el) {
                    return if acc > 0 { acc * 2 } else { 1 };
                } else {
                    acc
                }
            })
        })
        .sum()
}

fn solve_2(input: &str) -> u32 {
    let num_games: usize = input.lines().count();
    let mut copies_count = vec![1; num_games];

    input
        .lines()
        .map(|line| {
            let mut splitted = line.split(':');

            let game_id: u32 = splitted.next().unwrap()[5..].trim().parse().unwrap();
            let game_index = game_id - 1;
            let mut game_results = splitted.next().unwrap().split('|');

            let winning_numbers: Vec<_> = game_results
                .next()
                .unwrap()
                .split(' ')
                .filter(|k| *k != "")
                .collect();
            let played_numbers: Vec<_> = game_results
                .next()
                .unwrap()
                .split(' ')
                .filter(|v| *v != "")
                .collect();

            let copies_won = played_numbers.iter().fold(0, |acc, el| {
                if winning_numbers.contains(el) {
                    return acc + 1;
                } else {
                    return acc;
                };
            });

            for copy_index in game_id..game_id + copies_won {
                copies_count[copy_index as usize] =
                    copies_count[copy_index as usize] + copies_count[game_index as usize];
            }
            copies_count[game_index as usize]
        })
        .sum()
}
