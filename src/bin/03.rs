/*
playing with bits just for fun 🙂
- map chars to priorities in a vector for quick lookups
-- the ASCII code point is the index and the value is the priority
- then convert each char to a priority and keep the occurrences in a 64 bit map
- use bit operations to find the common element
-- each common element is a power of 2 (as there should be only 1 bit set)
- convert power of 2 number back to priority by counting the trailing zeros
 */
pub fn part_one(input: &str) -> Option<u32> {
    let chars_to_prio = chars_to_prio();

    let mut x1: u64 = 0;
    let mut x2: u64 = 0;
    let result = input.lines().into_iter().map(|line| {
        x1 = 0;
        x2 = 0;
        let (s1, s2) = line.split_at(line.len() / 2);

        s1.chars().for_each(|c| {
            let n = chars_to_prio[c as usize];
            x1 |= 1 << (n - 1) as u64
        });
        s2.chars().for_each(|c| {
            let n = chars_to_prio[c as usize];
            x2 |= 1 << (n - 1) as u64
        });
        (x1 & x2).trailing_zeros() + 1
    })
        .sum::<u32>();

    Some(result)
}

fn chars_to_prio() -> Vec<u32> {
    let mut result = vec![];

    for i in 0..123 {
        if i >= 97 {
            result.push(i - 97 + 1);
        } else if i >= 65 {
            result.push(i - 65 + 26 + 1);
        } else {
            result.push(0);
        }
    }
    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let chars_to_prio = chars_to_prio();

    let lines = input.lines().collect::<Vec<_>>();

    let result = lines.chunks_exact(3).map(|chunk| {
        let mut common_bit: u64 = !0;
        for s in chunk {
            let mut bits: u64 = 0;
            s.chars().for_each(|c| {
                let n = chars_to_prio[c as usize];
                bits |= 1 << (n - 1) as u64;
            });
            common_bit &= bits;
        }
        common_bit.trailing_zeros() + 1
    })
        .sum::<u32>();

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
