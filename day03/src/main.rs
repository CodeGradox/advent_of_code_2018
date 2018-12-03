extern crate regex;

use regex::Regex;
use std::collections::{ HashMap, HashSet };
use std::fs;

fn main() {
    let input = input_file();
    println!("Part 1: {}", part1(1000, &input));
    println!("Part 2: {:?}", part2(1000, &input));
}

fn part1(grid_size: usize, input: &str) -> usize {
    let mut fabric = vec![0usize; grid_size * grid_size];
    let re = Regex::new(r"(\d+),(\d+):\s(\d+)x(\d+)").unwrap();

    for cap in re.captures_iter(input) {
        let offset_x: usize = (&cap[1]).parse().unwrap();
        let offset_y: usize = (&cap[2]).parse().unwrap();
        let width: usize = (&cap[3]).parse().unwrap();
        let height: usize = (&cap[4]).parse().unwrap();

        for h in offset_y..(offset_y + height) {
            for w in offset_x..(offset_x + width) {
                fabric[h * grid_size + w] += 1;
            }
        }
    }

    fabric.iter()
        .filter(|val| **val > 1)
        .count()
}

fn part2(grid_size: usize, input: &str) -> Option<usize> {
    let re = Regex::new(r"#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)").unwrap();
    let mut lookup: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut ids = HashSet::new();

    for cap in re.captures_iter(input) {
        let id: usize = (&cap[1]).parse().unwrap();
        let offset_x: usize = (&cap[2]).parse().unwrap();
        let offset_y: usize = (&cap[3]).parse().unwrap();
        let width: usize = (&cap[4]).parse().unwrap();
        let height: usize = (&cap[5]).parse().unwrap();

        let _ = ids.insert(id);

        for h in offset_y..(offset_y + height) {
            for w in offset_x..(offset_x + width) {
                lookup.entry(h * grid_size + w)
                    .and_modify(|vec| vec.push(id))
                    .or_insert(vec![id]);
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

fn input_file() -> String {
    fs::read_to_string("input.txt").expect("Failed to read input.txt")
}

#[cfg(test)]
mod test {
    use super::{ part1, part2 };

    #[test]
    fn test_part1() {
        let grid_size = 8;
        let input = "#1 @ 1,3: 4x4
        #2 @ 3,1: 4x4
        #3 @ 5,5: 2x2";
        assert_eq!(part1(grid_size, input), 4);
    }

    #[test]
    fn test_part2() {
        let grid_size = 8;
        let input = "#1 @ 1,3: 4x4
        #2 @ 3,1: 4x4
        #3 @ 5,5: 2x2";
        assert_eq!(part2(grid_size, input), Some(3));
    }
}
