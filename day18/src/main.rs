use pathfinding::prelude::dijkstra;
use std::{collections::HashSet, vec};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

impl From<&str> for Pos {
    fn from(s: &str) -> Self {
        let mut parts = s.split(',');
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        Pos(x, y)
    }
}

#[derive(Debug)]
struct Field {
    obstacles: HashSet<Pos>,
    start: Pos,
    end: Pos,
}

impl Field {
    fn legal_move(&self, pos: Pos) -> bool {
        pos.0 >= 0
            && pos.0 <= self.end.0
            && pos.1 >= 0
            && pos.1 <= self.end.1
            && !self.obstacles.contains(&pos)
    }
}

impl Pos {
    fn successors(&self, field: &Field) -> Vec<(Pos, usize)> {
        let mut moves = vec![];
        for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos = Pos(self.0 + dx, self.1 + dy);
            if field.legal_move(new_pos) {
                moves.push(new_pos);
            }
        }
        moves.into_iter().map(|p| (p, 1)).collect()
    }
}

fn main() {
    let input = include_str!("input.txt");
    let x = input.lines().map(|l| Pos::from(l)).collect::<Vec<_>>();
    let mut obstacles = x.into_iter().rev().collect::<Vec<_>>();

    let mut field = Field {
        obstacles: vec![].into_iter().collect(),
        start: Pos(0, 0),
        end: Pos(70, 70),
    };

    let mut next_byte = Pos(0, 0);
    while let Some(_) = dijkstra(&field.start, |p| p.successors(&field), |p| *p == field.end) {
        next_byte = obstacles.pop().unwrap().clone();
        field.obstacles.insert(next_byte);
    }

    println!("{:?}", next_byte);
}
