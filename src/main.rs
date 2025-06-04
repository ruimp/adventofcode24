use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    // let a = vec![3, 4, 2, 1, 3, 3];
    // let b = vec![4, 3, 5, 3, 9, 3];
    let file_path = String::from("input.txt");
    let input = get_input(file_path)?;

    let distance = dist(&input.0, &input.1);
    println!("{}", distance);
    Ok(())
}

// Read input file
fn get_input(file_path: String) -> Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);
    let mut x: Vec<i32> = Vec::new();
    let mut y: Vec<i32> = Vec::new();
    for line in buf_reader.lines() {
        let line = line?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        x.push(nums[0]);
        y.push(nums[1]);
    }
    Ok((x, y))
}

// Find distance between vectors by summing absolute element-wise difference
fn dist(x: &Vec<i32>, y: &Vec<i32>) -> i32 {
    let mut x_sorted = x.clone();
    x_sorted.sort();
    let mut y_sorted = y.clone();
    y_sorted.sort();
    x_sorted
        .iter()
        .zip(y_sorted.iter())
        .map(|(&x, &y)| (x - y).abs())
        .sum()
}
