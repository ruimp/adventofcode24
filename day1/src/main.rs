use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    // let file_path = String::from("test_input.txt");
    let file_path = String::from("input.txt");
    let input = get_input(file_path)?;

    // let distance = dist(&input.0, &input.1);
    // println!("{}", distance);

    let distance = dist(&input.0, &input.1);
    let similarity = simi(&input.0, &input.1);
    println!("Distance: {}", distance);
    println!("Similarity: {}", similarity);
    Ok(())
}

// Read input file
fn get_input(file_path: String) -> Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();
    for line in buf_reader.lines() {
        let line = line?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        a.push(nums[0]);
        b.push(nums[1]);
    }
    Ok((a, b))
}

// Find distance between vectors by summing absolute element-wise difference
fn dist(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    let mut a_sorted = a.clone();
    a_sorted.sort();
    let mut b_sorted = b.clone();
    b_sorted.sort();
    a_sorted
        .iter()
        .zip(b_sorted.iter())
        .map(|(x, y)| (x - y).abs())
        .sum()
}

fn simi(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    let mut scores: Vec<i32> = Vec::new();
    for x in a.iter() {
        let count = b.iter().filter(|&y| x == y).count() as i32;
        scores.push(count * x);
    }
    scores.iter().sum()
}
