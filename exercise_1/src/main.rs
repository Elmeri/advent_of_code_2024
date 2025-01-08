use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()>  {
    // Open the file
    let file= File::open("data/real_data.txt")?;

    let reader = io::BufReader::new(file);

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    let mut list3: Vec<i32> = Vec::new();
    let mut total_distance: i32 = 0;

    for line in reader.lines() {
        // Each line is a Result<String>, so we handle errors if needed
        let line = line?;
        let numbers: Vec<i32> = line
        .split_whitespace() // Split line into parts by whitespace
        .filter_map(|num| num.parse::<i32>().ok()) // Try to parse numbers and ignore invalid entries
        .collect();

        if numbers.len() == 2 {
            list1.push(numbers[0]);
            list2.push(numbers[1]);
        } else {
            eprintln!("Invalid line format: {}", line);
        }
    }

    list1.sort();
    list2.sort();

    for n in 0..=list1.len() - 1 {
        let distance: i32 = (list1[n] - list2[n]).abs();
        list3.push(distance);
        total_distance += distance;
    }

    println!("List 1: {:?}", list1);
    println!("List 2: {:?}", list2);
    println!("List 3: {:?}", list3); 
    println!("List total distance: {:?}", total_distance);
    Ok(())
}
