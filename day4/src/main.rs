use std::{collections::HashMap, vec};

fn main() {
    let input = include_str!("input.txt");

    let mut x = 0;
    let mut y = 0;

    let lut: &HashMap<(i32, i32), char> = &input.chars().fold(HashMap::new(), |mut acc, c| {
        match c {
            'X' | 'M' | 'A' | 'S' => {
                acc.insert((x, y), c);
                x += 1;
            }
            '\n' => {
                y += 1;
                x = 0;
            }
            _ => x += 1,
        }
        acc
    });

    let mut occurances = 0;
    for (k, _) in lut {
        occurances += find_word_occurances(*k, lut, "XMAS");
    }

    println!("Occurances of XMAS {}", occurances);

    let mut cross_count = 0;
    for (k, _) in lut {
        if is_cross(*k, lut) {
            cross_count += 1;
        }
    }

    println!("Cross count {}", cross_count);
}

fn find_word_occurances(pos: (i32, i32), lut: &HashMap<(i32, i32), char>, word: &str) -> i32 {
    if lut.get(&pos) != Some(&word.chars().nth(0).unwrap()) {
        return 0;
    }

    let neighbours = neighbours(pos.0, pos.1, lut);

    let mut count = 0;

    for n in neighbours {
        let direction = (n.0 - pos.0, n.1 - pos.1);

        for i in 1..word.len() {
            let x = pos.0 + direction.0 * i as i32;
            let y = pos.1 + direction.1 * i as i32;

            if let Some(c) = lut.get(&(x, y)) {
                if c != &word.chars().nth(i).unwrap() {
                    break;
                }
                if i == word.len() - 1 {
                    count += 1;
                }
            } else {
                break;
            }
        }
    }

    return count;
}

fn is_cross(pos: (i32, i32), lut: &HashMap<(i32, i32), char>) -> bool {
    if lut.get(&pos) != Some(&'A') {
        return false;
    }

    let neighbours = diagonal_neighbours(pos.0, pos.1, lut);
    if neighbours.len() != 4 {
        return false;
    }

    let mut s = String::new();
    for n in neighbours {
        s.push(*lut.get(&n).unwrap());
    }

    if s == "MSMS" || s == "SMSM" || s == "SSMM" || s == "MMSS" {
        return true;
    }

    return false;
}

fn neighbours(x: i32, y: i32, lut: &HashMap<(i32, i32), char>) -> Vec<(i32, i32)> {
    let mut v = vec![];
    v.push((x + 1, y));
    v.push((x, y + 1));
    v.push((x + 1, y + 1));
    v.push((x - 1, y));
    v.push((x - 1, y + 1));
    v.push((x, y - 1));
    v.push((x + 1, y - 1));
    v.push((x - 1, y - 1));
    v.into_iter().filter(|n| lut.contains_key(n)).collect()
}

fn diagonal_neighbours(x: i32, y: i32, lut: &HashMap<(i32, i32), char>) -> Vec<(i32, i32)> {
    let mut v = vec![];
    v.push((x + 1, y + 1));
    v.push((x - 1, y + 1));
    v.push((x + 1, y - 1));
    v.push((x - 1, y - 1));
    v.into_iter().filter(|n| lut.contains_key(n)).collect()
}
