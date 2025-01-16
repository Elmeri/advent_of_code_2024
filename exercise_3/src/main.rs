use std::fs::File;
use std::io::{self, BufRead};


fn main() -> io::Result<()>  {
    // Open the file
    let file= File::open("data/example_data.txt")?;

    let reader = io::BufReader::new(file);

    let mut report_vector: Vec<String> = Vec::new();


    for line in reader.lines() {
        // Each line is a Result<String>, so we handle errors if needed
        let line = line?;
        report_vector.push(line);
    }

    // calculate similarity score

    println!("report_vector: {:?}", report_vector);

    Ok(())
}
