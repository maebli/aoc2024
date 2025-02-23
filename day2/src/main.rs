fn main() {
    let input = include_str!("input.txt");

    println!("Result1: {:?}", calc_res(1, input));
    println!("Result2: {:?}", calc_res(2, input));
}

fn calc_res(num: i32, input: &str) -> usize {
    input
        .lines()
        .map(|x| {
            let line = x
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            if num == 2 {
                let line_variations = subsets_missing_one(&line);

                for line in line_variations {
                    if check_line(&line) {
                        return true;
                    }
                }
            }
            check_line(&line)
        })
        .filter(|x| *x)
        .count()
}

fn check_line(line: &Vec<i32>) -> bool {
    let x = line
        .windows(2)
        .map(|w| w[0] - w[1])
        .fold((true, 0), |acc, x| {
            ((
                acc.0 && (x.abs() > 0 && x.abs() <= 3),
                acc.1 + ((x >= 0) as u32),
            ))
        });
    x.0 && (x.1 == 0 || x.1 == (line.len() - 1) as u32)
}

fn subsets_missing_one<T: Clone>(input: &Vec<T>) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    for i in 0..input.len() {
        let mut subset = input.clone();
        subset.remove(i); // Remove the element at index `i`
        result.push(subset);
    }
    result
}
