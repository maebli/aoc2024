fn main() {
    let input = include_str!("input.txt").split("\n\n");

    let (locks, key) =input.fold((vec![],vec![]),|mut acc, x| {
        let lines = x.lines().collect::<Vec<_>>();

        let mut res = [0;5];

        for l in &lines {
            for row in 0..5 {
                if l.chars().nth(row).unwrap() == '#' {
                    res[row] += 1;
                }
            }
        }

        if lines[0] == "....."{
            acc.1.push(res);
        } else {
            acc.0.push(res);
        }
        acc
    });


    let mut matches = 0;

    for key in key.iter() {
        for lock in locks.iter() {
            let mut found = true;
            for i in 0..5 {
                if key[i] + lock[i] > 7 {
                    found = false;
                    break;
                }
            }
            if found {
                matches += 1;
            }
        }
    }

    println!("{}", matches);
}
