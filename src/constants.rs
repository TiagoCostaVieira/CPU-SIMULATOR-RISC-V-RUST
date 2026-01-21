// Opcode (bits 6..0)
pub const OP_IMM:  u32 = 0b0010011;
pub const LOAD:    u32 = 0b0000011;
pub const STORE:   u32 = 0b0100011;
pub const JALR:    u32 = 0b1100111;
pub const LUI:     u32 = 0b0110111;
pub const AUIPC:   u32 = 0b0010111;

// Masks
pub const OPCODE_MASK: u32 = 0x7F;


pub const NUMS_REGS: usize = 32;

pub const MEMORY_SIZE: u64 = 1024 * 1024 * 128;