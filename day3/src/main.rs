use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    // let file_path = String::from("test_input2.txt");
    let file_path = String::from("input.txt");
    let data = read_file(file_path)?;
    let total = parse(&data);
    println!("Total: {:?}", total);
    let valid_chunks = split_chunks(&data);
    let valid_total = val_parse(valid_chunks);
    println!("Valid total: {:?}", valid_total);
    Ok(())
}

fn read_file(s: String) -> Result<String> {
    let mut content: String = String::new();
    let file = File::open(s)?;
    let buffer = BufReader::new(file);
    for line in buffer.lines() {
        content += &line?;
    }
    Ok(content)
}

fn mul(s: &str) -> Option<usize> {
    let (x, rest) = s.split_once(",")?;
    let x = x.parse::<usize>().ok()?;
    let (y, _) = rest.split_once(")")?;
    let y = y.parse::<usize>().ok()?;
    Some(x * y)
}

fn parse(s: &str) -> usize {
    let chunks: Vec<&str> = s.split("mul(").collect();
    chunks.iter().filter_map(|s| mul(s)).sum()
}

fn split_chunks(s: &str) -> Vec<&str> {
    let mut parsed_chunks: Vec<&str> = Vec::new();
    let do_chunks: Vec<&str> = s.split_inclusive("do()").collect();
    for chunk in do_chunks.iter() {
        let chunks: Vec<&str> = chunk.split_inclusive("don't()").collect();
        parsed_chunks.extend(chunks);
    }
    parsed_chunks
}

fn val_parse(chunks: Vec<&str>) -> usize {
    let mut valid: bool = true;
    let mut total: usize = 0;
    for chunk in chunks.iter() {
        if valid {
            total += parse(chunk);
        }
        if chunk.ends_with("don't()") {
            valid = false;
        } else if chunk.ends_with("do()") {
            valid = true;
        }
    }
    total
}
