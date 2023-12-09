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

#[derive(Debug)]
enum HandTypes {
    HighCard([Cards; 5]),
    OnePair([Cards; 5]),
    TwoPair([Cards; 5]),
    ThreeOfKind([Cards; 5]),
    FullHouse([Cards; 5]),
    FourOfKind([Cards; 5]),
    FiveOfKind([Cards; 5]),
}

impl HandTypes {
    fn from_str(str: &str) -> Self {
        let cards: Vec<_> = str
            .chars()
            .map(|c| match c {
                'A' => Cards::A,
                'K' => Cards::K,
                'Q' => Cards::Q,
                'J' => Cards::J,
                'T' => Cards::T,
                '9' => Cards::Nine,
                '8' => Cards::Eight,
                '7' => Cards::Seven,
                '6' => Cards::Six,
                '5' => Cards::Five,
                '4' => Cards::Four,
                '3' => Cards::Three,
                '2' => Cards::Two,

                _ => {
                    panic!("Not known char")
                }
            })
            .collect();

        let mut same_types = 0;
        let mut different_types = 0;

        let mut cards_buckets: Vec<Vec<Cards>> = vec![];

        for card in cards.iter() {
            let mut to_add_new = true;
            // this is fine since it's only max 5 cards
            for bucket in cards_buckets.iter_mut() {
                if bucket.contains(card) {
                    bucket.push(card.clone());
                    to_add_new = false;
                }
            }
            if to_add_new {
                cards_buckets.push(vec![card.clone()]);
            }
        }

        for bucket in cards_buckets.iter() {
            if bucket.len() > 1 {
                same_types += 1;
            } else {
                different_types += 1;
            }
        }
        let cards: [Cards; 5] = [
            cards[0].clone(),
            cards[1].clone(),
            cards[2].clone(),
            cards[3].clone(),
            cards[4].clone(),
        ];

        match (same_types, different_types) {
            (1, 0) => HandTypes::FiveOfKind(cards),
            (1, 1) => HandTypes::FourOfKind(cards),
            (2, 0) => HandTypes::FullHouse(cards),
            (1, 2) => HandTypes::ThreeOfKind(cards),
            (2, 1) => HandTypes::TwoPair(cards),
            (1, 3) => HandTypes::OnePair(cards),
            (0, 5) => HandTypes::HighCard(cards),
            _ => panic!("Unknown game type"),
        }
    }
    fn to_number(&self) -> u64 {
        match self {
            Self::HighCard(..) => 0,
            Self::OnePair(..) => 1,
            Self::TwoPair(..) => 2,
            Self::ThreeOfKind(..) => 3,
            Self::FullHouse(..) => 4,
            Self::FourOfKind(..) => 5,
            Self::FiveOfKind(..) => 6,
        }
    }
    fn get_array(&self) -> &[Cards; 5] {
        match self {
            HandTypes::HighCard(a) => a,
            HandTypes::OnePair(a) => a,
            HandTypes::TwoPair(a) => a,
            HandTypes::ThreeOfKind(a) => a,
            HandTypes::FullHouse(a) => a,
            HandTypes::FourOfKind(a) => a,
            HandTypes::FiveOfKind(a) => a,
        }
    }
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.to_number() == other.to_number() {
            let a = self.get_array();
            let b = other.get_array();

            for (i, _) in a.iter().enumerate() {
                if let std::cmp::Ordering::Equal = a[i].cmp(&b[i]) {
                    continue;
                } else {
                    return a[i].cmp(&b[i]);
                }
            }
            std::cmp::Ordering::Equal
        } else {
            self.to_number().cmp(&other.to_number())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Cards {
    A = 12,
    K = 11,
    Q = 10,
    J = 9,
    T = 8,
    Nine = 7,
    Eight = 6,
    Seven = 5,
    Six = 4,
    Five = 3,
    Four = 2,
    Three = 1,
    Two = 0,
}
#[derive(Debug)]
struct Game(HandTypes, u64);

fn solve_1(input: &str) -> Result<u64, Box<dyn Error>> {
    let mut games: Vec<Game> = vec![];

    for line in input.lines() {
        let mut l = line.split_whitespace();
        let hand = l.next().ok_or(AOCError)?;
        let bid = l.next().ok_or(AOCError)?;

        games.push(Game(HandTypes::from_str(hand), bid.parse()?));
    }

    games.sort_by(|a, b| a.0.cmp(&b.0));

    let mut result = 0;

    for (i, game) in games.iter().enumerate() {
        result += (i + 1) as u64 * game.1
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
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

        assert_eq!(solve_1(input).unwrap(), 6440);
    }
    #[test]
    fn test_solve_2() {
        let input = r#""#;
        assert_eq!(solve_2(input).unwrap(), todo!());
    }
}
