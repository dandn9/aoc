use std::ops::Range;

fn main() {
    let input_1 = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    println!("---RESULT_1---");
    let result_1 = solve_1(input_1);
    println!("{result_1}");
    println!("---RESULT_2---");
}

#[derive(Debug, Clone)]
struct MapRange {
    start: Range<u32>,
    end: Range<u32>,
}

impl PartialEq for MapRange {
    fn eq(&self, other: &Self) -> bool {
        // no idea why i have to clone :\
        self.start.clone().eq(other.start.clone())
    }
}
impl Eq for MapRange {}
impl PartialOrd for MapRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // no idea why i have to clone :\
        Some(self.start.clone().cmp(other.start.clone()))
    }
}
impl Ord for MapRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp(other)
    }
}

fn solve_1(input: &str) -> u32 {
    let mut result = 0;
    let mut lines = input.lines();

    let mut seeds: Vec<_> = lines.next().unwrap()[7..]
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    seeds.sort();
    let rules: Vec<_> = lines.skip(1).filter(|l| !l.is_empty()).collect();
    // process the blocks
    let mut blocks: Vec<Vec<MapRange>> = vec![];

    for rule_line in rules.iter() {
        // this is fine since its always ascii chars
        if rule_line[rule_line.len() - 1..rule_line.len()] == *":" {
            // block start
            if let Some(last_blocks) = blocks.last_mut() {
                last_blocks.sort()
            }
            blocks.push(vec![]);
        } else {
            let mut lines = rule_line.split_whitespace();
            let end = lines.next().unwrap().parse::<u32>().unwrap();
            let start = lines.next().unwrap().parse::<u32>().unwrap();
            let span = lines.next().unwrap().parse::<u32>().unwrap();

            if let Some(last_blocks) = blocks.last_mut() {
                last_blocks.push(MapRange {
                    start: Range {
                        start: start,
                        end: start + span + 1,
                    },
                    end: Range {
                        start: end,
                        end: end + span + 1,
                    },
                })
            }
        }
    }
    println!("{blocks:?} last bock");

    result
}
