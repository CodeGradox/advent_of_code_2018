use std::fs;
use std::collections::HashMap;

fn main() {
    let input = input_file();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let mut twos = 0;
    let mut threes = 0;

    for line in input.lines().map(str::trim) {
        let mut lookup: HashMap<char, u32> = HashMap::new();

        for c in line.chars() {
            lookup.entry(c)
                .and_modify(|val| *val += 1)
                .or_insert(1);
        }

        if lookup.values().find(|val| **val == 2).is_some() {
            twos += 1;
        }

        if lookup.values().find(|val| **val == 3).is_some() {
            threes += 1;
        }
    }
    twos * threes
}

fn part2(input: &str) -> Option<String> {
    for (i, line) in input.lines().map(str::trim).enumerate() {
        for cmp_line in input.lines().skip(i + 1).map(str::trim) {
            let mut missmatches = 0;
            let mut output = String::with_capacity(line.len());

            for (a, b) in line.chars().zip(cmp_line.chars()) {
                if a != b {
                    missmatches += 1;
                    if missmatches > 1 {
                        break;
                    }
                    continue;
                }
                output.push(a);
            }

            if missmatches == 1 {
                return Some(output)
            }
        }
    }
    None
}

fn input_file() -> String {
    fs::read_to_string("input.txt").expect("Failed to read input.txt")
}

#[cfg(test)]
mod test {
    use super::{ part1, part2 };

    #[test]
    fn test_part1() {
        let input = "abcdef
        bababc
        abbcde
        abcccd
        aabcdd
        abcdee
        ababab";
        assert_eq!(part1(&input), 12);
    }

    #[test]
    fn test_part2() {
        let input = "abcde
        fghij
        klmno
        pqrst
        fguij
        axcye
        wvxyz";
        assert_eq!(part2(input), Some(String::from("fgij")));
    }
}
