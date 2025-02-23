use std::{
    collections::{HashMap, HashSet}, hash::Hash, process::exit, thread, time::Duration, vec
};

const WALL: char = '#';
const BOX: char = 'O';
const ROBOT: char = '@';
const EMPTY: char = '.';
const UP: char = '^';
const DOWN: char = 'v';
const LEFT: char = '<';
const RIGHT: char = '>';
const BIG_BOX_LEFT: char = '[';
const BIG_BOX_RIGHT: char = ']';

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(i32, i32);
#[derive(Debug, Clone, Copy)]
struct Move(i32, i32);
#[derive(Debug)]
struct Warehouse {
    legal: Vec<Pos>,
    walls: HashSet<Pos>,
    boxes: HashSet<Pos>,
    robot: Pos,
    moves: Vec<Move>,
    moves_index: usize,
}


impl Warehouse {
    fn move_robot(&mut self) {
        let Pos(x, y) = self.robot;
        let d = self.moves[self.moves_index];
        let new_pos = Pos(x + d.0, y + d.1);
        if self.legal.contains(&new_pos) {
            if self.walls.contains(&new_pos) {
            } else {
                if !self.is_occupied_by_box(new_pos) {
                    self.robot = new_pos;
                } else {
                    if self.push_box(d, new_pos) {
                        self.robot = new_pos;
                    }
                }
            }
        }
        self.moves_index += 1;
    }

    fn robot_finished(&self) -> bool {
        self.moves_index == self.moves.len()
    }

    fn print(&self) {
        let mut previous_y = 0;
        let mut skip_next = false;
        for Pos(x, y) in self.legal.iter() {
            if previous_y != *y {
                print!("\n");
                previous_y = *y;
            }
            if skip_next {
                skip_next = false;
                continue;
            }
            if self.walls.contains(&Pos(*x, *y)) {
                print!("{}", WALL);
            } else if self.boxes.contains(&Pos(*x, *y)) {
                print!("{}", BIG_BOX_LEFT);
                print!("{}", BIG_BOX_RIGHT);
                skip_next = true;
            } else if self.robot == Pos(*x, *y) {
                print!("{}", ROBOT);
            } else {
                print!("{}", EMPTY);
            }
        }
        print!("\n");
    }

    fn push_box(&mut self, next_move: Move, pos: Pos) -> bool {
        let is_horizontal_push = next_move.0 != 0;

        if is_horizontal_push {
            self.push_box_horizontal(pos, next_move)
        } else {
            self.push_box_vertical(pos, next_move)
        }
    }

    fn push_box_horizontal(&mut self, mut pos: Pos, next_move: Move) -> bool {
        let boxes = self
            .boxes
            .iter()
            .map(|Pos(x, y)| vec![Pos(*x, *y), Pos(*x + 1, *y)])
            .flatten()
            .collect::<Vec<Pos>>();

        let mut box_cluster = vec![];
        while boxes.contains(&pos) {
            box_cluster.push(pos);
            pos = Pos(pos.0+ next_move.0, pos.1);
        }

        if self.walls.contains(&pos) {
            return false;
        }


        let box_cluster = box_cluster.iter().filter(|p| self.boxes.contains(p)).map(|x| *x).collect::<Vec<Pos>>();

        for pos in box_cluster.iter() {
            self.boxes.remove(&pos);
        }
        for pos in box_cluster.iter() {
            self.boxes.insert(Pos(pos.0 + next_move.0, pos.1));
        }

       true
    }

    fn push_box_vertical(&mut self, pos: Pos, next_move: Move) -> bool {
        // get the box we need to push
        let box_to_push = self
            .boxes
            .iter()
            .find(|Pos(x, y)| (*x == pos.0 || (*x + 1) == pos.0) && *y == pos.1)
            .unwrap()
            .clone();

        let cluster = self.get_box_cluster(box_to_push, next_move);
        if self.is_box_cluster_moveable(&cluster, next_move) {
            self.move_cluster(&cluster, next_move);
            return true;
        }
        return false;
    }

    fn is_occupied_by_box(&self, pos: Pos) -> bool {
        self.boxes
            .iter()
            .any(|p| (*p == pos || Pos(p.0 + 1, p.1) == pos))
    }

    fn is_a_left_box(&self, pos: Pos) -> bool {
        self.boxes.iter().any(|p| *p == pos)
    }

