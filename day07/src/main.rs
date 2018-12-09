use std::collections::{ HashMap, HashSet };

fn main() {
    println!("{:?}", part1(PART1_INPUT));
}

fn part1(input: &[(char, char)]) -> Option<String> {
    let mut dependencies: HashMap<char, u32> = HashMap::new();
    let mut edges: HashMap<char, HashSet<char>> = HashMap::new();
    let mut ret = String::with_capacity(input.len());

    for (left, right) in input {
        dependencies.entry(*left)
            .or_insert(0);
        dependencies.entry(*right)
            .and_modify(|val| *val += 1)
            .or_insert(1);

        let edge = edges.entry(*left)
            .or_default();
        edge.insert(*right);
    }

    while dependencies.len() > 0 {
        let entry = dependencies.iter()
            .filter(|(_, v)| **v == 0)
            .min_by(|a, b| a.0.cmp(b.0))
            .map(|(key, _val)| *key)?;
        ret.push(entry);
        dependencies.remove(&entry);

        if let Some (nodes) =  edges.get(&entry) {
            for edge in nodes.iter() {
                dependencies.get_mut(&edge).map(|val| *val -= 1)?;
            }
        }
    }

    Some(ret)
}

#[cfg(test)]
mod test {
    use super::part1;

    #[test]
    fn test_part1() {
        let input = vec![
            ('C', 'A'),
            ('C', 'F'),
            ('A', 'B'),
            ('A', 'D'),
            ('B', 'E'),
            ('D', 'E'),
            ('F', 'E')
        ];
        assert_eq!(part1(&input), Some("CABDFE".into()));
    }
}

const PART1_INPUT: &[(char, char)] = &[
    ('T', 'X'),
    ('G', 'O'),
    ('X', 'B'),
    ('I', 'W'),
    ('N', 'V'),
    ('K', 'H'),
    ('S', 'R'),
    ('P', 'J'),
    ('L', 'V'),
    ('D', 'E'),
    ('J', 'R'),
    ('U', 'W'),
    ('M', 'Q'),
    ('B', 'F'),
    ('F', 'E'),
    ('V', 'Q'),
    ('C', 'A'),
    ('H', 'Z'),
    ('A', 'Y'),
    ('O', 'Y'),
    ('W', 'Q'),
    ('E', 'Y'),
    ('Y', 'Z'),
    ('Q', 'R'),
    ('R', 'Z'),
    ('S', 'E'),
    ('O', 'W'),
    ('G', 'B'),
    ('I', 'N'),
    ('G', 'I'),
    ('H', 'R'),
    ('N', 'C'),
    ('M', 'W'),
    ('Y', 'R'),
    ('T', 'B'),
    ('G', 'D'),
    ('J', 'O'),
    ('I', 'A'),
    ('J', 'H'),
    ('T', 'Y'),
    ('N', 'H'),
    ('B', 'V'),
    ('M', 'R'),
    ('Y', 'Q'),
    ('X', 'J'),
    ('A', 'E'),
    ('P', 'Z'),
    ('P', 'C'),
    ('N', 'Q'),
    ('A', 'O'),
    ('G', 'X'),
    ('P', 'U'),
    ('T', 'S'),
    ('I', 'V'),
    ('V', 'H'),
    ('U', 'F'),
    ('D', 'Q'),
    ('D', 'O'),
    ('G', 'H'),
    ('I', 'Z'),
    ('N', 'D'),
    ('B', 'Y'),
    ('J', 'M'),
    ('V', 'Y'),
    ('W', 'Y'),
    ('E', 'Z'),
    ('T', 'N'),
    ('L', 'U'),
    ('S', 'A'),
    ('Q', 'Z'),
    ('T', 'F'),
    ('F', 'Z'),
    ('J', 'C'),
    ('X', 'Y'),
    ('K', 'V'),
    ('T', 'I'),
    ('I', 'O'),
    ('C', 'W'),
    ('B', 'Q'),
    ('W', 'Z'),
    ('D', 'H'),
    ('K', 'A'),
    ('M', 'E'),
    ('T', 'U'),
    ('I', 'J'),
    ('O', 'Q'),
    ('M', 'Z'),
    ('U', 'C'),
    ('N', 'F'),
    ('C', 'H'),
    ('X', 'E'),
    ('F', 'O'),
    ('P', 'O'),
    ('J', 'A'),
    ('H', 'Y'),
    ('A', 'Q'),
    ('V', 'Z'),
    ('S', 'L'),
    ('H', 'E'),
    ('X', 'I'),
    ('O', 'R')
];