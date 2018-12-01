use std::fs;
use std::collections::HashSet;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> i32 {
    let input = fs::read_to_string("input.txt").expect("failed to read input file day01.txt");
    input.lines().map(|s| s.parse::<i32>().unwrap()).sum()
}

fn part2() -> i32 {
    let input = fs::read_to_string("input.txt").expect("failed to read input file day01.txt");
    let mut iter = input.lines().map(|s| s.parse::<i32>().unwrap()).cycle();
    let mut lookup = HashSet::new();

    let mut cur = 0;
    while let Some(frequency) = iter.next() {
        if lookup.contains(&cur) {
            break;
        }
        lookup.insert(cur);
        cur += frequency;
    }
    cur
}

#[cfg(test)]
mod test {
    use super::{ part1, part2 };

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 484);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 367);
    }
}
