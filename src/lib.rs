use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn process_files<F>(files: &Vec<String>, mut consumer: F) -> std::io::Result<()>
where
    F: FnMut(String),
{
    if files.first().unwrap().eq("-") {
        let buffer = BufReader::new(std::io::stdin());

        for line in buffer.lines() {
            consumer(line?);
        }
    } else {
        for fname in files {
            let buffer = BufReader::new(File::open(fname)?);

            for line in buffer.lines() {
                consumer(line?);
            }
        }
    }

    Ok(())
}
