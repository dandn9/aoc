use std::error::Error;

fn main() {
    let input_1 = include_str!("input_1.txt");
    println!("---RESULT_1---");
    let result_1 = solve_1(input_1);
    println!("{}", result_1.unwrap());
    // println!("---RESULT_2---");
    // let result_2 = solve_2(input_1);
    // println!("{result_2}");
}

#[derive(Debug)]
struct Race {
    time: u32,
    distance_to_beat: u32,
}

// z = num of ms
// k = score to beat
// x(z-x)>k
// => -x2 + zx -k > 0
// (-z +- sq(z2 -4ak)) / -2
fn solve_1(input: &str) -> Result<u32, Box<dyn Error>> {
    let mut lines = input.lines();
    let time = lines.next().ok_or("err")?;
    let distance = lines.next().ok_or("err")?;

    let time: Vec<_> = time.split_whitespace().skip(1).collect();
    let distance: Vec<_> = distance.split_whitespace().skip(1).collect();

    let mut races: Vec<Race> = Vec::with_capacity(time.len());

    for i in 0..time.len() {
        races.push(Race {
            time: time[i].parse()?,
            distance_to_beat: distance[i].parse()?,
        });
    }
    let mut result = 0;

    for race in races.iter() {
        let mb = -(race.time as f32);
        let dv = -2.0;
        let ds = f32::sqrt(
            f32::powi(race.time as f32, 2) + (4.0 * -((race.distance_to_beat + 1) as f32)),
        );

        let x1 = f32::ceil((mb + ds) / dv) as u32;
        let x2 = f32::floor((mb - ds) / dv) as u32;

        let ways_of_beating = x2 - (x1 - 1);

        if result == 0 {
            result = ways_of_beating
        } else {
            result *= ways_of_beating
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve_1() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;

        assert_eq!(solve_1(input).unwrap(), 288);
    }
}
