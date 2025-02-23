use std::collections::HashMap;

fn main() {
    let seq = vec![
        123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
        5908254,
    ];

    for i in 0..seq.len() - 1 {
        assert_eq!(evolve_once(seq[i]), seq[i + 1]);
    }

    let input = include_str!("input.txt");

    let secrets = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let sum = secrets
        .iter()
        .map(|&secret| evolve_repeated_loop(secret, 2000))
        .sum::<i64>();
    println!("solution part 1: {}", sum);

    let mut banana_count = HashMap::<(i64, i64, i64, i64), i64>::new();

    let sol = secrets
        .iter()
        .fold(& mut banana_count, | current_banana_count, &secret| {
            evolution(current_banana_count, secret, 2000)
        })
        .iter()
        .max_by_key(|&(_, &count)| count)
        .unwrap();

    println!("solution part 2: {:?}", sol);


}

fn evolve_once(mut secret_number: i64) -> i64 {
    let tmp = secret_number.wrapping_mul(64);
    secret_number ^= tmp;
    secret_number %= 16777216;

    let tmp = secret_number / 32;
    secret_number ^= tmp;
    secret_number %= 16777216;

    let tmp = secret_number.wrapping_mul(2048);
    secret_number ^= tmp;
    secret_number %= 16777216;

    secret_number
}

fn evolve_repeated_loop(mut secret_number: i64, times: i64) -> i64 {
    for _ in 0..times {
        secret_number = evolve_once(secret_number);
    }
    secret_number
}


fn evolution(
    lut: &mut HashMap<(i64, i64, i64, i64), i64>,
    mut secret_number: i64,
    times: usize,
) -> &mut HashMap<(i64, i64, i64, i64), i64> {
    let mut vec = Vec::with_capacity(times);
    for _ in 0..times {
        secret_number = evolve_once(secret_number);
        vec.push(secret_number % 10);
    }
    
    let mut local_lookup = HashMap::new();
    
    for window in vec.windows(5) {
        let key = (
            window[1] - window[0],
            window[2] - window[1],
            window[3] - window[2],
            window[4] - window[3],
        );
        let value = window[4];
        
        if !local_lookup.contains_key(&key) {
            *lut.entry(key).or_insert(0) += value;
            local_lookup.insert(key, value);
        }
    }
    
    lut
}