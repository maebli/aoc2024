use pathfinding::prelude::dijkstra;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct ReindeerArena {
    legal: Vec<Pos>,
    illegal: Vec<Pos>,
    end: Pos,
    start: Pos,
}

impl ReindeerArena {
    fn is_legal(&self, pos: &Pos) -> bool {
        self.legal.contains(pos)
    }
}

fn successors(pos: &Pos, arena: &ReindeerArena) -> Vec<(Pos, i32)> {
    let mut out = vec![];

    let Pos(x, y) = pos;
    let legal_jumps = vec![Pos(1, 0), Pos(-1, 0), Pos(0, 1), Pos(0, -1)];

    for jump in legal_jumps.iter() {
        let new_pos = Pos(x + jump.0, y + jump.1);
        if arena.is_legal(&new_pos) {
            out.push((new_pos, 1));
        }
    }
    out
}

fn manhattan_distance(a: &Pos, b: &Pos) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}


fn main() {
    let input = include_str!("input.txt");

    let mut x = 0;
    let mut y = 0;

    let arena = input.chars().fold(
        ReindeerArena {
            legal: vec![],
            illegal: vec![],
            end: Pos(0, 0),
            start: Pos(0, 0),
        },
        |mut arena, c| {
            match c {
                '.' => {
                    arena.legal.push(Pos(x, y));
                    x += 1;
                }
                '#' => {
                    x += 1;
                    arena.illegal.push(Pos(x, y));
                }
                '\n' => {
                    x = 0;
                    y += 1;
                }
                'S' => {
                    arena.start = Pos(x, y);
                    arena.legal.push(Pos(x, y));
                    x += 1;
                }
                'E' => {
                    arena.end = Pos(x, y);
                    arena.legal.push(Pos(x, y));
                    x += 1;
                }
                _ => (),
            }
            arena
        },
    );

    let best_result_wihout_cheating = dijkstra(
        &arena.start,
        |pos| successors(pos, &arena),
        |pos| *pos == arena.end,
    )
    .unwrap();

    let best_path = best_result_wihout_cheating.0.clone();

    let mut path_count = 0;
    let save_threshold = 100;
    
    for i in 0..best_path.len() {
        let mut skip_candidates = vec![];

        for j in (i + 1)..best_path.len() {
            let distance = manhattan_distance(&best_path[i], &best_path[j]);
            let normal_steps = (j - i) as i32;
            let skip_saving = normal_steps - distance;

            if skip_saving >= save_threshold && distance <= 20 {
                skip_candidates.push(j);
            }
        }

        path_count += skip_candidates.len();
    }

    println!("Number of paths: {}", path_count);
}


