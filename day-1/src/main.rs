use std::fs::File;
use std::io::{self, prelude::*, BufReader};
fn main() -> io::Result<()> {
    let file = File::open("./inputs.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut current: i32 = 0;
        let mut largest: i32 = 0;

        if line?.trim().is_empty() {
            
        } else {
            let cals: i32 = line?.parse().unwrap();
            current += cals;
            
        }
    }

    Ok(())
}