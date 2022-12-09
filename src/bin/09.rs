use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Rope {
    head: Position,
    tail_segments: Vec<Position>,
    visited: HashSet<Position>,
}

impl Rope {
    fn apply_move(&mut self, direction: &str, distance: i64) {
        match direction {
            "R" => self.move_right(distance),
            "L" => self.move_left(distance),
            "U" => self.move_up(distance),
            "D" => self.move_down(distance),
            _ => {}
        };
    }

    fn move_right(&mut self, distance: i64) {
        for _ in 0..distance {
            self.head.x += 1;
            self.move_tail();
        }
    }
    fn move_left(&mut self, distance: i64) {
        for _ in 0..distance {
            self.head.x -= 1;
            self.move_tail();
        }
    }
    fn move_up(&mut self, distance: i64) {
        for _ in 0..distance {
            self.head.y += 1;
            self.move_tail();
        }
    }
    fn move_down(&mut self, distance: i64) {
        for _ in 0..distance {
            self.head.y -= 1;
            self.move_tail();
        }
    }

    fn move_tail(&mut self) {
        for i in 0..self.tail_segments.len() {
            let relative_to = if i == 0 {
                self.head.clone()
            } else {
                self.tail_segments[i - 1].clone()
            };
            self.move_tail_segment(i, relative_to);
        }
    }

    fn move_tail_segment(&mut self, tail_index: usize, relative_to: Position) {
        let (delta_x, delta_y) = (relative_to.x - self.tail_segments[tail_index].x, relative_to.y - self.tail_segments[tail_index].y);

        if delta_x.abs() > 1 || delta_y.abs() > 1 {
            self.tail_segments[tail_index].x += delta_x.signum();
            self.tail_segments[tail_index].y += delta_y.signum();
        }

        if tail_index == self.tail_segments.len() - 1 {
            self.visited.insert(self.tail_segments[tail_index].clone());
        }
    }
}


pub fn part_one(input: &str) -> Option<usize> {
    let mut rope = Rope {
        head: Position { x: 0, y: 0 },
        tail_segments: vec![Position { x: 0, y: 0 }],
        visited: HashSet::new(),
    };

    input.lines().for_each(|line| {
        let (direction, distance) = line.split_once(' ').unwrap();
        let distance = distance.parse::<i64>().unwrap();
        rope.apply_move(direction, distance);
    });

    let visited = rope.visited.len();
    Some(visited)
}


pub fn part_two(input: &str) -> Option<usize> {
    let mut rope = Rope {
        head: Position { x: 0, y: 0 },
        tail_segments: (0..9).into_iter().map(|_| Position { x: 0, y: 0 }).collect::<Vec<_>>(),
        visited: HashSet::new(),
    };

    input.lines().for_each(|line| {
        let (direction, distance) = line.split_once(' ').unwrap();
        let distance = distance.parse::<i64>().unwrap();
        rope.apply_move(direction, distance);
    });

    let visited = rope.visited.len();
    Some(visited)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
