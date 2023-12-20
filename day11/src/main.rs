use std::{collections::HashMap, error::Error};

#[derive(Debug)]
struct AOCError;
impl std::fmt::Display for AOCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AOCError")
    }
}
impl Error for AOCError {}

fn main() {
    let input_1 = include_str!("input_1.txt");
    println!("---RESULT_1---");
    let result_1 = solve_1(input_1);
    println!("{}", result_1.unwrap());
    println!("---RESULT_2---");
    let result_2 = solve_2(input_1);
    println!("{}", result_2.unwrap());
}

fn solve_1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut lines: Vec<_> = input
        .lines()
        .map(|line| {
            if line.chars().all(|c| c == '.') {
                vec![line.to_string(), line.to_string()]
            } else {
                vec![line.to_string()]
            }
        })
        .flatten()
        .collect();

    let mut added = 0;
    'm: for x in 0..lines[0].len() {
        let x = x + added;
        for y in 0..lines.len() {
            if lines[y].chars().nth(x).unwrap() == '#' {
                continue 'm;
            }
        }
        for y in 0..lines.len() {
            lines[y] = format!("{}.{}", &lines[y][0..=x], &lines[y][x + 1..]);
        }
        added += 1;
    }

    let mut star_count = 0;
    let mut stars_pos: HashMap<u32, (u32, u32)> = HashMap::new();

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y].chars().nth(x).unwrap() == '#' {
                star_count += 1;
                stars_pos.insert(star_count, (y as u32, x as u32));
            }
        }
    }

    let mut sum = 0;

    for star in 1..star_count {
        for other_star in star + 1..=star_count {
            let a = stars_pos.get(&star).unwrap();
            let b = stars_pos.get(&other_star).unwrap();
            sum += (i32::abs((b.1 as i32) - a.1 as i32) + i32::abs(b.0 as i32 - a.0 as i32)) as u32;
        }
    }

    Ok(sum as i64)
}
fn solve_2(input: &str) -> Result<i64, Box<dyn Error>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve_1() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

        assert_eq!(solve_1(input).unwrap(), 374);
    }
    #[test]
    fn test_solve_2() {
        let input = r#""#;
        assert_eq!(solve_2(input).unwrap(), todo!());
    }
}
