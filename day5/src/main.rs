use std::{collections::VecDeque, ops::Range};

fn main() {
    let input_1 = include_str!("input_1.txt");
    println!("---RESULT_1---");
    let result_1 = solve_1(input_1);
    println!("{result_1}");
    println!("---RESULT_2---");
    let result_2 = solve_2(input_1);
    println!("{result_2}");
}

#[derive(Debug, Clone)]
struct MapRange {
    start: Range<u64>,
    end: Range<u64>,
}

impl PartialEq for MapRange {
    fn eq(&self, other: &Self) -> bool {
        self.start.start == other.start.start
    }
}
impl Eq for MapRange {}
impl PartialOrd for MapRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.start.start.partial_cmp(&other.start.start)
    }
}
impl Ord for MapRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.start.cmp(&other.start.start)
    }
}

fn solve_1(input: &str) -> u64 {
    let mut lines = input.lines();

    let mut seeds: Vec<_> = lines.next().unwrap()[7..]
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    seeds.sort();
    let rules: Vec<_> = lines.skip(1).filter(|l| !l.is_empty()).collect();
    // process the blocks
    let mut blocks: VecDeque<Vec<MapRange>> = VecDeque::new();

    for (_, rule_line) in rules.iter().enumerate() {
        // this is fine since its always ascii chars
        if rule_line[rule_line.len() - 1..rule_line.len()] == *":" {
            // block start
            blocks.push_back(vec![]);
        } else {
            let mut lines = rule_line.split_whitespace();
            let end = lines.next().unwrap().parse::<u64>().unwrap();
            let start = lines.next().unwrap().parse::<u64>().unwrap();
            let span = lines.next().unwrap().parse::<u64>().unwrap();

            if let Some(last_blocks) = blocks.get_mut(blocks.len() - 1) {
                last_blocks.push(MapRange {
                    start: Range {
                        start,
                        end: start + span,
                    },
                    end: Range {
                        start: end,
                        end: end + span,
                    },
                })
            }
        }
    }

    // Consume the blocks and transform the numbers
    while blocks.len() > 0 {
        let mut transform = blocks.pop_front().unwrap();
        let mut t_index = 0;
        transform.sort();
        seeds.sort();
        for seed in seeds.iter_mut() {
            while t_index < transform.len()
                && !transform[t_index].start.contains(seed)
                && transform[t_index].start.end < *seed
            {
                t_index += 1;
            }
            if t_index < transform.len() && transform[t_index].start.contains(seed) {
                *seed = transform[t_index].end.start + (*seed - transform[t_index].start.start)
            } else {
                continue;
            }
        }
    }

    *seeds.iter().min().unwrap()
}

fn solve_2(input: &str) -> u64 {
    let mut lines = input.lines();

    let mut seeds: Vec<Range<u64>> = vec![];
    for (index, str) in lines.next().unwrap()[7..].split_whitespace().enumerate() {
        if index % 2 == 0 {
            seeds.push(Range {
                start: str.parse().unwrap(),
                end: str.parse().unwrap(),
            });
        } else {
            if let Some(last) = seeds.last_mut() {
                last.end += str.parse::<u64>().unwrap();
            }
        }
    }
    seeds.sort_by(|a, b| a.start.cmp(&b.start));

    let rules: Vec<_> = lines.skip(1).filter(|l| !l.is_empty()).collect();
    // // process the blocks
    let mut blocks: VecDeque<Vec<MapRange>> = VecDeque::new();

    for (_, rule_line) in rules.iter().enumerate() {
        // this is fine since its always ascii chars
        if rule_line[rule_line.len() - 1..rule_line.len()] == *":" {
            // block start
            blocks.push_back(vec![]);
        } else {
            let mut lines = rule_line.split_whitespace();
            let end = lines.next().unwrap().parse::<u64>().unwrap();
            let start = lines.next().unwrap().parse::<u64>().unwrap();
            let span = lines.next().unwrap().parse::<u64>().unwrap();

            if let Some(last_blocks) = blocks.get_mut(blocks.len() - 1) {
                last_blocks.push(MapRange {
                    start: Range {
                        start,
                        end: start + span,
                    },
                    end: Range {
                        start: end,
                        end: end + span,
                    },
                })
            }
        }
    }
    // Sort by end
    blocks
        .iter_mut()
        .for_each(|el| el.sort_by(|a, b| a.end.start.cmp(&b.end.start)));

    // Find the shortest path by going in reverse
    let mut result = u64::MAX;
    for num in 0..blocks
        .get(blocks.len() - 1)
        .unwrap()
        .last()
        .unwrap()
        .end
        .end
    {
        let mut curr_num = num;
        let mut blocks_index = (blocks.len() - 1) as i32;
        while blocks_index >= 0 {
            let curr_block = blocks.get(blocks_index as usize).unwrap();

            for rule in curr_block.iter() {
                if curr_num < rule.end.start {
                    break;
                }
                if rule.end.contains(&curr_num) {
                    curr_num = rule.start.start + (curr_num - rule.end.start);
                    break;
                }
            }

            blocks_index -= 1;
        }

        for seed_range in seeds.iter() {
            if seed_range.contains(&curr_num) {
                result = u64::min(result, num);
            };
        }
    }

    result
}
