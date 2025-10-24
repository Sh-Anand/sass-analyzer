use std::{fs::File, io::BufReader, io::BufRead};

use crate::sass::SassInstr;

struct SassParser {
    header: Vec<String>,
    reader: BufReader<File>
}

impl SassParser {
    pub fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let mut reader = BufReader::new(file);
        let mut header = Vec::new();
        
        let mut line_buffer = String::new();
        let mut in_text_section = false;
        
        loop {
            line_buffer.clear();
            if reader.read_line(&mut line_buffer).unwrap() == 0 {
                break;
            }
            
            if line_buffer.trim_start().starts_with("//--------------------- .text.") {
                in_text_section = true;
                header.push(line_buffer.trim_end().to_string());
                continue;
            }
            
            if in_text_section && line_buffer.trim_start().starts_with("/*") {
                break;
            }
            
            header.push(line_buffer.trim_end().to_string());
        }
        
        Self { header, reader }
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