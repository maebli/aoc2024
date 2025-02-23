use std::{collections::HashMap, hash::Hash, vec};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Stone(u64);

#[derive(Debug)]
struct Rule {
    apply: fn(Stone) -> Vec<Stone>,
    applies: fn(Stone) -> bool,
}

impl Stone {
    fn split(&self) -> Vec<Stone> {
        let num_digits = self.0.to_string().len();
        let split_position = num_digits / 2;

        let divisor = 10u64.pow((num_digits - split_position) as u32);
        let first_part = self.0 / divisor;
        let second_part = self.0 % divisor;
        vec![Stone(first_part), Stone(second_part)]
    }
}

#[derive(Debug)]
struct TreeNode {
    value: Stone,
    left: Option<Stone>,
    right: Option<Stone>,
}

fn main() {
    let rules = vec![
        Rule {
            apply: |_| vec![Stone(1)],
            applies: |x| x.0 == 0,
        },
        Rule {
            apply: |x| x.split(),
            applies: |x| x.0.to_string().len() % 2 == 0,
        },
    ];

    let input = include_str!("input.txt")
        .split(" ")
        .map(|x| Stone(x.parse().unwrap()))
        .collect::<Vec<Stone>>();

    let blink_number = 75;

    println!("{:?}", input);

    let mut counter = HashMap::<Stone, u64>::new();

    for stone in input.iter() {
        *counter.entry(*stone).or_insert(0) += 1;
    }

    for _ in 0..blink_number {
        let mut new_counter = HashMap::new();

        for (stone, &count) in counter.iter() {
            let new_stones = rules
                .iter()
                .find(|rule| (rule.applies)(*stone))
                .map(|rule| (rule.apply)(*stone))
                .unwrap_or_else(|| vec![Stone(2024 * stone.0)]);

            for new_stone in new_stones {
                *new_counter.entry(new_stone).or_insert(0) += count;
            }
        }

        counter = new_counter;
    }

    println!("{:?}", counter.iter().map(|(_, v)| v).sum::<u64>());
}
