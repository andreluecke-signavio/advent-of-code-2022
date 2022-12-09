use std::collections::HashMap;
use regex::Regex;

/*
returns: Map of "absolute directory path -> (total size of directory, Vector of individual file sizes)
 */
fn create_directory_flatmap(input: &str) -> HashMap<String, (u64, Vec<u64>)> {
    let mut directory_flatmap = HashMap::new();
    let mut current = "/".to_string();

    directory_flatmap.insert(current.clone(), (0u64, vec![]));

    let re_cd = Regex::new(r"^\$\scd\s(.+)").unwrap();
    let re_ls = Regex::new(r"^\$\sls").unwrap();
    let re_file = Regex::new(r"^(\d+)\s(.+)").unwrap();
    let re_dir = Regex::new(r"^dir\s(\w+)").unwrap();

    input.lines().for_each(|line| {
        if let Some(captures) = re_cd.captures(line) {
            let dir = captures.get(1).unwrap().as_str();
            match dir {
                "/" => {
                    current = "/".to_string();
                }
                ".." => {
                    current = move_one_directory_up(&current);
                }
                dir => {
                    current = current.clone() + dir + "/";
                    add_directory_to_flatmap(&mut directory_flatmap, current.clone());
                }
            }
        } else if re_ls.is_match(line) {
            // do nothing
        } else if let Some(captures) = re_dir.captures(line) {
            let dir = current.clone() + captures.get(1).unwrap().as_str() + "/";
            add_directory_to_flatmap(&mut directory_flatmap, dir);
        } else if let Some(captures) = re_file.captures(line) {
            let file_size = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
            add_file_size_recursively(&mut directory_flatmap, &current, file_size);
            directory_flatmap.get_mut(&current).unwrap().1.push(file_size);
        } else {
            panic!("unrecognized line {}", line)
        }
    });
    directory_flatmap
}

fn add_directory_to_flatmap(directory_flatmap: &mut HashMap<String, (u64, Vec<u64>)>, dir: String) {
    directory_flatmap.entry(dir).or_insert((0, vec![]));
}

fn add_file_size_recursively(map: &mut HashMap<String, (u64, Vec<u64>)>, current: &str, file_size: u64) {
    map.get_mut(current).unwrap().0 = map.get(current).unwrap().0 + file_size;
    if current == "/" {
        return;
    }
    let new_current = move_one_directory_up(current);
    add_file_size_recursively(map, &new_current, file_size);
}

fn move_one_directory_up(dir: &str) -> String {
    dir.trim_end_matches('/')
        .rsplit_once('/')
        .unwrap().0.to_string()
        + "/"
}


pub fn part_one(input: &str) -> Option<u64> {
    let map = create_directory_flatmap(input);
    let sum = map.values().into_iter()
        .map(|(size, _)| *size)
        .filter(|size| *size <= 100_000u64)
        .sum::<u64>();
    Some(sum)
}


pub fn part_two(input: &str) -> Option<u64> {
    let map = create_directory_flatmap(input);
    let root_size = map.get("/").unwrap().0;
    let diff = 70_000_000 - root_size;

    let mut values = map.values().into_iter()
        .map(|(size, _)| *size)
        .filter(|size| *size >= (30_000_000 - diff))
        .collect::<Vec<_>>();

    values.sort_unstable();
    Some(*values.get(0).unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
