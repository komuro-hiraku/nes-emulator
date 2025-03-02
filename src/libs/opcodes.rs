use lazy_static::lazy_static;
use crate::libs::cpu::AddressingMode;


lazy_static! {
    static ref CPU_OPS_CODES: Vec<OpCode> = vec![
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),
        OpCode::new(0xaa, "TAX", 1, 2, AddressingMode::NoneAddressing),

        /* Stores, Loads */
        OpCode::new(0xa9, "LDA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa5, "LDA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb5, "LDA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xad, "LDA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbd, "LDA", 3, 4 /*+1 if page crossed */, AddressingMode::Absolute_X),
        OpCode::new(0xb9, "LDA", 3, 4 /*+1 if page crossed */, AddressingMode::Absolute_Y),
        OpCode::new(0xa1, "LDA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0xb1, "LDA", 2, 5 /*+1 if page crossed */, AddressingMode::Indirect_Y),
    ];
}

pub struct OpCode {
    op_code: u8,
    inst: &'static str,
    bytes: u8,
    cycles: u8,
    mode: AddressingMode,
}

impl OpCode {
    pub fn new(op_code: u8, inst: &'static str, bytes:u8, cycles:u8, mode: AddressingMode) -> Self {
        OpCode {
            op_code,
            inst,
            bytes,
            cycles,
            mode
        }
    }
}
