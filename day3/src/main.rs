fn main() {
    let input = include_str!("input1.txt");

    println!("---RESULT_1---");
    let result_1 = solve_1(input);
    println!("{}", result_1);
    println!("---RESULT_2---");
    let result_2 = solve_2(input);
    println!("{}", result_2);
}

fn solve_1(input: &str) -> u32 {
    let mat: Vec<_> = input.split('\n').collect();
    let mut sum = 0;

    for (row, str) in mat.iter().enumerate() {
        for (col, char) in str.chars().enumerate() {
            if char != '.' && !char.is_ascii_digit() {
                // it means its a symbol
                for i in row - 1..=row + 1 {
                    let mut j = if col == 0 { 0 } else { col - 1 };

                    loop {
                        if i == row && j == col {
                            j += 1;
                            continue;
                        } else {
                            let res = check_for_num(&mat, i, j);
                            sum += res.0;
                            j += res.1 - j;
                            if j > col + 1 {
                                break;
                            };
                        }
                    }
                }
            }
        }
    }
    sum
}
fn solve_2(input: &str) -> u32 {
    let mat: Vec<_> = input.split('\n').collect();
    let mut sum = 0;

    for (row, str) in mat.iter().enumerate() {
        for (col, char) in str.chars().enumerate() {
            if char == '*' {
                let mut nums = [0, 0];
                let mut gear_numbers = 0;

                for i in row - 1..=row + 1 {
                    let mut j = if col == 0 { 0 } else { col - 1 };

                    loop {
                        if i == row && j == col {
                            j += 1;
                            continue;
                        } else {
                            let res = check_for_num(&mat, i, j);
                            if res.0 > 0 {
                                nums[gear_numbers] = res.0;
                                gear_numbers += 1;
                            }
                            // It means there were more than 2 numbers adjacent to a gear
                            if gear_numbers > 2 {
                                nums[1] = 0;
                            }

                            j += res.1 - j;
                            if j > col + 1 {
                                break;
                            };
                        }
                    }
                }

                sum += nums[0] * nums[1];
            }
        }
    }
    sum
}

fn check_for_num(mat: &Vec<&str>, row: usize, col: usize) -> (u32, usize) {
    let mut res: u32 = 0;
    let mut left = col;
    let mut right = col + 1;

    if let Some(str) = mat.get(row) {
        let chars: Vec<_> = str.chars().collect();
        if chars[col].is_ascii_digit() {
            while left > 0 && chars[left - 1].is_ascii_digit() {
                left -= 1;
            }
            while right < chars.len() && chars[right].is_ascii_digit() {
                right += 1;
            }

            res = str[left..right].parse().unwrap();
        }
    };

    (res, right)
}
