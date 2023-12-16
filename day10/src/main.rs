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
type Coord = (i32, i32);

fn solve_1(input: &str) -> Result<i64, Box<dyn Error>> {
    let lines = input.lines();
    let matrix: Vec<Vec<char>> = lines
        .map(|line| line.chars().filter(|c| !c.is_whitespace()).collect())
        .collect();
    let mut starting_point: Coord = (0, 0);
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if matrix[y][x] == 'S' {
                starting_point = (x as i32, y as i32);
            }
        }
    }

    let offsets: [Coord; 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let pipes = HashMap::from([
        ('|', (0.0, 0.5)),
        ('-', (0.5, 0.0)),
        ('L', (1.0, -1.0)),
        ('J', (-1.0, -1.0)),
        ('7', (-1.0, 1.0)),
        ('F', (1.0, 1.0)),
        ('.', (0.0, 0.0)),
    ]);

    let mut offset_index = 0;

    let mut direction = offsets[0].clone();
    let mut prev_point = starting_point;
    let mut next_point = (prev_point.0 + direction.0, prev_point.1 + direction.1);

    let mut walk_scores: Vec<u32> = vec![];

    while offset_index < offsets.len() {
        if next_point.1 < 0
            || next_point.1 >= matrix.len() as i32
            || next_point.0 < 0
            || next_point.0 >= matrix[next_point.1 as usize].len() as i32
        {
            offset_index += 1;
            walk_scores = vec![];
            continue;
        }

        walk_scores.push(*walk_scores.last().unwrap_or(&0) + 1);

        let next_char = &matrix[next_point.1 as usize][next_point.0 as usize];
        if *next_char == 'S' {
            break;
        }

        if let Some(next_dir) = pipes.get(next_char) {
            direction = (
                i32::min(f64::round(next_dir.0 - (-direction.0 as f64)) as i32, 1),
                i32::min(f64::round(next_dir.1 - (-direction.1 as f64)) as i32, 1),
            );
            prev_point = next_point;
            next_point = (prev_point.0 + direction.0, prev_point.1 + direction.1)
        };
    }

    let median = f32::floor(walk_scores.len() as f32 / 2.0 - 1.0) as i64;

    Ok(walk_scores[median as usize] as i64)

    // now we should
}
fn solve_2(input: &str) -> Result<i64, Box<dyn Error>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve_1() {
        let input_1 = r#"   -L|F7
                            7S-7|
                            L|7||
                            -L-J|
                            L|-JF"#;

        assert_eq!(solve_1(input_1).unwrap(), 4);
        let input_2 = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#;

        assert_eq!(solve_1(input_2).unwrap(), 8);
    }
    #[test]
    fn test_solve_2() {
        let input = r#""#;
        assert_eq!(solve_2(input).unwrap(), todo!());
    }
}
