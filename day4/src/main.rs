use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive(Debug)]
struct Data {
    data: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

// Custom printing of input data
impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s: String = String::from("Data:\n");
        for row in self.data.iter() {
            s += "    ";
            for c in row {
                s += &c.to_string();
                s += " "
            }
            s += "\n";
        }
        write!(f, "{}", s)
    }
}

fn main() -> Result<()> {
    let file_path = String::from("test_input2.txt");
    // let file_path = String::from("input.txt");
    let data = read_file(file_path)?;
    println!("{}", &data);
    let count_rows = scan_rows(&data);
    println!("Row count: {:?}", count_rows);
    let count_cols = scan_cols(&data);
    println!("Column count: {:?}", count_cols);
    let count_diags = scan_diags(&data);
    println!("Diagonal count: {:?}", count_diags);
    let total_count = count_rows + count_cols + count_diags;
    println!("Total: {:?}", total_count);
    Ok(())
}

fn read_file(s: String) -> Result<Data> {
    let mut content: Vec<Vec<char>> = Vec::new();
    let file = File::open(s)?;
    let buffer = BufReader::new(file);
    for line in buffer.lines() {
        let line: Vec<char> = line?.chars().collect();
        content.push(line);
    }
    let data = Data {
        data: content.clone(),
        rows: content.len(),
        cols: content[0].len(),
    };
    Ok(data)
}

fn scan_rows(m: &Data) -> usize {
    let mut count: usize = 0;
    for i in 0..m.rows {
        let mut s: String = String::new();
        for j in 0..m.cols {
            s.push(m.data[i][j]);
        }
        count += s.matches("XMAS").count();
        count += s.matches("SAMX").count();
    }
    count
}

fn scan_cols(m: &Data) -> usize {
    let mut count: usize = 0;
    for i in 0..m.rows {
        let mut s: String = String::new();
        for j in 0..m.cols {
            s.push(m.data[j][i]);
        }
        count += s.matches("XMAS").count();
        count += s.matches("SAMX").count();
    }
    count
}

fn scan_diags(m: &Data) -> usize {
    let mut count: usize = 0;
    for i in 0..m.rows {
        let mut s = String::new();
        for j in 0..m.cols.min(i + 1) {
            s.push(m.data[m.rows - i + j - 1][j])
        }
        count += s.matches("XMAS").count();
        count += s.matches("SAMX").count();
    }
    for i in 0..m.cols - 1 {
        let mut s = String::new();
        for j in 0..m.rows.min(i + 1) {
            s.push(m.data[j][m.rows - i + j - 1])
        }
        count += s.matches("XMAS").count();
        count += s.matches("SAMX").count();
    }
    for i in 0..m.rows {
        let mut s = String::new();
        for j in 0..m.cols.min(i + 1) {
            s.push(m.data[i - j][j])
        }
        count += s.matches("XMAS").count();
        count += s.matches("SAMX").count();
    }
    for i in 0..m.cols - 1 {
        let mut s = String::new();
        for j in 0..m.rows.min(i + 1) {
            s.push(m.data[m.rows - j - 1][m.cols - i + j - 1])
        }
        count += s.matches("XMAS").count();
        count += s.matches("SAMX").count();
    }
    count
}
