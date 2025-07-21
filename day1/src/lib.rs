use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive(Debug, Clone)]
pub struct Data {
    col1: Vec<usize>,
    col2: Vec<usize>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            col1: Vec::new(),
            col2: Vec::new(),
        }
    }
}

pub fn read_file(file_path: impl AsRef<std::path::Path>) -> Result<Data> {
    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);
    let mut data = Data::new();
    for line in buf_reader.lines() {
        let line = line?;
        let mut nums = line
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok());
        if let (Some(a), Some(b)) = (nums.next(), nums.next()) {
            data.col1.push(a);
            data.col2.push(b);
        }
    }
    Ok(data)
}

pub fn dist(data: &Data) -> usize {
    let mut col1_sorted = data.col1.clone();
    let mut col2_sorted = data.col2.clone();
    col1_sorted.sort();
    col2_sorted.sort();
    col1_sorted
        .iter()
        .zip(col2_sorted.iter())
        .map(|(x, y)| x.abs_diff(*y))
        .sum()
}

pub fn simi(data: &Data) -> usize {
    let mut freqs = HashMap::new();
    for x in &data.col2 {
        *freqs.entry(x).or_insert(0) += 1;
    }
    data.col1
        .iter()
        .map(|x| x * freqs.get(&x).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let file_path = String::from("test_input.txt");
        let data = read_file(file_path).unwrap();
        assert_eq!(dist(&data), 11);
    }

    #[test]
    fn test_similarity() {
        let file_path = String::from("test_input.txt");
        let data = read_file(file_path).unwrap();
        assert_eq!(simi(&data), 31);
    }
}
