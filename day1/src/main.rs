#![feature(test)]
extern crate test;
use std::collections::HashMap;

fn run() -> (i32, usize) {
    let input = include_str!("input.txt");
    let tokens: Vec<&str> = input.split_whitespace().collect();
    
    let mut left = Vec::with_capacity(tokens.len() / 2 + 1);
    let mut right = Vec::with_capacity(tokens.len() / 2 + 1);

    for (i, &tok) in tokens.iter().enumerate() {
        let val = tok.parse::<i32>().unwrap();
        if i % 2 == 0 {
            left.push(val);
        } else {
            right.push(val);
        }
    }

    left.sort_unstable();
    right.sort_unstable();

    let result = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>();

    let mut freq_map = HashMap::with_capacity(right.len());
    for &r in &right {
        *freq_map.entry(r).or_insert(0) += 1;
    }

    let result2 = left
        .iter()
        .filter_map(|&l| freq_map.get(&l).map(|&count| (l as usize) * count))
        .sum();

    (result, result2)
}

fn main() {
    let (res1, res2) = run();
    println!("Result1: {}, Result2: {}", res1, res2);
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