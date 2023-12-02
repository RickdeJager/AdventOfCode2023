use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default)]
struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

impl Hand {
    fn from_str(hand: &str) -> Hand {
        let mut ret = Hand::default();

        hand.split(", ").for_each(|group| {
            let (num, color) = group.split_once(' ').unwrap();
            let num = num.parse::<u32>().unwrap();
            match color {
                "red" => ret.red = num,
                "green" => ret.green = num,
                "blue" => ret.blue = num,
                _ => unreachable!(),
            };
        });

        ret
    }
}

#[derive(Debug, Default)]
pub struct Game {
    id: usize,
    hands: Vec<Hand>,
}

impl Game {
    fn from_line(line: &str) -> Game {
        let (identifier, game) = line.split_once(": ").unwrap();

        let id = identifier
            .strip_prefix("Game ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let hands = game.split("; ").map(Hand::from_str).collect::<Vec<Hand>>();

        Game { id, hands }
    }
}

pub fn part1(input: &Vec<Game>) -> usize {
    let max_hand = Hand {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut result = 0;

    'game: for game in input {
        for hand in &game.hands {
            if max_hand.blue < hand.blue || max_hand.red < hand.red || max_hand.green < hand.green {
                continue 'game;
            }
        }
        result += game.id;
    }

    result
}

pub fn part2(input: &Vec<Game>) -> usize {
    let mut result = 0;
    for game in input {
        let mut min_hand = Hand::default();
        for hand in &game.hands {
            if hand.red > min_hand.red {
                min_hand.red = hand.red;
            }

            if hand.blue > min_hand.blue {
                min_hand.blue = hand.blue;
            }

            if hand.green > min_hand.green {
                min_hand.green = hand.green;
            }
        }
        result += (min_hand.blue * min_hand.red * min_hand.green) as usize;
    }

    result
}

pub fn read_input(filename: &str) -> Vec<Game> {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .flatten()
        .map(|line| Game::from_line(&line))
        .collect()
}
