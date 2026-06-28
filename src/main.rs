mod cpu;
mod dram;
mod bus;

use crate::dram::*;
use crate::cpu::*;

use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {

    //loading file
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid arguments");
    }
    let mut file = File::open(&args[1])?;
    let mut code = Vec::new();
    file.read_to_end(&mut code)?;

    //putting instructions in cpu memory
    let code_len = code.len(); // saving length for the loop
    let mut cpu = CPU::new(code);

    loop{
        // 1. fetch
        let instruction = match cpu.fetch() {       
            Ok(instruction) => instruction,
            Err(_) => break,    // '_' -> throwaway
        };

        cpu.pc += 4; // instruction is always 32 = 8 * 4 bits wide

        //2.decode
        //3.execute

        match cpu.decode_execute(instruction){
            Ok(_) => {},
            Err(_) => break,
        }

        //avoiding infinite loop
        if cpu.pc - DRAM_BASE + 4 > code_len as u64 { break; } 

    }

    for reg in cpu.reg{
        print!("\n{}",reg);
    }
    Ok(())
}


