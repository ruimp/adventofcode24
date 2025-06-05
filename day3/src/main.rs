// use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let file_path = String::from("test_input2.txt");
    // let file_path = String::from("input.txt");
    let data = read_file(file_path)?;
    // println!("{:?}", data);
    let total = proc(&data);
    println!("{:?}", total);
    Ok(())
}

fn read_file(s: String) -> Result<String> {
    let mut content: String = String::new();
    let file = File::open(s)?;
    let buffer = BufReader::new(file);
    for line in buffer.lines() {
        let line = line?;
        content += &line;
    }
    Ok(content)
}

fn mul(s: &str) -> Option<u32> {
    let (x, rest) = s.split_once(",")?;
    let x = x.parse::<u32>().ok()?;
    let (y, _) = rest.split_once(")")?;
    let y = y.parse::<u32>().ok()?;
    Some(x * y)
}

fn proc(s: &str) -> u32 {
    let chunks: Vec<&str> = s.split("mul(").collect();
    let muls: Vec<u32> = chunks.iter().filter_map(|s| mul(s)).collect();
    // println!("{:?}", muls);
    muls.iter().sum()
}
