use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    // let file_path = String::from("faulty_input.txt");
    // let file_path = String::from("test_input.txt");
    let file_path = String::from("input.txt");
    let (safe_reports, safe_damp_reports) = parse_data(file_path)?;
    println!("Safe reports: {}", safe_reports);
    println!("Safe reports with dampening: {}", safe_damp_reports);
    Ok(())
}

fn parse_data(file_path: String) -> Result<(isize, isize)> {
    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);
    let mut count: isize = 0;
    let mut damp_count: isize = 0;
    for line in buf_reader.lines() {
        let nums: Vec<isize> = line?
            .split_whitespace()
            .filter_map(|s| match s.parse::<isize>() {
                Ok(s) => Some(s),
                Err(_) => {
                    println!("Skipping invalid entry {:?}", s);
                    None
                }
            })
            .collect();
        if test_safety(&nums)? {
            count += 1
        }
        if damp_test_safety(&nums)? {
            damp_count += 1
        }
    }
    Ok((count, damp_count))
}

fn test_safety(a: &Vec<isize>) -> Result<bool> {
    let diffs: Vec<isize> = a.windows(2).map(|x| x[1] - x[0]).collect();
    let positive = diffs.iter().all(|&x| x >= 1 && x <= 3);
    let negative = diffs.iter().all(|&x| x <= -1 && x >= -3);
    Ok(positive || negative)
}

fn damp_test_safety(a: &Vec<isize>) -> Result<bool> {
    for i in 1..a.len() {
        let left = &a[..i - 1];
        let right = &a[i..];
        let b: Vec<isize> = left.iter().chain(right.iter()).cloned().collect();
        if test_safety(&b)? {
            return Ok(true);
        };
    }
    Ok(false)
}
