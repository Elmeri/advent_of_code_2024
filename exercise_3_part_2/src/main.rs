use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn mul(v: Vec<i32>) -> i32 {
    v[0] * v[1]
}

fn main() -> io::Result<()>  {
    // Open the file
    let file= File::open("data/real_data.txt")?;

    let reader = io::BufReader::new(file);

    let mut line_vector: Vec<String> = Vec::new();
    let mut report_vector: Vec<String> = Vec::new();
    let mut mul_vector: Vec<i32> = Vec::new();
    let mut mul_vector_2: Vec<i32> = Vec::new();
    let mut data_string: String = String::new();

    for line in reader.lines() {
        // Each line is a Result<String>, so we handle errors if needed
        let line = line?;
        line_vector.push(line.clone());
        data_string.push_str(&line);
    }
    // Reference starts here //
    let re: Regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    for line in line_vector.iter() {
        // Each line is a Result<String>, so we handle errors if needed
        let line = line;
        report_vector.push(line.clone());

        for mat in re.find_iter(&line) {
            mul_vector.push(mul(mat.as_str().replace("mul(", "").replace(")", "").split(",").map(|x|->i32{x.parse().unwrap()}).collect()));
        }
    }
    // Reference ends here //
    let re_2: Regex = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    let new = re_2.replace_all(&data_string, "");
    println!("new: {:?}", new);


    let re_3: Regex = Regex::new(r"don't\(\).*").unwrap();
    let new_2 = re_3.replace_all(&new, "");
    println!("new 2: {:?}", new_2);

    for mat in re.find_iter(&new_2) {
        mul_vector_2.push(mul(mat.as_str().replace("mul(", "").replace(")", "").split(",").map(|x|->i32{x.parse().unwrap()}).collect()));
    }

    println!("mul_vector: {:?}", mul_vector);
    println!("mul_vector_2: {:?}", mul_vector_2);
    let sum: i32 = mul_vector.iter().sum();
    println!("sum: {:?}", sum);
    let sum_2: i32 = mul_vector_2.iter().sum();
    println!("sum_2: {:?}", sum_2);
    Ok(())
}
