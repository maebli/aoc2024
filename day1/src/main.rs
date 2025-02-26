#![feature(test)]
extern crate test;
use std::collections::HashMap;


fn run() -> (i32, usize) {
    let input = include_str!("input.txt");
    let numbers = input.split_whitespace();

    let mut left = numbers
        .clone()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, n)| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let numbers = input.split_whitespace();
    let mut right = numbers
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, n)| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    left.sort();
    right.sort();

    let result = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>();


    let mut freq_map: HashMap<i32, usize> = HashMap::new();
    for &elem in &right {
        *freq_map.entry(elem).or_insert(0) += 1;
    }
    
    let result2= left
        .iter()
        .filter_map(|&l| {
            freq_map.get(&l).map(|&count| (l as usize) * count)
        })
        .sum();

    (result, result2)
}

fn main() {
    run();
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_run(b: &mut Bencher) {
        b.iter(|| run());
    }
}