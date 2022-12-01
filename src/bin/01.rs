pub fn part_one(input: &str) -> Option<u32> {
    let mut most_cals = 0;
    let mut current_cals = 0;

    for s in input.lines() {
        match s.parse::<u32>() {
            Ok(n) => {
                current_cals = current_cals + n;
                if current_cals > most_cals {
                    most_cals = current_cals;
                }
            }
            Err(_) => {
                current_cals = 0;
            }
        }
    }
    Some(most_cals)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elves_calories = vec![];
    let mut current_cals = 0;

    for s in input.lines() {
        match s.parse::<u32>() {
            Ok(n) => {
                current_cals = current_cals + n;
            }
            Err(_) => {
                elves_calories.push(current_cals);
                current_cals = 0;
            }
        }
    }
    elves_calories.push(current_cals);
    elves_calories.sort_by(|a, b| b.cmp(a));

    Some(elves_calories.into_iter().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24_000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45_000));
    }
}
