use crate::bus::Bus;
use crate::dram::DRAM_BASE;

pub struct CPU{             //pub allows other modules to use cpu structure and methods
    pub reg : [u64;32],
    pub pc: u64,
    pub system_bus: Bus     // talking to memory (dram) throught system bus
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

        //always on that place so we can extract before match
        let opcode = instruction & 0x7f;
        let rd = ((instruction >> 7 ) & 0x1f as u32) as usize;
        let rs1 = ((instruction >> 15 ) & 0x1f as u32) as usize;       
        let rs2 = ((instruction >> 20 ) & 0x1f as u32) as usize;
        let func3 = ((instruction >> 12) & 0x7 as u32) as usize;
        let func7 = ((instruction >> 25) & 0x7f as u32) as usize;


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

            0x13 => { // REGISTER - IMMEDIATE ALU ( I type )
                let imm = ((instruction as i32) >> 20) as i64 as u64;
                let shamt = (instruction >> 20) & 0x3f;
                let srl_bit = (instruction >> 30) & 1; // i could also use func7 
                match func3{
                    0x0 =>{ //addi
                        self.reg[rd] = self.reg[rs1].wrapping_add(imm);                     //wrapping add safer than '+' if aritmetic overflow happens
                        Ok(())
                    },
                    0x1 =>{ //slli
                        self.reg[rd] = self.reg[rs1] << shamt;                                      // rd = rs1 << con[0:5]
                        Ok(())          
                    },
                    0x2 =>{ //slti
                        self.reg[rd] = if (self.reg[rs1] as i64) < (imm as i64) { 1 } else { 0 };   //rd = (rs1 < sext(con)) ? 1 : 0  
                        Ok(())                           
                    },
                    0x3 =>{ //sltiu
                        self.reg[rd] = if self.reg[rs1] < imm { 1 } else { 0 };                     //rd = (rs1 < sext(con)) ? 1 : 0 
                        Ok(())                                                              //(imm is sign extended , but < is unsigned operation here)
                    },
                    0x4 =>{ //xori
                        self.reg[rd] = self.reg[rs1] ^ imm;                                         // rd = rs1 ^ sext(con)
                        Ok(())
                    },
                    0x5 =>{ //srai 
                        if srl_bit==1 {self.reg[rd] = ((self.reg[rs1] as i64) >> shamt) as u64}     //rd = rs1 >> con[0:5]
                            //srli
                        else {self.reg[rd] = self.reg[rs1] >> shamt};                               //rd = rs1 >> con[0:5]
                        Ok(())
                    },
                    0x6 =>{ //ori
                        self.reg[rd] = self.reg[rs1] | imm;                                         //rd = rs1 | sext(con)
                        Ok(())
                    },
                    0x7 =>{ //andi
                        self.reg[rd] = self.reg[rs1] & imm;                                         //rd = rs1 & sext(con)
                        Ok(())
                    },
                    _ =>{
                        println!("Invalid register-immediate AL operation");
                        Err(())
                    },

                }
                
            }
            0x33 => { //REGISTER - REGISTER ALU ( R type )
                let shamt = (self.reg[rs2] & 0x3f) as u64; // shamt = rs2[0:5]
                let srl_bit = (instruction >> 30) & 1; // telling is it arithmetic or logic, i could also use func7 
                match func3{
                    0x0 =>{ 
                        match func7{
                            0x00=>{ // add 
                                self.reg[rd] = self.reg[rs1].wrapping_add(self.reg[rs2]);   //rd = rs1 + rs2
                                Ok(())
                            },
                            0x20 =>{ //sub
                                self.reg[rd] = self.reg[rs1].wrapping_sub(self.reg[rs2]);   //rd = rs1 - rs2
                                Ok(())
                            },
                            _ => {
                                println!("Invalid register-register add/sub operation");
                                Err(())
                            },
                        }
                    },
                    0x1 =>{ //sll
                        self.reg[rd] = self.reg[rs1] << shamt;     //rd = rs1 << rs2[0:5]
                        Ok(())      
                    },
                    0x2 =>{ //slt
                        self.reg[rd] = if ( self.reg[rs1] as i64 ) < ( self.reg[rs2] as i64) {1} else {0};  //rd = (rs1 < rs2) ? 1 : 0
                        Ok(())
                    },
                    0x3 =>{ //sltu
                        self.reg[rd] = if self.reg[rs1] < self.reg[rs2] {1} else {0};   //rd = (rs1 < rs2) ? 1 : 0
                        Ok(())                                                            
                    },
                    0x4 =>{ 
                        match func7 {
                            0x00 => { // xor
                                self.reg[rd] = self.reg[rs1] ^ self.reg[rs2];                       //rd = rs1 ^ rs2
                                Ok(())
                            },
                            0x01 => { // div 
                                self.reg[rd] = if self.reg[rs2] == 0 { 0xFFFFFFFF }                 //rd = rs1 / rs2
                                else{
                                    (self.reg[rs1] as i64).wrapping_div(self.reg[rs2] as i64) as u64
                                };
                                Ok(())
                            },
                            _ => { println!("Invalid register-register xor/div operation"); Err(()) },
                        }
                    },
                    0x5 =>{ 
                        //sra
                        if srl_bit==1 {self.reg[rd] = ((self.reg[rs1] as i64) >> shamt) as u64}     //rd = rs1 >> rs2[0:5]
                        //srl
                        else {self.reg[rd] = self.reg[rs1] >> shamt};                               // rd = rs1 >> rs2[0:5]    
                        Ok(())                                
                    },
                    0x6 =>{ 
                        match func7 {
                            0x00 => { // or
                                self.reg[rd] = self.reg[rs1] | self.reg[rs2];                       //rd = rs1 | rs2
                                Ok(())
                            },
                            0x01 => { // rem 
                                self.reg[rd] = if self.reg[rs2] == 0 { self.reg[rs1] }
                                else {
                                    (self.reg[rs1] as i64).wrapping_rem(self.reg[rs2] as i64) as u64 //rd = rs1 % rs2
                                };
                                Ok(())
                            },
                             _ => { println!("Invalid register-register or/rem operation"); Err(()) },
                        }
                    },
                    0x7 =>{ //and
                        self.reg[rd] = self.reg[rs1] & self.reg[rs2];   //rd = rs1 & rs2
                        Ok(())
                    },
                    _ =>{
                        println!("Invalid register-register AL operation");
                        Err(())
                    },

                }
            }
            0x1B =>{ // W REGISTER - IMMEDIATE ALU ( I type ) 32-bit
                let imm = ((instruction as i32) >> 20) as i64 as u64;
                let shamt = (instruction >> 20) & 0x1f;
                let srl_bit = (instruction >> 30) & 1; 
                match func3{
                    0x0 =>{ //addiw
                        self.reg[rd] = self.reg[rs1].wrapping_add(imm) as i32 as i64 as u64;       //rd = sext((rs1 + sext(con))[0:31])              
                        Ok(())
                    },
                    0x1 =>{ //slliw
                        self.reg[rd] = ((self.reg[rs1] as i32) << shamt) as i64 as u64;         //rd = sext((rs1 << con[0:4])[0:31])                              
                        Ok(())          
                    },
                    0x5 =>{ //sraiw 
                        if srl_bit==1 {self.reg[rd] = ((self.reg[rs1] as i32) >> shamt) as i64 as u64}     //sext(rd[0:31] = rs1 >> con[0:4])
                            //srliw
                        else { self.reg[rd] = ((self.reg[rs1] as u32) >> shamt) as i32 as i64 as u64; }                //rd = sext(rs1[0:31] >> con[0:4])               
                        Ok(())
                    },
                    _ =>{
                        println!("Invalid W register-immediate AL 32-bit operation");
                        Err(())
                    },
                }
            }
            0x3B => { // W REGISTER - REGISTER ALU ( R type ) 32-bit
                let shamt = (self.reg[rs2] & 0x1f) as u32; //rs2[0:4]
                let srl_bit = (instruction >> 30) & 1;
                match func3{
                    0x0 =>{
                        match func7{
                            0x00=>{ // addw 
                                self.reg[rd] =  self.reg[rs1].wrapping_add(self.reg[rs2]) as i32 as i64 as u64; //rd = sext((rs1 + rs2)[0:31])
                                Ok(())
                            },
                            0x20 =>{ //subw
                                self.reg[rd] = self.reg[rs1].wrapping_sub(self.reg[rs2]) as i32 as i64 as u64; //rd = sext((rs1 - rs2)[0:31])
                                Ok(())
                            },
                            _ =>{
                                println!("Invalid W register-register add/sub 32-bit operation");
                                Err(())
                            },
                        }
                    },
                    0x1 =>{ //sllw
                        self.reg[rd] = ((self.reg[rs1] as i32) << shamt) as i64 as u64;      //rd = sext((rs1 << rs2[0:4])[0:31])
                        Ok(()) 
                    },
                    0x5 =>{ 
                        //sraw
                        if srl_bit==1 { self.reg[rd] = ((self.reg[rs1] as i32) >> shamt) as i64 as u64; }     //rd = rs1 >> rs2[0:5]
                        //srlw
                        else { self.reg[rd] = ((self.reg[rs1] as u32) >> shamt) as i32 as i64 as u64; }                  // rd = rs1 >> rs2[0:5]    
                        Ok(())
                    },
                    _ =>{
                        println!("Invalid W register-register 32-bit operation");
                        Err(())
                    },
                }
            },
            0x37 =>{ //lui ( U type )
                self.reg[rd] = (instruction & 0xfffff000) as i32 as i64 as u64; //rd = sext(con[31:12 << 12)
                Ok(())
            },
            0x17 =>{ //auipc ( U type )
                let imm = (instruction & 0xfffff000) as i32 as i64 as u64;
                self.reg[rd] = self.pc.wrapping_add(imm).wrapping_sub(4); //rd = pc + sext(con[31:12] << 12)
                //-4 because in main I incremented pc to next instruction before execution of current one
                Ok(())
            },
            0x6f =>{ //jal ( J type )
                self.reg[rd] = self.pc; // already on pc+4
                let imm = (((instruction & 0x80000000) as i32 as i64 >> 11) as u64) //extracting immediate from J type instruction
                        | (instruction & 0xff000) as u64                       
                        | ((instruction >> 9) & 0x800) as u64 
                        | ((instruction >> 20) & 0x7fe) as u64; 

                self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);    //rd = pc + 4; pc += sext(con)
                Ok(())

            },
            0x67 =>{ //jalr ( I type )
                let old_pc = self.pc;
                let imm = ((((instruction & 0xfff00000) as i32) as i64) >> 20) as u64;
                self.pc = (self.reg[rs1].wrapping_add(imm)) & !1;      //rd = pc + 4; pc = (rs1 + sext(offset)) & ~1 
                // & !1 clears the lowest bit to force an even (aligned) address
                self.reg[rd] = old_pc; //link
                Ok(())
            },
            
            0x63 =>{  //BRANCH INSTRUCTIONS ( B Type )
                let imm = (((instruction & 0x80000000) as i32 as i64 >> 19) as u64)
                        | (((instruction & 0x80) << 4) as u64)
                        | (((instruction >> 20) & 0x7e0) as u64)
                        | (((instruction >> 7) & 0x1e) as u64); 

                match func3{
                    0x0 =>{ //beq
                        if self.reg[rs1] == self.reg[rs2]{ 
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);    //if (rs1 == rs2) pc += sext(con)
                        }
                        Ok(())
                    },
                    0x1 =>{ //bne
                        if self.reg[rs1] != self.reg[rs2]{ 
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);    //if (rs1 != rs2) pc += sext(con)
                        }
                        Ok(())
                    },
                    0x4 =>{ //blt 
                        if (self.reg[rs1] as i64) < (self.reg[rs2] as i64) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);    //if (rs1 < rs2) pc += sext(con)
                        }
                        Ok(())
                    },
                    0x5 =>{ //bge
                        if self.reg[rs1] as i64 >= self.reg[rs2] as i64 { 
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);    //if (rs1 < rs2) pc += sext(con)
                        }
                        Ok(())
                    },
                    0x6 =>{ //bltu
                        if self.reg[rs1] < self.reg[rs2] { 
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);    //if (rs1 < rs2) pc += sext(con)
                        }
                        Ok(())
                    },
                    0x7 =>{ //bgeu
                        if self.reg[rs1] >= self.reg[rs2] { 
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);    //if (rs1 < rs2) pc += sext(con)
                        }
                        Ok(())
                    },
                    _ =>{
                        println!("Invalid branch instruction");
                        Err(())
                    },
                }

            },

        
            _=> {println!("Not supported command detected"); Err(())},
        }
        
    } 
}