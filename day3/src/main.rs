use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    // let file_path = String::from("test_input2.txt");
    let file_path = String::from("input.txt");
    let data = read_file(file_path)?;
    let total = parse(&data);
    println!("Total: {:?}", total);
    let chunks = val_split(&data);
    let valid_total = val_parse(chunks);
    println!("Valid total: {:?}", valid_total);
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

fn parse(s: &str) -> u32 {
    let chunks: Vec<&str> = s.split("mul(").collect();
    let muls: Vec<u32> = chunks.iter().filter_map(|s| mul(s)).collect();
    muls.iter().sum()
}

fn val_split(s: &str) -> Vec<&str> {
    let mut chunks: Vec<&str> = Vec::new();
    let do_chunks: Vec<&str> = s.split_inclusive("do()").collect();
    for do_chunk in do_chunks.iter() {
        let dont_chunk: Vec<&str> = do_chunk.split_inclusive("don't()").collect();
        for chunk in dont_chunk.iter() {
            chunks.push(&chunk)
        }
    }
    chunks
}

fn val_parse(ss: Vec<&str>) -> u32 {
    let mut valid: bool = true;
    let mut total: u32 = 0;
    for s in ss.iter() {
        if valid {
            total += parse(s);
        }
        if s.ends_with("don't()") {
            valid = false;
        } else if s.ends_with("do()") {
            valid = true;
        }
    }
    total
}
