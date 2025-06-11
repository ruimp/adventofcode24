use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let file_path = String::from("test_input.txt");
    let content = read_file(file_path)?;
    println!("{:?}", content);
    let h = content.len();
    let w = content[0].len();
    println!("Height: {:?}   |   Width: {:?}", h, w);
    let count_rows = scan_rows(&content);
    println!("Row count: {:?}", count_rows);
    let count_cols = scan_cols(&content);
    println!("Column count: {:?}", count_cols);
    Ok(())
}

fn read_file(s: String) -> Result<Vec<Vec<char>>> {
    let mut content: Vec<Vec<char>> = Vec::new();
    let file = File::open(s)?;
    let buffer = BufReader::new(file);
    for line in buffer.lines() {
        let line: Vec<char> = line?.chars().collect();
        content.push(line);
    }
    Ok(content)
}

fn scan_rows(m: &Vec<Vec<char>>) -> usize {
    let mut count: usize = 0;
    for row in m.iter() {
        let s: String = row.into_iter().collect();
        count += s.matches("XMAS").count();
        count += s.matches("SAMX").count();
    }
    count
}

fn scan_cols(m: &Vec<Vec<char>>) -> usize {
    let n_rows = m.len();
    let n_cols = m[0].len();
    let mut count: usize = 0;
    for i in 1..n_cols {
        let mut s: String = String::new();
        for j in 1..n_rows {
            s.push(m[j][i]);
        }
        count += s.matches("XMAS").count();
        count += s.matches("SAMX").count();
    }
    count
}
