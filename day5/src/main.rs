use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    // let file_name = String::from("test_input.txt");
    let file_name = String::from("input.txt");
    let updates = get_updates(&file_name)?;
    let rules = get_rules(&file_name)?;
    let valid_count = get_valid_count(&updates, &rules);
    println!("Valid count: {:?}", valid_count);
    let sorted_count = get_sorted_count(&updates, &rules);
    println!("Sorted update: {:?}", sorted_count);
    println!("Invalid count: {:?}", sorted_count - valid_count);
    Ok(())
}

fn get_rules(file_name: &String) -> Result<Vec<Vec<usize>>> {
    let mut rules: Vec<Vec<usize>> = Vec::new();
    let file = File::open(file_name)?;
    let buffer = BufReader::new(file);
    for line in buffer.lines() {
        let line = line?;
        if line.contains("|") {
            let rule: Vec<usize> = line
                .split("|")
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();
            rules.push(rule);
        }
    }
    Ok(rules)
}

fn get_updates(file_name: &String) -> Result<Vec<Vec<usize>>> {
    let mut updates: Vec<Vec<usize>> = Vec::new();
    let file = File::open(file_name)?;
    let buffer = BufReader::new(file);
    for line in buffer.lines() {
        let line = line?;
        if !line.is_empty() && !line.contains("|") {
            let update: Vec<usize> = line
                .split(",")
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();
            updates.push(update);
        }
    }
    Ok(updates)
}

fn compare(x: &usize, y: &usize, rules: &Vec<Vec<usize>>) -> Ordering {
    match rules.iter().find(|&r| &r[0] == x && &r[1] == y) {
        Some(_) => Ordering::Less,
        None => Ordering::Greater,
    }
}

fn get_valid_count(updates: &Vec<Vec<usize>>, rules: &Vec<Vec<usize>>) -> usize {
    updates
        .iter()
        .filter(|&u| {
            u.iter().is_sorted_by(|a, b| match compare(a, b, &rules) {
                Ordering::Less => true,
                _ => false,
            })
        })
        .map(|u| u[u.len() / 2])
        .sum()
}

fn get_sorted_count(updates: &Vec<Vec<usize>>, rules: &Vec<Vec<usize>>) -> usize {
    let mut count: usize = 0;
    for update in updates.iter() {
        let mut sorted_update = update.clone();
        sorted_update.sort_by(|x, y| compare(x, y, &rules));
        count += sorted_update[sorted_update.len() / 2]
    }
    count
}
