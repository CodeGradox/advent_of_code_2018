extern crate regex;

use regex::{ Regex, Captures };
use std::collections::{ HashMap, HashSet };
use std::fs;

#[derive(Debug)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

fn main() {
    let input = input_file();
    let claims = collect_claims(&input);
    println!("Part 1: {}", part1(&claims));
    println!("Part 2: {:?}", part2(&claims));
}

fn part1(claims: &[Claim]) -> usize {
    let mut fabric: HashMap<(usize, usize), usize> = HashMap::new();

    for claim in claims.iter() {
        for h in (0..claim.height).map(|c| c + claim.y) {
            for w in (0..claim.width).map(|c| c + claim.x) {
                fabric.entry((h, w))
                    .and_modify(|val| *val += 1)
                    .or_insert(1);
            }
        }
    }
    fabric.values()
        .filter(|val| **val > 1)
        .count()
}

fn part2(claims: &[Claim]) -> Option<usize> {
    let mut lookup: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    let mut ids = HashSet::new();

    for claim in claims.iter() {
        ids.insert(claim.id);
        for h in (0..claim.height).map(|c| c + claim.y) {
            for w in (0..claim.width).map(|c| c + claim.x) {
                lookup.entry((h, w))
                    .and_modify(|vec| vec.push(claim.id))
                    .or_insert(vec![claim.id]);
            }
        }
    }

    for vec in lookup.values().filter(|vec| vec.len() > 1) {
        for id in vec {
            ids.remove(id);
        }
    }
    if ids.len() != 1 {
        return None
    }
    ids.iter().next().map(|v| *v)
}

fn collect_claims(input: &str) -> Vec<Claim> {
    let re = Regex::new(r"#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            Claim {
                id: (&cap[1]).parse().unwrap(),
                x: (&cap[2]).parse().unwrap(),
                y: (&cap[3]).parse().unwrap(),
                width: (&cap[4]).parse().unwrap(),
                height: (&cap[5]).parse().unwrap()
            }
        }).collect()
}

fn input_file() -> String {
    fs::read_to_string("input.txt").expect("Failed to read input.txt")
}

#[cfg(test)]
mod test {
    use super::{ part1, part2, collect_claims };

    #[test]
    fn test_part1() {
        let input = "#1 @ 1,3: 4x4
        #2 @ 3,1: 4x4
        #3 @ 5,5: 2x2";
        assert_eq!(part1(&collect_claims(input)), 4);
    }

    #[test]
    fn test_part2() {
        let input = "#1 @ 1,3: 4x4
        #2 @ 3,1: 4x4
        #3 @ 5,5: 2x2";
        assert_eq!(part2(&collect_claims(input)), Some(3));
    }
}
