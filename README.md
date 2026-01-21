# 🖥️ CPU Simulator in Rust (RISC-V)

This project is a **CPU simulator written in Rust**, focused on education and experimentation, inspired by the **RISC-V architecture**.  
Its goal is to explore **computer architecture**, **instruction execution**, **register modeling**, **memory**, and the **fetch–decode–execute cycle** in a clear and incremental way.

---

## 🧠 Architecture Overview

The CPU is explicitly modeled as a stateful structure:

```rust
pub struct Cpu {
    pub regs: [u64; NUMS_REGS],
    pub pc: u64,
    pub dram: Vec<u8>
}
