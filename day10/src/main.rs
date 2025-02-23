#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

#[derive(Debug, Clone)]
struct Movement {
    dx: i64,
    dy: i64,
    dz: i64,
}

#[derive(Debug, Clone)]
struct CloudMap {
    points: Vec<Point>,
    trailheads: Vec<Point>,
    collected_tops: Vec<Point>,
}

fn main() {
    let input = include_str!("input.txt");

    let mut x = 0;
    let mut y = 0;
    let mut map = input.chars().fold(
        CloudMap {
            points: vec![],
            trailheads: vec![],
            collected_tops: vec![],
        },
        |mut map, c| {
            match c {
                height if height.is_numeric() => {
                    let z = height.to_string().parse::<u64>().unwrap();
                    assert!(z <= 9);
                    map.points.push(Point { x, y, z });

                    if z == 0 {
                        map.trailheads.push(Point { x, y, z });
                    }
                    x += 1;
                }
                '\n' => {
                    x = 0;
                    y += 1;
                }
                _ => {
                    x += 1;
                }
            }
            map
        },
    );

    let out = map
        .trailheads
        .iter()
        .map(|trailhead| trailhead_score(trailhead, &mut map.clone()))
        .sum::<u64>();

    println!("{}", out);
}

fn trailhead_score(trailhead: &Point, map: &mut CloudMap) -> u64 {
    if trailhead.z == 9 && map.collected_tops.iter().all(|p| *p != *trailhead) {
        map.collected_tops.push(trailhead.clone());
        return 1;
    }

    let mut score = 0;

    let legal_deltas = vec![
        Movement {
            dx: 0,
            dy: 1,
            dz: 1,
        },
        Movement {
            dx: 1,
            dy: 0,
            dz: 1,
        },
        Movement {
            dx: 0,
            dy: -1,
            dz: 1,
        },
        Movement {
            dx: -1,
            dy: 0,
            dz: 1,
        },
    ];

    for delta in legal_deltas {
        let new_trailhead = Point {
            x: (trailhead.x as i64 + delta.dx) as u64,
            y: (trailhead.y as i64 + delta.dy) as u64,
            z: trailhead.z + delta.dz as u64,
        };
        if map.points.iter().any(|p| *p == new_trailhead) {
            score += trailhead_score(&new_trailhead, map);
        }
    }
    score
}
