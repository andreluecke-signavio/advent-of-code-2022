use std::cmp::max;

pub fn part_one(input: &str) -> Option<u32> {
    let grid = build_grid(input);

    let mut visible: u32 = (2 * grid.len() + 2 * grid[0].len() - 4) as u32;

    for y in 1..grid.len() - 1 {
        for x in 1..grid[y].len() - 1 {
            let height = grid[y][x];
            if is_visible(&grid, x, y, height) {
                visible = visible + 1;
            }
        }
    }

    Some(visible)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = build_grid(input);

    let mut max_score = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let height = grid[y][x];
            let score = compute_scenic_score(&grid, x, y, height);
            if score > max_score {
                max_score = score
            }
        }
    }

    Some(max_score)
}

fn build_grid(input: &str) -> Vec<Vec<u32>> {
    let grid = input.lines().map(|line| {
        line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()
    })
        .collect::<Vec<_>>();
    grid
}

fn is_visible(grid: &Vec<Vec<u32>>, x: usize, y: usize, height: u32) -> bool {
    is_visible_left(grid, x, y, height)
        || is_visible_right(grid, x, y, height)
        || is_visible_up(grid, x, y, height)
        || is_visible_down(grid, x, y, height)
}

fn is_visible_down(grid: &Vec<Vec<u32>>, x: usize, y: usize, height: u32) -> bool {
    for i in y + 1..grid.len() {
        if grid[i][x] >= height {
            return false;
        }
    }
    true
}

fn is_visible_up(grid: &Vec<Vec<u32>>, x: usize, y: usize, height: u32) -> bool {
    for i in 0..y {
        if grid[i][x] >= height {
            return false;
        }
    }
    true
}

fn is_visible_right(grid: &Vec<Vec<u32>>, x: usize, y: usize, height: u32) -> bool {
    for i in x + 1..grid[y].len() {
        if grid[y][i] >= height {
            return false;
        }
    }
    true
}

fn is_visible_left(grid: &Vec<Vec<u32>>, x: usize, y: usize, height: u32) -> bool {
    for i in 0..x {
        if grid[y][i] >= height {
            return false;
        }
    }
    true
}


fn compute_scenic_score(grid: &Vec<Vec<u32>>, x: usize, y: usize, height: u32) -> u64 {
    score_left(grid, x, y, height)
        * score_right(grid, x, y, height)
        * score_up(grid, x, y, height)
        * score_down(grid, x, y, height)
}

fn score_left(grid: &Vec<Vec<u32>>, x: usize, y: usize, height: u32) -> u64 {
    let mut count = 0;
    for i in (0..x).rev() {
        count = count + 1;
        if grid[y][i] >= height {
            break;
        }
    }
    count
}

fn score_right(grid: &Vec<Vec<u32>>, x: usize, y: usize, height: u32) -> u64 {
    let mut count = 0;
    for i in x + 1..grid[y].len() {
        count = count + 1;
        if grid[y][i] >= height {
            break;
        }
    }
    count
}

fn score_up(grid: &Vec<Vec<u32>>, x: usize, y: usize, height: u32) -> u64 {
    let mut count = 0;
    for i in (0..y).rev() {
        count = count + 1;
        if grid[i][x] >= height {
            break;
        }
    }
    count
}

fn score_down(grid: &Vec<Vec<u32>>, x: usize, y: usize, height: u32) -> u64 {
    let mut count = 0;
    for i in y + 1..grid.len() {
        count = count + 1;
        if grid[i][x] >= height {
            break;
        }
    }
    count
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
