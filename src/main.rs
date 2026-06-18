use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

struct CPU{
    reg : [u64;32],
    pc: u64,
    memory: Vec<u8> // 1 byte blocks
}

fn main() -> io::Result<()> {

    //loading file
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(args[1].clone())?;
    let mut code = Vec::new();
    let _= file.read_to_end(&mut code);

    //putting instructions in cpu memory
    let mut cpu = CPU::new(code);

    //cpu working
    while cpu.pc + 4  <= cpu.memory.len() as u64 {
        let instruction = cpu.fetch();
        cpu.pc = cpu.pc + 4;
        cpu.execute(instruction); 
    }
    for reg in cpu.reg{
        print!("\n{}",reg);
    }
    Ok(())
}

impl CPU{

    fn new(code: Vec<u8>) -> Self{
        //writing code in our cpu memory when initialiazed
        Self{
            reg: [0;32],
            pc: 0,
            memory: code
        }
    }
    fn fetch(&self) -> u32 {

        //index is set where pc is pointing so we can puzzle together u32 instruction (little-endian)
        let index = self.pc as usize;
        return   (self.memory[index] as u32)           |
                 ((self.memory[index + 1] as u32)<<8)  |
                 ((self.memory[index + 2] as u32)<<16) |
                 ((self.memory[index + 3] as u32)<<24);
               
    }
    fn execute(&mut self, instruction: u32){
        //decode
        let opcode = instruction & 0x7f;
        let rd = ((instruction >> 7 ) & 0x1f as u32) as usize;
        let rs1 = ((instruction >> 15 ) & 0x1f as u32) as usize;
        let rs2 = ((instruction >> 20 ) & 0x1f as u32) as usize;
        let imm = ((instruction as i32) >> 20) as i64 as u64; //for immidiate we need to save sign

        //execute
        match opcode {
            0x33 => //add
                self.reg[rd] = self.reg[rs1] + self.reg[rs2],
            0x13 => //addi
                self.reg[rd] = self.reg[rs1] + imm,
            _=> print!("Not supported command detected"),
        }
        
    } 
}
