use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    // let file_name = String::from("test_input.txt");
    let file_name = String::from("input.txt");
    let updates = get_updates(&file_name)?;
    let rules = get_rules(&file_name)?;
    let valid_count = get_valid_count(&updates, &rules);
    println!("Valid count: {:?}", valid_count);
    Ok(())
}

fn get_valid_count(updates: &Vec<Vec<usize>>, rules: &Vec<Vec<usize>>) -> usize {
    let mut count: usize = 0;
    for update in updates.iter() {
        let validity = filter_update(update, rules);
        if validity {
            count += update[update.len() / 2]
        }
    }
    count
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

fn filter_update(update: &Vec<usize>, rules: &Vec<Vec<usize>>) -> bool {
    for (i, page) in update.iter().enumerate() {
        for rule in rules.iter() {
            if rule.first() == Some(page) {
                if update[..i].contains(rule.last().unwrap()) {
                    return false;
                }
            }
        }
    }
    true
}
