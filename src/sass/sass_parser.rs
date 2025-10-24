use std::{fs::File, io::BufReader, io::BufRead};

use crate::sass::SassInstr;

struct SassParser {
    reader: BufReader<File>
}

impl SassParser {
    pub fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        Self { reader }
    }

    pub fn parse(&mut self) -> Result<Vec<SassInstr>, ()> {
        let mut instructions = Vec::new();
        let reader = &mut self.reader;
        reader
            .lines()
            .map(|line| line.expect("").trim().split_once("*/").expect("failed to split line").1.split_once(";").expect("failed to split line").0.trim().to_string());
        Ok(instructions)
    }
}