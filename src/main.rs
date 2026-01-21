use crate::cpu::Cpu;

use std::fs::File;
use std::io::Read;   
use std::env;
use std::io;

mod constants;
mod cpu;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect(); //Returns an iterator containing the arguments passed to the program via the command line.

    if args.len() != 2{
        panic!("Usage: rvemu-simple <filename>");
    }
    
    let mut file = File::open(&args[1])?;
    let mut code = Vec::new();
    file.read_to_end(&mut code)?;

    let mut cpu = Cpu::new(code);
    
    loop{
        let inst = cpu.fetch();
        
        if inst == 0x00000000 {
            break;            
        }
        
        cpu.execute(inst);
    }

    Ok(())
}