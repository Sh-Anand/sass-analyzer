#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum SassOpcode {
    // Memory Operations
    STARTMEM = 0,
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
    ENDMEM,

    // Floating-Point Operations
    STARTFP,
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
    ENDFP,

    // Integer ALU Operations
    STARTALU,
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
    ENDALU,

    // Conversion Operations
    STARTCONV,
    I2F,
    I2I,
    F2F,
    F2I,
    FRND,
    ENDCONV,

    // Movement Operations
    STARTMOV,
    MOV,
    SHFL,
    PRMT,
    SEL,
    ENDMOV,

    // Special Register Operations
    STARTSR,
    S2R,
    CS2R,
    ENDSR,

    // Control Flow Operations
    STARTCF,
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
    ENDCF,
    
    // Predicate Operations
    STARTPRED,
    PLOP3,
    PSETP,
    P2R,
    R2P,
    ENDPRED,

    // Texture/Surface Operations
    STARTTEX,
    TEX,
    TLD,
    TLD4,
    TMML,
    TXQ,
    SULD,
    SUST,
    SUATOM,
    ENDTEX,

    // Video Operations
    STARTVIDEO,
    VMNMX,
    VABSDIFF,
    VADD,
    VSET,
    VSETP,
    ENDVIDEO,

    // Miscellaneous
    STARTMISC,
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
    ENDMISC,
}

impl SassOpcode {
    /// Check if opcode falls in range (start, end) (excluding both markers)
    const fn in_range(&self, start: Self, end: Self) -> bool {
        let val = *self as u16;
        let start_val = start as u16;
        let end_val = end as u16;
        val > start_val && val < end_val
    }
    
    pub const fn is_memory_op(&self) -> bool {
        self.in_range(Self::STARTMEM, Self::ENDMEM)
    }
    
    pub const fn is_fp_op(&self) -> bool {
        self.in_range(Self::STARTFP, Self::ENDFP)
    }
    
    pub const fn is_alu_op(&self) -> bool {
        self.in_range(Self::STARTALU, Self::ENDALU)
    }

    pub const fn is_control_flow(&self) -> bool {
        self.in_range(Self::STARTCF, Self::ENDCF)
    }

    pub const fn is_predicate_op(&self) -> bool {
        self.in_range(Self::STARTPRED, Self::ENDPRED)
    }

    pub const fn is_special_register_op(&self) -> bool {
        self.in_range(Self::STARTSR, Self::ENDSR)
    }

    pub const fn is_conversion_op(&self) -> bool {
        self.in_range(Self::STARTCONV, Self::ENDCONV)
    }

    pub const fn is_movement_op(&self) -> bool {
        self.in_range(Self::STARTMOV, Self::ENDMOV)
    }
}