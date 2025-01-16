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

    let mut report_vector: Vec<String> = Vec::new();
    let mut mul_vector: Vec<i32> = Vec::new();


    for line in reader.lines() {
        // Each line is a Result<String>, so we handle errors if needed
        let re: Regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
        let line = line?;
        report_vector.push(line.clone());

        for mat in re.find_iter(&line) {
            println!("{:?}", mat.as_str());
            mul_vector.push(mul(mat.as_str().replace("mul(", "").replace(")", "").split(",").map(|x|->i32{x.parse().unwrap()}).collect()));
        }
    }

    // calculate similarity score

    println!("report_vector: {:?}", report_vector);
    println!("mul_vector: {:?}", mul_vector);
    let sum: i32 = mul_vector.iter().sum();
    println!("sum: {:?}", sum);

    Ok(())
}
