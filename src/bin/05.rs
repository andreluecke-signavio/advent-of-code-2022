use regex::{Captures, Regex};


struct Instruction {
    amount: u32,
    from: usize,
    to: usize,
}

impl<'a> From<Captures<'a>> for Instruction {
    fn from(caps: Captures) -> Self {
        Instruction {
            amount: caps.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            from: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            to: caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (instructions, mut area) = prepare_area(input);

    let regex = Regex::new(r"move\s(\d+)\sfrom\s(\d+)\sto\s(\d+)").unwrap();
    instructions.into_iter().for_each(|line| {
        let ins: Instruction = regex.captures(line).unwrap().into();

        (0..ins.amount).into_iter().for_each(|_| {
            if let Some(c) = area[ins.from - 1].pop() {
                area[ins.to - 1].push(c);
            }
        });
    });


    let result = area.into_iter().map(|mut v| {
        v.pop().unwrap_or("".to_string())
    })
        .collect::<String>();

    println!("result: {:?}", &result);
    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let (instructions, mut area) = prepare_area(input);

    let regex = Regex::new(r"move\s(\d+)\sfrom\s(\d+)\sto\s(\d+)").unwrap();
    instructions.into_iter().for_each(|line| {
        let caps = regex.captures(line).unwrap();
        let ins = Instruction::from(caps);

        let mut temp = vec![];
        (0..ins.amount).into_iter().for_each(|_| {
            if let Some(c) = area[ins.from - 1].pop() {
                temp.push(c);
            }
        });
        temp.reverse();
        area[ins.to - 1].extend(temp);
    });


    let result = area.into_iter().map(|mut v| {
        v.pop().unwrap_or("".to_string())
    })
        .collect::<String>();

    println!("result: {:?}", &result);
    Some(result)
}

fn prepare_area(input: &str) -> (Vec<&str>, Vec<Vec<String>>) {
    let mut v1 = vec![];
    let mut v2 = vec![];
    let mut switch = false;
    for line in input.lines() {
        // println!("line: {}", line);
        if line.len() == 0 {
            switch = true;
            continue;
        }
        if switch {
            v2.push(line);
        } else {
            v1.push(line);
        }
    }
    v1.reverse();

    let n = v1[0].split_whitespace().count();
    let m = v1[0].len();
    println!("n: {}", n);

    let mut area: Vec<Vec<String>> = vec![];
    (0..n).into_iter().for_each(|_| area.push(vec![]));

    v1.into_iter().skip(1).for_each(|line| {
        for i in (0..m).step_by(4) {
            let c = line.chars().skip(i + 1).take(1).collect::<String>();
            // println!("i: {}, c: {}", i, c);
            if !c.trim().is_empty() {
                area[i / 4 as usize].push(c);
            }
        }
    });
    (v2, area)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
