use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    // let file_path = String::from("test_input.txt");
    let file_path = String::from("input.txt");
    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);
    let mut safe_checks: Vec<i32> = Vec::new();
    let mut damp_safe_checks: Vec<i32> = Vec::new();
    for line in buf_reader.lines() {
        let line = line?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        let safe_check = test_safety(&nums)? as i32;
        safe_checks.push(safe_check);
        let damp_safe_check = damp_test_safety(&nums)? as i32;
        damp_safe_checks.push(damp_safe_check)
    }
    let saves: i32 = safe_checks.iter().sum();
    println!("Number of safe reports (no dampening): {}", saves);
    let damp_saves: i32 = damp_safe_checks.iter().sum();
    println!("Number of safe reports (with dampening): {}", damp_saves);

    Ok(())
}

fn test_safety(a: &Vec<i32>) -> Result<bool> {
    let size = a.len();
    let diffs: Vec<i32> = a[1..]
        .iter()
        .zip(a[..size - 1].iter())
        .map(|(x, y)| y - x)
        .collect();

    let positive = diffs.iter().all(|&x| x > 0);
    let negative = diffs.iter().all(|&x| x < 0);
    let monotone = positive || negative;
    let bounded = diffs.iter().map(|&x| x.abs()).all(|x| x >= 0 && x <= 3);

    Ok(monotone && bounded)
}

fn damp_test_safety(a: &Vec<i32>) -> Result<bool> {
    let size = a.len();

    for i in 0..size {
        let mut b = a.clone();
        b.remove(i);
        if test_safety(&b)? {
            return Ok(true);
        }
    }

    Ok(false)
}
