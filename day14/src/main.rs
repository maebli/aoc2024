
use std::io::Write;
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
struct Pos(u32, u32);

#[derive(Debug, Clone)]
struct Velocity(i32, i32);

#[derive(Debug, Clone)]
struct Robot {
    pos: Pos,
    vel: Velocity,
}

enum Quadrant {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    None,
}

impl Robot {
    fn next(&mut self, max_pos: &Pos) {
        let max_x = max_pos.0 as i32 + 1;
        let max_y = max_pos.1 as i32 + 1;

        let new_x = (self.pos.0 as i32 + self.vel.0 as i32).rem_euclid(max_x) as u32;
        let new_y = (self.pos.1 as i32 + self.vel.1 as i32).rem_euclid(max_y) as u32;

        self.pos = Pos(new_x, new_y);
    }
}

impl Pos {
    fn get_quadrant(&self, max_pos: &Pos) -> Quadrant {
        if self.0 > max_pos.0 / 2 && self.1 > max_pos.1 / 2 {
            Quadrant::TopRight
        } else if self.0 < max_pos.0 / 2 && self.1 > max_pos.1 / 2 {
            Quadrant::TopLeft
        } else if self.0 > max_pos.0 / 2 && self.1 < max_pos.1 / 2 {
            Quadrant::BottomRight
        } else if self.0 < max_pos.0 / 2 && self.1 < max_pos.1 / 2 {
            Quadrant::BottomLeft
        } else {
            Quadrant::None
        }
    }
}


fn print_bots_to_file(time:i32,bots: &[Robot], max_pos: &Pos, file_name: &str) {
    let mut grid = vec![vec!['.'; max_pos.0 as usize + 1]; max_pos.1 as usize + 1];
    for bot in bots {
        grid[bot.pos.1 as usize][bot.pos.0 as usize] = '#';
    }

    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name)
        .unwrap();
    
    writeln!(file, "{}", time).unwrap();
    for row in grid {
        writeln!(file, "{}", row.iter().collect::<String>()).unwrap();
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut bots = input
        .lines()
        .map(|line| {
            let mut parts = line.split(|c| c == '=' || c == ',' || c == ' ');
            let x = parts.nth(1).unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            let vx = parts.nth(1).unwrap().parse().unwrap();
            let vy = parts.next().unwrap().parse().unwrap();
            Robot {
                pos: Pos(x, y),
                vel: Velocity(vx, vy),
            }
        })
        .collect::<Vec<_>>();

    let simulation_time = 10_000; 
    let max_pos = &Pos(100, 102);
    for i in 0..simulation_time {
        for bot in &mut bots {
            bot.next(max_pos);
        }

            let quadrant_counts = bots.iter().fold([0; 4], |mut counts, bot| {
                match bot.pos.get_quadrant(max_pos) {
                    Quadrant::TopLeft => counts[0] += 1,
                    Quadrant::TopRight => counts[1] += 1,
                    Quadrant::BottomRight => counts[2] += 1,
                    Quadrant::BottomLeft => counts[3] += 1,
                    Quadrant::None => (),
                }
                counts
            });
        if i == 99 {
            let quandrant_product = quadrant_counts.iter().fold(1, |acc, &count| acc * count);
            assert_eq!(quandrant_product, 219512160);
        }

        println!("Time: {}", i);
        print_bots_to_file(i,&bots, max_pos, "images.txt");

    }
}
