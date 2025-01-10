use std::fs::File;
use std::io::{self, BufRead};

fn have_same_sign(a: i32, b: i32) -> bool {
    (a >= 0 && b >= 0) || (a < 0 && b < 0)
}

fn calculate_increases(level_vector: &Vec<i32>, acceptable_increments: &Vec<i32>) -> usize {
    let mut valid_increases: usize = 0;
    let increase_sign: i32 = level_vector[1] - level_vector[0];
    println!("increase_sign: {:?}", increase_sign);
    for j in 1..=level_vector.len() -1 {
        let increase: i32 = level_vector[j] - level_vector[j-1];
        if acceptable_increments.contains(&(increase).abs()) && have_same_sign(increase_sign, increase) {
            valid_increases += 1;
        }
    }
    valid_increases
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
        let mut invalid_increase_index: Vec<usize> = Vec::new();
        let mut invalid_increases = 0;
        let mut flag: bool = false;
        for j in 1..=report_vector[i].len() -1 {
            let increase: i32 = report_vector[i][j] - report_vector[i][j-1];
            if acceptable_increments.contains(&(increase).abs()) && have_same_sign(increase_sign, increase) {
                valid_increases += 1;
            } else {
                invalid_increase_index.push(j);
                invalid_increases += 1;
            }
        }
        println!("level_vector: {:?}, invalid_increase_index: {:?} , invalid_increases: {:?} ", report_vector[i] , invalid_increase_index, invalid_increases);
        if valid_increases == report_vector[i].len() -1 {
            total_safe_reports += 1
        } else {
            println!("report_vector[i]: {:?}", report_vector[i]);
            for k in 0..=report_vector[i].len() - 1 {
                let mut level_vector: Vec<i32> = report_vector[i].clone();
                level_vector.remove(k);
                println!("level_vector: {:?}", level_vector);
                let subset_valid_increases: usize = calculate_increases(&level_vector, &acceptable_increments);
                if subset_valid_increases == level_vector.len() -1 {
                    total_safe_reports += 1;
                    flag = true;
                    break;
                }
            }
        }
        if invalid_increases > 0 && flag != true{ // if the invalid number is the first number
            let mut level_vector: Vec<i32> = report_vector[i].clone();
            level_vector.remove(0);
            println!("level_vector without first: {:?}", level_vector);
            let subset_valid_increases: usize = calculate_increases(&level_vector, &acceptable_increments);
            if subset_valid_increases == level_vector.len() -1 {
                total_safe_reports += 1
            }
        }
    }

    println!("report_vector: {:?}", report_vector);
    println!("Total safe reports: {:?}", total_safe_reports); // 347 is too low
    Ok(())
}
