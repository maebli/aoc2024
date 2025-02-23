use itertools::Itertools;

fn main() {
    let symbols = vec![0, 1, 3];
    let input = include_str!("input.txt");

    let x = input
        .lines()
        .map(|line| {
            let line = line.replace(":", "");
            let mut line = line.split(" ").collect::<Vec<&str>>();
            let result = line.remove(0).parse::<i64>().unwrap();
            let numbers = line
                .iter()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            (result, numbers)
        })
        .fold(0, |acc, x| {
            let permutations = std::iter::repeat(symbols.clone())
                .take(x.1.len())
                .multi_cartesian_product();

            for perm in permutations {
                let values: Vec<_> = perm;
                let sum = values.iter().enumerate().fold(0, |acc, k| match k.1 {
                    0 => acc + x.1[k.0],
                    1 => acc * x.1[k.0],
                    3 => {
                        let digits = count_digits(x.1[k.0]);
                        acc * 10i64.pow(digits) + x.1[k.0]
                    }
                    _ => panic!("Invalid symbol"),
                });

                if sum == x.0 {
                    return acc + sum;
                }
            }
            acc
        });

    println!("x: {:?}", x);
}

fn count_digits(n: i64) -> u32 {
    if n == 0 {
        1
    } else {
        (n.abs() as f64).log10().floor() as u32 + 1
    }
}
