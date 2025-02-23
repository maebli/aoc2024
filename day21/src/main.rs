use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos(i32, i32);

fn main() {
    let input = include_str!("input.txt");

    let mut res = 0u64;
    for line in input.lines() {
        let num = line
            .chars()
            .take(3)
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        let seq = format!("A{}", line);
        let mut complexity_sum = 0;
        for (a, b) in seq.chars().tuple_windows() {
            complexity_sum += complexity(a, b, 26);
        }

        res += complexity_sum * num;

        println!("{}", complexity_sum);
    }

    println!("{}", res);
}
#[cached]
fn complexity(from: char, to: char, mut robot_count: i32) -> u64 {
    robot_count -= 1;

    #[rustfmt::skip]
    let dir_layout: HashMap<char, Pos> = [
                          ('^', Pos(1, 0)), ('A', Pos(2, 0)),
        ('<', Pos(0, 1)), ('v', Pos(1, 1)), ('>', Pos(2, 1))
    ].into_iter().collect();

    #[rustfmt::skip]
    let num_layout: HashMap<char, Pos> = [
        ('7', Pos(0,0)), ('8', Pos(1,0)), ('9', Pos(2,0)),
        ('4', Pos(0,1)), ('5', Pos(1,1)), ('6', Pos(2,1)),
        ('1', Pos(0,2)), ('2', Pos(1,2)), ('3', Pos(2,2)),
                         ('0', Pos(1,3)), ('A', Pos(2,3))
    ].into_iter().collect();

    let layout = if dir_layout.contains_key(&from) && dir_layout.contains_key(&to) {
        &dir_layout
    } else {
        &num_layout
    };

    let start = layout.get(&from).unwrap();
    let end = layout.get(&to).unwrap();

    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    // Directions to move
    let dx_dir = if dx > 0 { ">" } else { "<" };
    let dy_dir = if dy > 0 { "v" } else { "^" };

    // Create the steps vector
    let mut steps = vec![];
    for _ in 0..dx.abs() {
        steps.push(dx_dir);
    }
    for _ in 0..dy.abs() {
        steps.push(dy_dir);
    }

    let mut paths = vec![];
    let mut stack = vec![(vec![], steps.clone())];

    while let Some((mut current_path, remaining_steps)) = stack.pop() {
        if remaining_steps.is_empty() {
            current_path.push("A"); // Add termination symbol
            paths.push(current_path.join(""));
        } else {
            for i in 0..remaining_steps.len() {
                let mut next_path = current_path.clone();
                next_path.push(remaining_steps[i]);

                let mut next_steps = remaining_steps.clone();
                next_steps.remove(i);

                stack.push((next_path, next_steps));
            }
        }
    }

    paths = paths
        .iter()
        .map(|path| {
            let mut current_pos = *start;
            let mut valid = true;

            for c in path.chars() {
                println!(" pos {:?} c {}", current_pos, c);
                match c {
                    '^' => current_pos.1 -= 1,
                    'v' => current_pos.1 += 1,
                    '<' => current_pos.0 -= 1,
                    '>' => current_pos.0 += 1,
                    _ => (),
                }
                if !layout.values().any(|&pos| pos == current_pos) {
                    println!("invalid pos {:?}", current_pos);
                    valid = false;
                    break;
                }
            }
            (valid, path)
        })
        .filter(|(valid, _)| *valid)
        .map(|(_, path)| path.clone())
        .collect::<Vec<String>>();

    if dx == 0 && dy == 0 {
        paths.push("A".to_string());
    }

    if robot_count > 0 {
        return paths
            .iter()
            .map(|path| {
                // prepend 'A' to the path
                let path = format!("A{}", path);
                path.chars()
                    .tuple_windows()
                    .map(|(a, b)| complexity(a, b, robot_count))
                    .sum::<u64>()
            })
            .min()
            .expect("Problem finding min");
    }
    paths
        .iter()
        .map(|path| path.len() as u64)
        .min()
        .expect("Problem finding min")
}
