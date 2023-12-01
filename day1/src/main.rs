fn main() {
    let part1 = include_str!("input1.txt");
    let res1 = solve_part1(part1);
    println!("----RES1----");
    println!("{}", res1);

    let part2 = include_str!("input1.txt");
    let res2 = solve_part2(part2);
    println!("----RES2----");
    println!("{}", res2);
}

fn solve_part1(str: &str) -> u32 {
    let mut calibrations: Vec<u32> = vec![];

    for line in str.split('\n') {
        let chars: Vec<_> = line.chars().collect();

        if chars.len() < 1 {
            continue;
        }

        let mut l = 0;
        let mut r = chars.len() - 1;

        while l < r {
            let ld = chars[l].is_ascii_digit();
            let rd = chars[r].is_ascii_digit();

            if !ld {
                l += 1;
            }
            if !rd {
                r -= 1;
            }

            if ld && rd {
                break;
            }
        }

        let res = format!("{}{}", chars[l], chars[r]);
        calibrations.push(res.parse().unwrap())
    }

    calibrations.iter().sum()
}

fn solve_part2(str: &str) -> u32 {
    let mut calibrations: Vec<u32> = vec![];

    for line in str.split('\n') {
        let mut left_most = String::from("");
        let mut right_most = String::from("");

        for i in 0..line.len() {
            let r: i32 = {
                if let Ok(num) = &line[i..=i].parse::<i32>() {
                    *num
                } else if line[i..].starts_with("zero") {
                    0
                } else if line[i..].starts_with("one") {
                    1
                } else if line[i..].starts_with("two") {
                    2
                } else if line[i..].starts_with("three") {
                    3
                } else if line[i..].starts_with("four") {
                    4
                } else if line[i..].starts_with("five") {
                    5
                } else if line[i..].starts_with("six") {
                    6
                } else if line[i..].starts_with("seven") {
                    7
                } else if line[i..].starts_with("eight") {
                    8
                } else if line[i..].starts_with("nine") {
                    9
                } else {
                    -1
                }
            };

            if r >= 0 {
                if left_most == "" {
                    left_most = r.to_string();
                }
                right_most = r.to_string()
            }
        }
        calibrations.push(format!("{}{}", left_most, right_most).parse().unwrap());
    }
    calibrations.iter().sum()
}
