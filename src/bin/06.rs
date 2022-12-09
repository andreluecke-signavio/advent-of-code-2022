use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<usize> {
    let mut bag = HashSet::new();
    let mut result = 0;
    for i in 4..input.len() + 1 {
        bag.insert(input.get(i - 4..i - 3).unwrap());
        bag.insert(input.get(i - 3..i - 2).unwrap());
        bag.insert(input.get(i - 2..i - 1).unwrap());
        bag.insert(input.get(i - 1..i).unwrap());
        if bag.len() == 4 {
            result = i;
            break;
        }
        bag.clear();
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut bag = HashSet::new();
    let mut result = 0;
    for i in 14..input.len() + 1 {
        for j in (1..15).rev() {
            bag.insert(input.get(i - j..i - j + 1).unwrap());
        }
        if bag.len() == 14 {
            result = i;
            break;
        }
        bag.clear();
    }
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(6));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(23));
    }
}
