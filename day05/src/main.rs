use std::fs;

fn main() {
    let input = input_file();
    println!("{}", part1(input.bytes()));
    println!("{}", part2(&input));
}

fn part1(input: impl Iterator<Item = u8>) -> usize {
    let mut chars = input.collect::<Vec<u8>>();

    let mut i = 0;
    while !chars.is_empty() && i < chars.len() - 1 {
        if chars[i] != chars[i + 1] && chars[i] | 32 == chars[i + 1] | 32 {
            chars.remove(i);
            chars.remove(i);
            if i > 0 {
                i -= 1;
            }
            continue;
        }
        i += 1;
    }

    chars.iter().count()
}

fn part2(input: &str) -> usize {
    let mut shortest = input.len();

    for i in b'a'..=b'z' {
        let chars = input.as_bytes().iter().cloned().filter(|b| (*b | 32) != i);
        let length = part1(chars);
        if length < shortest {
            shortest = length;
        }
    }
    shortest
}

fn input_file() -> String {
    fs::read_to_string("input.txt").expect("Failed to read input.txt")
    // fs::read_to_string("biginput.txt").expect("Failed to read input.txt")
    // fs::read_to_string("biginput2.txt").expect("Failed to read input.txt")
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