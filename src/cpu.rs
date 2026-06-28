use crate::bus::Bus;
use crate::dram::DRAM_BASE;

pub struct CPU{ //pub allows other modules to use cpu structure and methods
    pub reg : [u64;32],
    pub pc: u64,
    pub system_bus: Bus // talking to memory (dram) throw system bus
}

impl CPU{

    pub fn new(code: Vec<u8>) -> Self{
        //writing code in our cpu (DRAM) memory when initialiazed
        Self{
            reg: [0;32],
            pc: DRAM_BASE,
            system_bus: Bus::new(code),
        }
    }
    pub fn fetch(&mut self) -> Result<u32,()> {
        Ok(self.system_bus.load(self.pc,32)? as u32)
    }
    pub fn decode_execute(&mut self, instruction: u32) -> Result<(),()>{

        self.reg[0] = 0; // zero register - always hardwired to ground (0)

        //decode

        //always on that place so we can extract before decoding
        let opcode = instruction & 0x7f;
        let rd = ((instruction >> 7 ) & 0x1f as u32) as usize;
        let rs1 = ((instruction >> 15 ) & 0x1f as u32) as usize;       
        let rs2 = ((instruction >> 20 ) & 0x1f as u32) as usize;
        let func3 =((instruction >> 12) & 0x7 as u32) as usize;


        //execute
        match opcode {            
            0x03 =>{ //LOAD INSTRUCTIONS ( I type )
            
                // imm[11:0] for I type instructions
                let imm = ((instruction as i32) >> 20) as i64 as u64; // casting form i64 to u64 we sign extended imm
                // wrapping_add insted of '+' to avoid causing an arithmetic overflow when the result is beyond 64 bits
                let address = self.reg[rs1].wrapping_add(imm); // address = rs1 + sext(offset)
                match func3 { // is it 8/16/32/64 bits and is it sext or ext
                    0x0=>{ //lb
                        let value = self.system_bus.load(address,8)?;
                        self.reg[rd] = value as i8 as i64 as u64;               //rd = sext(M[rs1 + sext(offset)])[0:7]
                        Ok(())                
                    },
                    0x1=>{ //lh
                        let value = self.system_bus.load(address,16)?; 
                        self.reg[rd] = value as i16 as i64 as u64;              //rd = sext(M[rs1 + sext(offset)])[0:15]   
                        Ok(())              
                    },
                    0x2=>{ //lw
                        let value = self.system_bus.load(address,32)?; 
                        self.reg[rd] = value as i32 as i64 as u64;              //rd = sext(M[rs1 + sext(offset)])[0:31]
                        Ok(())                 
                    },
                    0x3=>{ //ld
                        let value = self.system_bus.load(address,64)?; 
                        self.reg[rd] = value as i64 as u64;                     //rd = M[rs1 + sext(offset)][0:63]
                        Ok(())                        
                    },
                    0x4=>{ //lbu
                        let value = self.system_bus.load(address,8)?; 
                        self.reg[rd] = value;                                   //rd = ext(M[rs1 + sext(offset)])[0:7]
                        Ok(())                                      
                    },
                    0x5=>{ //lhu
                        let value = self.system_bus.load(address,16)?; 
                        self.reg[rd] = value;                                   //rd = ext(M[rs1 + sext(offset)])[0:15]
                        Ok(())                                     
                    },
                    0x6=>{ //lwu
                        let value = self.system_bus.load(address,32)?; 
                        self.reg[rd] = value;                                   //rd = ext(M[rs1 + sext(offset)])[0:31]
                        Ok(())                                      
                    },
                    _=> {print!("Invalid load instruction"); Err(())},
                }
            }

            0x23 =>{ //STORE INSTRUCTIONS ( S type )
                let imm = (((instruction >> 7 as i32) & 0x1f) | ((instruction >> 20 as i32) & 0xfe000000)) as i64 as u64 ;
                let address = self.reg[rs1].wrapping_add(imm);
                match func3{ // is it 8/16/32/64 bits
                    0x0 =>{ 
                        self.system_bus.store(address,self.reg[rs2],8)?;     //sb M[rs1 + sext(offset)] = rs2[0:7]
                        Ok(())
                    },        
                    0x1 =>{ 
                        self.system_bus.store(address,self.reg[rs2],16)?;     //sh M[rs1 + sext(offset)] = rs2[0:15]
                        Ok(())
                    },     
                    0x2 =>{ 
                        self.system_bus.store(address,self.reg[rs2],32)?;     //sw M[rs1 + sext(offset)] = rs2[0:31]
                        Ok(())
                    },
                    0x3 =>{ 
                        self.system_bus.store(address,self.reg[rs2],64)?;     //sd M[rs1 + sext(offset)] = rs2[0:63]
                        Ok(())
                    },
                    _ => {Err(())},
                }
            }
  
            0x33 => {
                self.reg[rd] = self.reg[rs1].wrapping_add(self.reg[rs2]);
                Ok(())
            }
            0x13 => {
                let imm = ((instruction as i32) >> 20) as i64 as u64;
                self.reg[rd] = self.reg[rs1].wrapping_add(imm); //wrapping add safer than '+' if aritmetic overflow happens
                Ok(())
            }
            _=> {println!("Not supported command detected"); Err(())},
        }
        
    } 
}