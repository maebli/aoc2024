use pathfinding::prelude::count_paths;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(usize);

impl Pos {
    fn successors(&self, parts: &Vec<&str>, full: &str) -> Vec<Pos> {
        let max_length_parts = parts.iter().map(|x| x.len()).max().unwrap();
        let mut result = Vec::new();
        for i in 1..max_length_parts + 1 {
            let next = full.get(self.0..self.0 + i);
            if let Some(next) = next {
                for part in parts.iter() {
                    if next == *part {
                        result.push(Pos(self.0 + i));
                    }
                }
            }
        }
        result
    }
}

fn main() {
    let input = include_str!("input.txt");

    let mut split_input = input.split("\n\n");

    let parts = split_input.next().unwrap().split(", ").collect::<Vec<_>>();

    let mut possible_count = 0;
    let mut possibility_count = 0;
    let tic = std::time::Instant::now();
    for line in split_input.next().unwrap().lines() {
        let n = count_paths(
            Pos(0),
            |p| p.successors(&parts, &line),
            |p| p.0 == line.len(),
        );

        if n > 0 {
            possible_count += 1;
        }
        possibility_count += n;
    }
    let toc = tic.elapsed();
    println!("Time: {:?}", toc);

    println!("Possible count: {}", possible_count);
    println!("Possibility count: {}", possibility_count);
}
