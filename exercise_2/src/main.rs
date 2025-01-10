use std::fs::File;
use std::io::{self, BufRead};

fn have_same_sign(a: i32, b: i32) -> bool {
    (a >= 0 && b >= 0) || (a < 0 && b < 0)
}

fn main() -> io::Result<()>  {
    // Open the file
    let file= File::open("data/real_data.txt")?;

    let reader = io::BufReader::new(file);

    let mut report_vector: Vec<Vec<i32>> = Vec::new();
    let mut total_safe_reports: i32 = 0;
    let acceptable_increments: Vec<i32> = vec![1, 2, 3];

    for line in reader.lines() {
        // Each line is a Result<String>, so we handle errors if needed
        let line = line?;
        let levels: Vec<i32> = line
        .split_whitespace() // Split line into parts by whitespace
        .filter_map(|num| num.parse::<i32>().ok()) // Try to parse numbers and ignore invalid entries
        .collect();


        report_vector.push(levels);
    }

    // calculate similarity score
    for i in 0..=report_vector.len() - 1 {
        let mut valid_increases: usize = 0;
        let increase_sign: i32 = report_vector[i][1] - report_vector[i][0];
        for j in 1..=report_vector[i].len() -1 {
            let increase: i32 = report_vector[i][j] - report_vector[i][j-1];
            if acceptable_increments.contains(&(increase).abs()) && have_same_sign(increase_sign, increase) {
                valid_increases += 1
            }
        }
        if valid_increases == report_vector[i].len() -1 {
            total_safe_reports += 1
        }
    }

    println!("report_vector: {:?}", report_vector);
    println!("Total safe reports: {:?}", total_safe_reports);
    Ok(())
}
