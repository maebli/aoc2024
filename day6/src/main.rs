#[derive(Debug, Clone)]
struct State {
    pos: (i32, i32),
    dir: (i32, i32),
}

#[derive(Debug, Clone)]
struct Game {
    moves: i32,
    visited: Vec<State>,
    guard: Guard,
    obstacles: Vec<(i32, i32)>,
    legal_moves: Vec<(i32, i32)>,
}

#[derive(Debug, Clone)]
struct Guard {
    pos: (i32, i32),
    dir: (i32, i32),
    start: (i32, i32),
}

#[derive(Debug, PartialEq, Clone)]
enum GameState {
    Running,
    GuardExited,
    GuardLooped,
}

impl Game {
    fn move_guard(&mut self) -> GameState {
        if self
            .visited
            .iter()
            .any(|x| x.pos == self.guard.pos && x.dir == self.guard.dir)
        {
            return GameState::GuardLooped;
        }

        let state = State {
            pos: self.guard.pos,
            dir: self.guard.dir,
        };
        self.visited.push(state);

        let mut next_pos_in_same_dir = (
            self.guard.pos.0 + self.guard.dir.0,
            self.guard.pos.1 + self.guard.dir.1,
        );

        while self.obstacles.contains(&next_pos_in_same_dir) {
            // rotate 90 degrees to the right
            self.guard.dir = (-self.guard.dir.1, self.guard.dir.0);
            next_pos_in_same_dir = (
                self.guard.pos.0 + self.guard.dir.0,
                self.guard.pos.1 + self.guard.dir.1,
            );
        }

        self.guard.pos = next_pos_in_same_dir;

        if self.legal_moves.contains(&next_pos_in_same_dir) {
            self.moves += 1;
            return GameState::Running;
        }

        GameState::GuardExited
    }
}
fn main() {
    let input = include_str!("input.txt");

    let mut pos = (0, 0);
    let game = input.chars().fold(
        Game {
            moves: 0,
            visited: vec![],
            legal_moves: vec![],
            guard: Guard {
                pos: (0, 0),
                dir: (0, 0),
                start: (0, 0),
            },
            obstacles: vec![],
        },
        |mut acc, x| {
            acc.legal_moves.push(pos);
            match x {
                '#' => {
                    acc.obstacles.push(pos);
                    pos = (pos.0 + 1, pos.1);
                }
                '^' => {
                    acc.guard.start = pos;
                    acc.guard.pos = pos;
                    acc.guard.dir = (0, -1);
                    pos = (pos.0 + 1, pos.1);
                }
                'v' => {
                    acc.guard.start = pos;
                    acc.guard.pos = pos;
                    acc.guard.dir = (0, 1);
                    pos = (pos.0 + 1, pos.1);
                }
                '<' => {
                    acc.guard.start = pos;
                    acc.guard.pos = pos;
                    acc.guard.dir = (-1, 0);
                    pos = (pos.0 + 1, pos.1);
                }
                '>' => {
                    acc.guard.start = pos;
                    acc.guard.pos = pos;
                    acc.guard.dir = (1, 0);
                    pos = (pos.0 + 1, pos.1);
                }
                '\n' => pos = (0, pos.1 + 1),
                '.' => pos = (pos.0 + 1, pos.1),
                _ => panic!("Invalid character"),
            };
            acc
        },
    );

    let mut game1 = game.clone();
    while game1.move_guard() == GameState::Running {}

    let visited = game1.visited.iter().collect::<Vec<&State>>();

    let mut x = visited
        .iter()
        .map(|x| {
            let mut game2 = game.clone();
            if x.pos == game2.guard.start {
                return (GameState::GuardExited, x.pos);
            }
            game2.obstacles.push(x.pos);
            let mut game_state = GameState::Running;
            while game_state == GameState::Running {
                game_state = game2.move_guard();
            }
            game2.obstacles.pop();
            (game_state, x.pos)
        })
        .filter(|x| &x.0 == &GameState::GuardLooped)
        .map(|x| x.1)
        .collect::<Vec<(i32, i32)>>();

    x.sort();
    x.dedup();

    println!("{:?}", x.len());
}