    fn get_box_cluster(&self, pos: Pos, mov: Move) -> Vec<Pos> {
        // check its really a box
        if !self.is_occupied_by_box(pos) || self.walls.contains(&pos) || !self.legal.contains(&pos)
        {
            return vec![];
        }

        let mut offset_in_x = 0;
        if !self.is_a_left_box(pos) {
            offset_in_x = -1;
        }
        let mut out = vec![Pos(pos.0 + offset_in_x, pos.1)];

        for i in 0..3 {
            let new_pos = Pos(pos.0 + offset_in_x + i - 1, pos.1 + mov.1);
            if self.boxes.contains(&new_pos) {
                out.extend(self.get_box_cluster(new_pos, mov));
            }
        }
        out
    }

    fn is_box_cluster_moveable(&self, cluster: &Vec<Pos>, mov: Move) -> bool {
        let mut out = true;
        let none_cluster_boxes = self
            .boxes
            .iter()
            .filter(|p| !cluster.contains(p))
            .map(|x| *x)
            .collect::<Vec<Pos>>();
        for pos in cluster {
            for i in 0..2 {
                let new_pos = Pos(pos.0 + mov.0 + i, pos.1 + mov.1);
                if self.walls.contains(&new_pos) || none_cluster_boxes.contains(&new_pos) {
                    out = false;
                    break;
                }

            }
        }
        out
    }

    fn move_cluster(&mut self, cluster: &Vec<Pos>, mov: Move) {
        for pos in cluster {
            self.boxes.remove(&pos);
        }
        for pos in cluster {
            let new_pos = Pos(pos.0 + mov.0, pos.1 + mov.1);
            self.boxes.insert(new_pos);
        }
    }

    fn from(input: &str) -> Self {
        let mut input = input.split("\n\n");

        let mut x = 0;
        let mut y = 0;
        let mut warehouse_map = input.next().unwrap().lines().fold(
            Warehouse {
                walls: HashSet::new(),
                boxes: HashSet::new(),
                robot: Pos(0, 0),
                moves: Vec::new(),
                moves_index: 0,
                legal: Vec::new(),
            },
            |mut warehouse, line| {
                for c in line.chars() {
                    match c {
                        WALL => {
                            warehouse.walls.insert(Pos(x, y));
                            warehouse.walls.insert(Pos(x + 1, y));
                        }
                        BOX => {
                            warehouse.boxes.insert(Pos(x, y));
                        }
                        ROBOT => {
                            warehouse.robot = Pos(x, y);
                        }
                        _ => {}
                    }
                    warehouse.legal.push(Pos(x, y));
                    warehouse.legal.push(Pos(x + 1, y));
                    x += 2;
                }
                y += 1;
                x = 0;
                warehouse
            },
        );
        input.next().unwrap().lines().for_each(|line| {
            line.chars().for_each(|c| match c {
                UP => warehouse_map.moves.push(Move(0, -1)),
                DOWN => warehouse_map.moves.push(Move(0, 1)),
                LEFT => warehouse_map.moves.push(Move(-1, 0)),
                RIGHT => warehouse_map.moves.push(Move(1, 0)),
                _ => {}
            });
        });

        warehouse_map
    }
}

fn main() {
    let input = include_str!("input.txt");

    let mut warehouse_map = Warehouse::from(input);


    while !warehouse_map.robot_finished() {
        warehouse_map.move_robot();
    }

    warehouse_map.print();

    let sol1 = warehouse_map
        .boxes
        .into_iter()
        .fold(0, |acc, Pos(x, y)| {
            acc + y*100 + x
        });


    println!("Solution 1: {}", sol1);

}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("input.txt");

        let mut warehouse_map = Warehouse::from(input);

        while !warehouse_map.robot_finished() {
            warehouse_map.move_robot();
            warehouse_map.print();
        }

        warehouse_map.print();
        let x = &warehouse_map.get_box_cluster(Pos(7, 4), Move(0, -1));

        assert!(warehouse_map.is_box_cluster_moveable(x, Move(0, -1)));

        warehouse_map.move_cluster(x, Move(0, -1));

        warehouse_map.print();
    }
  
  #[test]
  fn test2() {
        let input = include_str!("input.txt");

        let mut warehouse_map = Warehouse::from(input);

        warehouse_map.print();

    }
}
