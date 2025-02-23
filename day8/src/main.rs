use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
struct Roof {
    antennas: HashMap<char, Vec<Position>>,
    spots: Vec<Position>,
}

fn main() {
    let input = include_str!("input.txt");
    let mut x = 0i64;
    let mut y = 0i64;

    let roof = input.chars().fold(
        Roof {
            antennas: HashMap::new(),
            spots: vec![],
        },
        |mut roof, spot| {
            match spot {
                '.' => {
                    roof.spots.push(Position { x, y });
                    x += 1;
                }
                '\n' => {
                    x = 0;
                    y += 1;
                }
                spot if spot.is_alphanumeric() => {
                    let new_spot = Position { x, y };
                    let antenna_positions = roof.antennas.entry(spot).or_insert(vec![]);
                    antenna_positions.push(new_spot.clone());
                    roof.spots.push(new_spot);
                    x += 1;
                }
                _ => panic!("Invalid character"),
            }
            roof
        },
    );

    let anti_nodes =
        roof.antennas
            .into_values()
            .fold(HashSet::new(), |anti_nodes, antenna_positions| {
                let pairs = antenna_positions
                    .iter()
                    .enumerate()
                    .flat_map(|(i, a)| antenna_positions.iter().skip(i + 1).map(move |b| (a, b)));

                let new_anti_nodes = pairs.fold(HashSet::new(), |mut new_anti_nodes, (a, b)| {
                    let mut loop_num = 0;
                    loop {
                        let delta = Position {
                            x: b.x - a.x,
                            y: b.y - a.y,
                        };
                        let (new_a, new_b) = (
                            Position {
                                x: a.x + delta.x * loop_num,
                                y: a.y + delta.y * loop_num,
                            },
                            Position {
                                x: b.x - delta.x * loop_num,
                                y: b.y - delta.y * loop_num,
                            },
                        );

                        let mut new_spots = 0;
                        if roof.spots.contains(&new_a) {
                            new_anti_nodes.insert(new_a);
                            new_spots += 1;
                        }
                        if roof.spots.contains(&new_b) {
                            new_anti_nodes.insert(new_b);
                            new_spots += 1;
                        }

                        if new_spots == 0 {
                            break;
                        }

                        loop_num += 1;
                    }
                    new_anti_nodes
                });

                anti_nodes.union(&new_anti_nodes).cloned().collect()
            });

    println!("{:?}", anti_nodes.len());
}
