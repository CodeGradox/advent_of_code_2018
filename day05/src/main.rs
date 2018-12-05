use std::fs;

fn main() {
    let input = input_file();
    println!("{}", part1(input.bytes()));
    println!("{}", part2(&input));
}

fn part1(input: impl Iterator<Item = u8>) -> usize {
    let mut ret = vec![];

    for b in input {
        if let Some(&a) = ret.last() {
            if a != b && a | 32 == b | 32 {
                ret.pop();
                continue;
            }
        }
        ret.push(b);
    }

    ret.len()
}

fn part2(input: &str) -> usize {
    (b'a'..b'z')
        .map(|low| part1(input.as_bytes().iter().cloned().filter(|b| *b | 32 != low)))
        .min()
        .unwrap()
}

fn input_file() -> String {
    // fs::read_to_string("input.txt").expect("Failed to read input.txt")
    // fs::read_to_string("biginput.txt").expect("Failed to read input.txt")
    fs::read_to_string("biginput2.txt").expect("Failed to read input.txt")
}

#[cfg(test)]
mod test {
    use super::{ part1, part2 };

    #[test]
    fn test_part1() {
        let input = "dabAcCaCBAcCcaDA".bytes();
        assert_eq!(part1(input), 10);
    }

    #[test]
    fn test_part1_2() {
        let input = "aaAA".bytes();
        assert_eq!(part1(input), 0);
    }

    #[test]
    fn test_part1_empty() {
        let input = "".bytes();
        assert_eq!(part1(input), 0);
    }

    #[test]
    fn test_part1_no_change() {
        let input = "aaBAA".bytes();
        assert_eq!(part1(input), 5);
    }

    #[test]
    fn test_part2() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!(part2(&input), 4);
    }
}