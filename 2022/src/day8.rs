use std::fs;
use std::cmp::max;

fn part1() { // implicit matrix rotation (via reversed indexing) for solution in O(nm) time and O(n) space
    let content = fs::read_to_string("data/day8.txt").unwrap();
    let lines = content.lines();
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let mut results: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        matrix.push(Vec::from_iter(line.chars().map(|x| x.to_digit(10).unwrap() as i32)));
        results.push(Vec::from_iter(line.chars().map(|_| 0)));
    }
    for i in 0..matrix.len() {
        let mut lmax = 0;
        let mut umax = 0;
        for j in 0..matrix[0].len() {
            if i==0 || i==matrix.len()-1 || j == 0 || j ==matrix[0].len()-1 {
                results[i][j] = 1;
            }
            if matrix[i][j] > lmax {
                lmax = matrix[i][j];
                results[i][j] = 1;
            }
            if matrix[j][i] > umax {
                umax = matrix[j][i];
                results[j][i] = 1;
            }
        }
        let mut rmax = 0;
        let mut dmax = 0;
        for j in (0..matrix[0].len()).rev() {
            if matrix[i][j] > rmax {
                rmax = matrix[i][j];
                results[i][j] = 1;
            }
            if matrix[j][i] > dmax {
                dmax = matrix[j][i];
                results[j][i] = 1;
            }
        }
    }

    let ret: i32 = results.iter().flat_map(|x: &Vec<_>| x.iter()).sum();
    println!("part1: {ret}");
}

fn get_score(m: &Vec<Vec<i32>>, i_val: usize, j_val: usize) -> i32 {
    let me = m[i_val][j_val];
    let mut i = i_val;
    let mut j = j_val;
    let mut lctr = 0;
    let mut rctr = 0;
    if !(j == 0 || j== m[0].len()-1) {
        loop {
            lctr+=1;
            j-=1;
            if j <= 0 || m[i][j] >= me { break }
        }
        let mut j = j_val;
        loop {
            rctr+=1;
            j+=1;
            if j >= m[0].len()-1 || m[i][j] >= me { break }
        }
    }
    let j = j_val;
    let mut uctr = 0;
    let mut dctr = 0;
    if !(i==0 || i==m.len()-1) {
        loop {
            uctr+=1;
            i-=1;
            if i <= 0 || m[i][j] >= me { break }
        }
        let mut i = i_val;
        loop {
            dctr+=1;
            i+=1;
            if i >= m.len()-1 || m[i][j] >= me { break }
        }
    }
    if i_val==0 || j_val == 0 || i_val==m.len()-1 || j_val==m[0].len()-1 { lctr=0 }
    return lctr * rctr * uctr * dctr;
}

fn part2() {
    let content = fs::read_to_string("data/day8.txt").unwrap();
    let lines = content.lines();
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        matrix.push(Vec::from_iter(line.chars().map(|x| x.to_digit(10).unwrap() as i32)));
    }
    let mut max_score = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            max_score = max(max_score, get_score(&matrix, i, j))
        }
    }
    println!("part2: {max_score}");
}

pub fn main() {
    println!("\nDay 8");
    part1();
    part2();
}