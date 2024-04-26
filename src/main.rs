use itertools::Itertools;
use rayon::prelude::*;
use std::io::BufReader;
use std::{fs::File, io::BufRead};

use anyhow::Result;

mod model;
mod parser;

fn main() -> Result<()> {
    for i in 1..=20 {
        let path = format!("raw/20240424_Public{i:02}.xml");
        eprintln!("{path}");
        let file = BufReader::new(File::open(path)?);
        for chunk in &file.lines().enumerate().chunks(65535) {
            let mut todo = Vec::new();
            for (i, line) in chunk {
                let line = line?;
                if !(i < 4 || line == "</Transfer>") {
                    todo.push(line);
                }
            }
            let chunk: Vec<_> = todo
                .par_iter()
                .map(|line| parser::parse_record(line))
                .collect();
            for record in chunk {
                match record {
                    Ok(x) => println!("{}", serde_json::to_string(&x)?),
                    Err(e) => eprintln!("{e}"),
                };
            }
        }
    }

    Ok(())
}
