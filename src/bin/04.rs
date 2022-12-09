struct Range {
    start: usize,
    end: usize,
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let (s1, s2) = s.split_once('-').unwrap();
        Range {
            start: s1.parse().unwrap(),
            end: s2.parse().unwrap(),
        }
    }
}


pub fn part_one(input: &str) -> Option<usize> {
    let range_pairs = input.lines().map(|line| {
        let (s1, s2) = line.split_once(',').unwrap();
        let range1 = Range::from(s1);
        let range2 = Range::from(s2);
        (range1, range2)
    })
        .filter(|(range1, range2)| {
            range1.start <= range2.start && range1.end >= range2.end
                || range2.start <= range1.start && range2.end >= range1.end
        })
        .count();

    Some(range_pairs)
}

pub fn part_two(input: &str) -> Option<usize> {
    let range_pairs = input.lines().map(|line| {
        let (s1, s2) = line.split_once(',').unwrap();
        let range1 = Range::from(s1);
        let range2 = Range::from(s2);
        (range1, range2)
    })
        .filter(|(range1, range2)| {
            range2.start <= range1.end && range2.end >= range1.start
        })
        .count();
    Some(range_pairs)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
