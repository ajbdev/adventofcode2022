use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./inputs.txt")?;
    let reader = BufReader::new(file);
    let mut current: i32 = 0;

    let mut v: Vec<i32> = vec![];

    for line in reader.lines() {
        let line = line?;

        if line.trim().is_empty() {
            v.push(current);
            current = 0;
        } else {
            let calories: i32 = line.parse().unwrap();
            current += calories;
        }
    }
    v.sort();

    let top3 = v.pop().unwrap() + v.pop().unwrap() + v.pop().unwrap();

    println!("{}", top3);

    Ok(())
}