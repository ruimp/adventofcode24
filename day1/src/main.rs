use day1::{dist, read_file, simi};

fn main() {
    let file_path = String::from("input.txt");
    let data = read_file(file_path).unwrap();
    let distance = dist(&data);
    println!("Distance: {distance}");
    let similarity = simi(&data);
    println!("Similarity: {similarity}");
}
