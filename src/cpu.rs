use crate::constants::{NUMS_REGS, MEMORY_SIZE};

pub struct Cpu{
    pub regs: [u64; NUMS_REGS],
    pub pc: u64,
    pub dram: Vec<u8>
}

impl Cpu{   
    pub fn new(code: Vec<u8>) -> Self{
        let mut regs = [0; NUMS_REGS];
        regs[2] = MEMORY_SIZE as u32 - 4;
        Self {
            regs: [0; 32],
            pc:0,
            dram: code
        }
    }
    
    pub fn fetch(&self) -> u32{
        let index = self.pc as usize;
        return(self.dram[index] as u32) 
        | ((self.dram[index + 1] as u32) <<  8)
        | ((self.dram[index + 2] as u32) << 16)
        | ((self.dram[index + 3] as u32) << 24);
    }
    
    pub fn execute(&mut self, inst:u32){
        let opcode = inst & 0x7f;
        let rd = ((inst >> 7) & 0x1f) as usize;
        let rs1 = ((inst >> 15) & 0x1f) as usize;
        let rs2 = ((inst >> 20) & 0x1f) as usize;
    
        match opcode{
            0x13 =>{
                let imm = ((inst & 0x0fff00000) as i32 as i64 >> 20) as u64;
                self.regs[rd] = self.regs[rs1].wrapping_add(imm);
            }
            0x33 =>{
                self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
            }
            _=>{
                dbg!("not implemented yet");
            }
        }
    }
}



    