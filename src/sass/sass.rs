use crate::sass_opcodes::SassOpcode;
pub struct SassInstr {
    pub opcode: SassOpcode,
    pub extension: Option<Vec<String>>,
    pub registers: Option<Vec<String>>,
    pub predicate: Option<String>,
}