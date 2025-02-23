
use fxhash::FxHashMap;

fn main() {
    let mut input = include_str!("input.txt").split("\n\n");

    let mut values = input
        .next()
        .unwrap()
        .lines()
        .fold(FxHashMap::default(), |mut lut, l| {
            let mut parts = l.split(": ");
            let left = parts.next().unwrap();
            let right = parts.next().unwrap() == "1";
            lut.insert(left, right);
            lut
        });

    let mut initial_values = values.clone();

    let operations = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let left = parts.next().unwrap();
            let operation = parts.next().unwrap();
            let right = parts.next().unwrap();
            let result = parts.last().unwrap();
            (left, operation, right, result)
        })
        .collect::<Vec<_>>();

    loop {
        let mut changed = false;
        for (left, operation, right, result) in &operations {
            if let (Some(left_value), Some(right_value)) = (values.get(left), values.get(right)) {
                let result_value = match *operation {
                    "AND" => *left_value && *right_value,
                    "OR" => *left_value || *right_value,
                    "XOR" => *left_value ^ *right_value,
                    _ => panic!("Unknown operation"),
                };

                if !values.contains_key(result) {
                    values.insert(result, result_value);
                    changed = true;
                }
            }
        }

        if !changed {
            break;
        }
    }

    let mut sorted_values: Vec<_> = values.iter().collect();
    sorted_values.sort_by(|a, b| a.0.cmp(b.0));

    let res = sorted_values
        .iter()
        .filter(|(k, _v)| k.starts_with("z"))
        .fold((0u64, 0), |(acc, pos), (_k, v)| {
            (acc | (if **v { 1 } else { 0 }) << pos, pos + 1)
        })
        .0;

    println!("Result: {}", res);

    let (x, y) = initial_values.iter().fold((0u64,0u64), |(x, y), (k, v)| {
        let variable = k.chars().next().unwrap();
        let index = k[1..].parse::<u64>().unwrap();
        if *v {
            match variable {
                'x' => (x | (1 << index), y),
                'y' => (x, y | (1 << index)),
                _ => panic!("Unknown variable"),
            }
        } else {
            (x, y)
        }
    });


    let expected = x + y;
    let actual = res ^ (x + y);

    println!("expected: {:b}", expected);
    println!("actual: {:b}", res);
    println!("compare: {:b}", actual);

}

rjm,wsv,z13,z07,swt,pqc,bgs,z31