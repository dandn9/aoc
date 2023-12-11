use std::error::Error;

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
    let sequences: Vec<_> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut result = 0;

    for sequence in sequences.iter() {
        let mut tree = vec![sequence.clone()];
        let mut curr_vec: Vec<i64> = vec![];

        loop {
            let last_tree = &tree[tree.len() - 1];
            for i in 1..last_tree.len() {
                let delta = last_tree[i] - last_tree[i - 1];
                curr_vec.push(delta);
            }

            if curr_vec.iter().all(|el| *el == 0) {
                tree.push(curr_vec.clone());
                break;
            } else {
                tree.push(curr_vec.clone());
                curr_vec = vec![];
            }
        }

        let mut prev_sequence: Option<Vec<i64>> = None;

        for (i, sequence) in tree.iter_mut().rev().enumerate() {
            if i == 0 {
                sequence.push(0);
            } else {
                let last_elem_of_prev = {
                    let prev_sequence = prev_sequence.as_ref().unwrap();
                    prev_sequence.last().unwrap().clone()
                };
                sequence.push(sequence.last().unwrap() + last_elem_of_prev);
            }
            prev_sequence = Some(sequence.clone());
        }

        result += tree.first().unwrap().last().unwrap();
    }

    Ok(result)
}
fn solve_2(input: &str) -> Result<u64, Box<dyn Error>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve_1() {
        let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
        assert_eq!(solve_1(input).unwrap(), 114);
    }
    #[test]
    fn test_solve_2() {
        let input = r#""#;
        assert_eq!(solve_2(input).unwrap(), todo!());
    }
}
