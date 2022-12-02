use std::fs::File;
use std::io::{self, prelude::*, BufReader};
fn main() -> io::Result<()> {
    let file = File::open("./inputs.txt")?;
    let reader = BufReader::new(file);
    let mut largest: i32 = 0;
    let mut current: i32 = 0;

    for line in reader.lines() {
        let line = line?;

        if line.trim().is_empty() {
            if current > largest {
                largest = current;
            }
            current = 0;
        } else {
            let calories: i32 = line.parse().unwrap();
            current += calories;
        }
    }
    println!("{}", largest);

    Ok(())
}