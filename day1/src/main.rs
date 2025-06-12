use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let file_path = String::from("faulty_input.txt");
    // let file_path = String::from("test_input.txt");
    // let file_path = String::from("input.txt");

    let (a, b) = read_file(file_path)?;
    let similarity = simi(&a, &b);
    let distance = dist(&a, &b);
    println!("Distance: {}", distance);
    println!("Similarity: {}", similarity);
    Ok(())
}

fn read_file(file_path: String) -> Result<(Vec<usize>, Vec<usize>)> {
    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);
    let mut a: Vec<usize> = Vec::new();
    let mut b: Vec<usize> = Vec::new();
    for line in buf_reader.lines() {
        let nums: Vec<usize> = line?
            .split_whitespace()
            .filter_map(|s| match s.parse::<usize>() {
                Ok(s) => Some(s),
                Err(_) => {
                    println!("Skipping line with invalid input: {:?}", s);
                    Some(0)
                }
            })
            .collect();
        a.push(nums[0]);
        b.push(nums[1]);
    }
    Ok((a, b))
}

fn dist(a: &Vec<usize>, b: &Vec<usize>) -> usize {
    let mut a = a.clone();
    let mut b = b.clone();
    a.sort();
    b.sort();

    // sum of differences |x - y| for (x, y) in zip(a, b)
    a.iter().zip(b.iter()).map(|(x, y)| x.abs_diff(*y)).sum()
}

fn simi(a: &Vec<usize>, b: &Vec<usize>) -> usize {
    // for x in a, sum x for each y in b such that y = x
    a.iter()
        .filter_map(|x| Some(x * b.iter().filter(|&y| y == x).count()))
        .sum()
}
