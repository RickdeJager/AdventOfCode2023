use std::fs::File;
use std::io::{BufRead, BufReader};

type Hand = u32;

#[repr(u8)]
#[derive(Debug)]
enum HandType {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn card_value(c: char, weak_joker: bool) -> u8 {
    match c {
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => match weak_joker {
            true => 0,
            false => 10,
        },
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => unreachable!(),
    }
}

fn get_handtype(hand: &str) -> HandType {
    // Sure, this _looks_ really ugly, but it completely avoid allocations.
    let mut labels = [0; 5];
    labels.copy_from_slice(hand.as_bytes());
    labels.sort_unstable();
    let break_points = labels.windows(2).map(|x| (x[0] != x[1]) as u8).sum();

    match break_points {
        0 => HandType::FiveOfAKind,
        // Four of a kind or Full house
        1 => match labels[1] == labels[3] {
            true => HandType::FourOfAKind,
            false => HandType::FullHouse,
        },
        // TwoPair or three of a kind
        2 => match labels[0] == labels[2] || labels[1] == labels[3] || labels[2] == labels[4] {
            true => HandType::ThreeOfAKind,
            false => HandType::TwoPair,
        },
        3 => HandType::OnePair,
        4 => HandType::HighCard,
        _ => unreachable!(),
    }
}

fn get_handvalue(s: &str, weak_joker: bool) -> Hand {
    let mut ret = 0;
    for c in s.chars() {
        ret <<= 4;
        ret |= card_value(c, weak_joker) as Hand & 0b1111;
    }
    ret
}

fn parse_hand(s: &str, weak_joker: bool) -> Hand {
    let t = get_handtype(s) as Hand;
    let val = get_handvalue(s, weak_joker);

    (t << 20) | val
}

pub fn part1(input: &[String]) -> usize {
    let mut hands: Vec<(Hand, usize)> = input
        .iter()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let hand = parse_hand(hand, false);
            let bid = bid.parse().unwrap();
            (hand, bid)
        })
        .collect();

    hands.sort_unstable_by_key(|(hand, _bid)| *hand);
    hands
        .iter()
        .enumerate()
        .map(|(rank, (_hand, bid))| (rank + 1) * bid)
        .sum()
}

fn update_hand(orig_hand: &str) -> Hand {
    let mut hand = [b'0'; 5];
    hand.copy_from_slice(orig_hand.as_bytes());

    let mut n = 0;
    for elem in &mut hand {
        if *elem == b'J' {
            *elem = b'a' + n;
            n += 1;
        }
    }
    let mut t = get_handtype(std::str::from_utf8(&hand).unwrap());

    if n > 0 {
        t = match t {
            HandType::FiveOfAKind => unreachable!(),
            HandType::FourOfAKind => HandType::FiveOfAKind,
            HandType::FullHouse => unreachable!(),
            HandType::ThreeOfAKind => match n {
                1 => HandType::FourOfAKind,
                2 => HandType::FiveOfAKind,
                _ => unreachable!(),
            },
            HandType::TwoPair => HandType::FullHouse,
            HandType::OnePair => match n {
                1 => HandType::ThreeOfAKind,
                2 => HandType::FourOfAKind,
                3 => HandType::FiveOfAKind,
                _ => unreachable!(),
            },
            HandType::HighCard => match n {
                1 => HandType::OnePair,
                2 => HandType::ThreeOfAKind,
                3 => HandType::FourOfAKind,
                4 => HandType::FiveOfAKind,
                5 => HandType::FiveOfAKind,
                _ => unreachable!(),
            },
        };
    }

    let t = t as Hand;
    let val = get_handvalue(orig_hand, true);
    (t << 20) | val
}

pub fn part2(input: &[String]) -> usize {
    let mut hands: Vec<(Hand, usize)> = input
        .iter()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let hand = update_hand(hand);
            let bid = bid.parse().unwrap();
            (hand, bid)
        })
        .collect();

    hands.sort_unstable_by_key(|(hand, _bid)| *hand);
    hands
        .iter()
        .enumerate()
        .map(|(rank, (_hand, bid))| (rank + 1) * bid)
        .sum()
}

pub fn read_input(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    reader.lines().flatten().collect()
}
