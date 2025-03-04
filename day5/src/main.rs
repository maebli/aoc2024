use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
struct Update<'a> {
    page_number: u64,
    rules: &'a HashMap<u64, Vec<u64>>,
}
impl<'a> PartialOrd for Update<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a> Ord for Update<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else if let Some(rule) = self.rules.get(&self.page_number) {
            if rule.contains(&other.page_number) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            Ordering::Greater
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut input = input.split("\n\n");
    let rules = input
        .next()
        .unwrap()
        .split("\n")
        .map(|x| {
            let mut x = x.split("|");
            (
                x.next().unwrap().parse::<u64>().unwrap(),
                x.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .fold(HashMap::new(), |mut acc: HashMap<u64, Vec<u64>>, x| {
            if acc.contains_key(&x.0) {
                acc.get_mut(&x.0).unwrap().push(x.1);
            } else {
                acc.insert(x.0, vec![x.1]);
            };
            acc
        });

    let updates = input
        .next()
        .unwrap()
        .split("\n")
        .map(|x| {
            let x = x.split(",");
            let mut out = vec![];
            for i in x {
                out.push(Update {
                    page_number: i.parse::<u64>().unwrap(),
                    rules: &rules,
                });
            }
            out
        })
        .collect::<Vec<_>>();

    let mut sum_of_middle_unsorted = 0;
    let mut sum_of_middle_sorted = 0;
    for mut update in updates {
        if !update.windows(2).all(|w| w[0] <= w[1]) {
            update.sort();
            let mid = update.len() / 2;
            sum_of_middle_sorted += update[mid].page_number;
        } else {
            let mid = update.len() / 2;
            sum_of_middle_unsorted += update[mid].page_number;
        }
    }

    println!("{}", sum_of_middle_unsorted);
    println!("{}", sum_of_middle_sorted);
}
