use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

/// A input handle is either Some file path or None for stdin
#[derive(Clone)]
pub struct InputHandle {
    path: Option<PathBuf>,
}

impl FromStr for InputHandle {
    type Err = String;

    fn from_str(path: &str) -> Result<Self, Self::Err> {
        if path.ne("-") {
            let p = PathBuf::from(&path);
            if p.is_file() && p.exists() {
                Ok(InputHandle { path: Some(p) })
            } else {
                Err(format!("{path} is not a valid input file"))
            }
        } else {
            Ok(InputHandle { path: None })
        }
    }
}

pub fn process_files<F>(inputs: &Vec<InputHandle>, mut consumer: F) -> std::io::Result<()>
where
    F: FnMut(String),
{
    match inputs.first().unwrap().path {
        Some(_) => {
            for input in inputs {
                if let Some(i) = &input.path {
                    let buffer = BufReader::new(File::open(i)?);

                    for line in buffer.lines() {
                        consumer(line?);
                    }
                }
            }
        }
        None => {
            let buffer = BufReader::new(std::io::stdin());

            for line in buffer.lines() {
                consumer(line?);
            }
        }
    }

    Ok(())
}
