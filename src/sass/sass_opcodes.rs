#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SassOpcode {
    // Memory Operations
    LD,
    LDG,
    LDS,
    LDL,
    LDGSTS,
    ST,
    STG,
    STS,
    STL,
    ATOM,
    ATOMG,
    ATOMS,
    RED,
    CCTL,
    MEMBAR,
    
    // Floating-Point Operations
    FADD,
    FMUL,
    FFMA,
    FMNMX,
    FSET,
    FSETP,
    FCMP,
    MUFU,
    HADD2,
    HMUL2,
    HFMA2,
    HSET2,
    HSETP2,
    
    // Integer ALU Operations
    IADD,
    IADD3,
    UIADD3,
    IMAD,
    IMUL,
    IMNMX,
    ISETP,
    ISET,
    ICMP,
    LOP3,
    LEA,
    SHF,
    SHL,
    SHR,
    POPC,
    FLO,
    
    // Conversion Operations  
    I2F,
    I2I,
    F2F,
    F2I,
    FRND,
    
    // Movement Operations
    MOV,
    SHFL,
    PRMT,
    SEL,
    
    // Special Register Operations
    S2R,
    CS2R,
    
    // Control Flow Operations
    BRA,
    JMP,
    JMX,
    BRX,
    CALL,
    RET,
    EXIT,
    BRK,
    CONT,
    SSY,
    SYNC,
    BSSY,
    BSYNC,
    
    // Predicate Operations
    PLOP3,
    PSETP,
    P2R,
    R2P,
    
    // Texture/Surface Operations
    TEX,
    TLD,
    TLD4,
    TMML,
    TXQ,
    SULD,
    SUST,
    SUATOM,
    
    // Video Operations
    VMNMX,
    VABSDIFF,
    VADD,
    VSET,
    VSETP,
    
    // Miscellaneous
    NOP,
    BAR,
    B2R,
    R2B,
    VOTE,
    BMOV,
    IPA,
    DEPBAR,
    GETCRSPTR,
    GETLMEMBASE,
    SETCRSPTR,
    SETLMEMBASE,
    ULDC,
    AL2P,
}

impl SassOpcode {    
    pub fn is_memory_op(&self) -> bool {
        Self::memory_ops().contains(self)
    }
    
    pub fn is_fp_op(&self) -> bool {
        Self::fp_ops().contains(self)
    }
    
    pub fn is_alu_op(&self) -> bool {
        Self::alu_ops().contains(self)
    }

    pub fn is_control_flow(&self) -> bool {
        Self::control_flow_ops().contains(self)
    }

    pub fn is_predicate_op(&self) -> bool {
        Self::predicate_ops().contains(self)
    }

    pub fn is_special_register_op(&self) -> bool {
        Self::special_register_ops().contains(self)
    }

    pub fn is_conversion_op(&self) -> bool {
        Self::conversion_ops().contains(self)
    }

    pub fn is_movement_op(&self) -> bool {
        Self::movement_ops().contains(self)
    }

    pub const fn memory_ops() -> &'static [SassOpcode] {
        &[
            Self::LD, Self::LDG, Self::LDS, Self::LDL, Self::LDGSTS,
            Self::ST, Self::STG, Self::STS, Self::STL,
            Self::ATOM, Self::ATOMG, Self::ATOMS,
            Self::RED, Self::CCTL, Self::MEMBAR,
        ]
    }
    
    pub const fn fp_ops() -> &'static [SassOpcode] {
        &[
            Self::FADD, Self::FMUL, Self::FFMA, Self::FMNMX,
            Self::FSET, Self::FSETP, Self::FCMP, Self::MUFU,
            Self::HADD2, Self::HMUL2, Self::HFMA2,
            Self::HSET2, Self::HSETP2,
        ]
    }
    
    pub const fn alu_ops() -> &'static [SassOpcode] {
        &[
            Self::IADD, Self::IADD3, Self::UIADD3, Self::IMAD, Self::IMUL,
            Self::IMNMX, Self::ISETP, Self::ISET, Self::ICMP,
            Self::LOP3, Self::LEA, Self::SHF, Self::SHL, Self::SHR,
            Self::POPC, Self::FLO,
        ]
    }
    
    pub const fn control_flow_ops() -> &'static [SassOpcode] {
        &[
            Self::BRA, Self::JMP, Self::JMX, Self::BRX,
            Self::CALL, Self::RET, Self::EXIT,
            Self::BRK, Self::CONT,
            Self::SSY, Self::SYNC, Self::BSSY, Self::BSYNC,
        ]
    }

    pub const fn predicate_ops() -> &'static [SassOpcode] {
        &[
            Self::PLOP3, Self::PSETP, Self::P2R, Self::R2P,
        ]
    }

    pub const fn special_register_ops() -> &'static [SassOpcode] {
        &[
            Self::S2R, Self::CS2R,
        ]
    }

    pub const fn conversion_ops() -> &'static [SassOpcode] {
        &[
            Self::I2F, Self::I2I, Self::F2F, Self::F2I, Self::FRND,
        ]
    }


    pub const fn movement_ops() -> &'static [SassOpcode] {
        &[
            Self::MOV, Self::SHFL, Self::PRMT, Self::SEL,
        ]
    }
}