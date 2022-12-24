use std::fs;
use std::collections::VecDeque;
use itertools::Itertools;

fn bfs(content: String, end: char) -> i32 {
    let lines = content.lines();
    let mut matrix: Vec<Vec<char>> = lines.map(|x| x.chars().collect_vec()).collect_vec();
    let mut start = (0, 0);

    for i in 0..matrix[0].len()-1 {
        for j in 0..matrix.len()-1 {
            if matrix[j][i] == 'E' { start = (i, j); matrix[j][i] = 'z'}
            if matrix[j][i] == 'S' { matrix[j][i] = end}
        }
    }
    let mut marked = vec![vec![false; matrix[0].len()]; matrix.len()];
    let mut queue = VecDeque::from([(start, 0)]);
    marked[start.1][start.0] = true;

    while let Some(((y, x), count)) = queue.pop_front() {
        if matrix[x][y] == end { return count; }
        let idx = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (a, b) in idx {
            let j = (x as i32 + a) as usize;
            let i = (y as i32 + b) as usize;
            let Some(val) = matrix.get(j)
                                         .and_then(|z| z.get(i))
                                         else { continue; };

            if (matrix[x][y] as i32  <= 1 + *val as i32) && !marked[j][i] {
                marked[j][i] = true;
                queue.push_back(((i, j), count+1));
            }
        }
    }
    return 0;
}

fn part1() {
    let content = fs::read_to_string("data/day12.txt").unwrap();
    let val = bfs(content, std::char::from_u32('a' as u32 - 1).unwrap());
    println!("part1: {val}");
}

fn part2() {
    let content = fs::read_to_string("data/day12.txt").unwrap();
    let val = bfs(content, 'a');
    println!("part2: {val}");
}

pub fn main() {
    println!("\nDay 12");
    part1();
    part2();
}