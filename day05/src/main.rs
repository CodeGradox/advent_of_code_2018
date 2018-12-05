use std::fs;

fn main() {
    let input = input_file();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut chars: Vec<char> = input.chars().collect();

    let mut i = 0;
    while !chars.is_empty() && i < chars.len() - 1 {
        if chars[i] != chars[i + 1] && chars[i].to_ascii_lowercase() == chars[i + 1].to_ascii_lowercase() {
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

    for i in 97u8..=122 {
        let polymer = i as char;
        let chars: String = input.chars().filter(|ch| ch.to_ascii_lowercase() != polymer).collect();
        let length = part1(&chars);
        if length < shortest {
            shortest = length;
        }
    }
    shortest
}

fn input_file() -> String {
  fs::read_to_string("input.txt").expect("Failed to read input.txt")
}

#[cfg(test)]
mod test {
    use super::{ part1, part2 };

    #[test]
    fn test_part1() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!(part1(&input), 10);
    }

    #[test]
    fn test_part1_2() {
        let input = "aaAA";
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part1_empty() {
        let input = "";
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part1_no_change() {
        let input = "aaBAA";
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn test_part2() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!(part2(&input), 4);
    }
}