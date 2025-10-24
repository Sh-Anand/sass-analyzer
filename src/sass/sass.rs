use std::str::FromStr;

use crate::sass_opcodes::SassOpcode;
pub struct SassInstr {
    pub opcode: SassOpcode,
    pub extension: Option<Vec<String>>,
    pub registers: Option<Vec<String>>,
    pub predicate: Option<String>,
}

impl FromStr for SassInstr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let opcode = SassOpcode::from_str(s).expect("failed to parse opcode");
        Ok(Self { opcode, extension: None, registers: None, predicate: None })
    }
}