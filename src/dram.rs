//von Neumann architecture - instructions & data in memory

pub const DRAM_SIZE: u64 = 1024 * 1024;    // 1MB ( TODO: xv6 is expecting 128MB like on QEMU)
pub const DRAM_BASE: u64 = 0x8000_0000;    // taken from QEMU

pub struct Dram{
    pub dra_memory: Vec<u8>, 
}

impl Dram{

    pub fn new(code: Vec<u8>) -> Self{

        let mut dram = vec![0;DRAM_SIZE as usize];
        dram[..code.len()].copy_from_slice(&code); // adding code in dram memory
        Self{ dra_memory: dram } 
        
    }

    pub fn load(&self, address: u64 , size: u64) -> Result<u64,()>{
        match size{
            8=> Ok(self.load8(address)),   // byte
            16=> Ok(self.load16(address)), // half-word 
            32=> Ok(self.load32(address)), // word
            64=> Ok(self.load64(address)), // double-word
            _=> Err(()), 
        }
    }

    pub fn store(&mut self, address: u64 , value: u64, size: u64) -> Result<(),()>{
        match size{
            8=> Ok(self.write8(address,value)),   // byte
            16=> Ok(self.write16(address,value)), // half-word 
            32=> Ok(self.write32(address,value)), // word
            64=> Ok(self.write64(address,value)), // double-word
            _=> Err(()), 
        }
    }

    pub fn load8(&self,address: u64) -> u64 {
        let index = (address - DRAM_BASE) as usize;
        return self.dra_memory[index] as u64; 
    }
    pub fn load16(&self,address: u64) -> u64 {
        let index = (address - DRAM_BASE) as usize;
        return (self.dra_memory[index] as u64) 
            | ((self.dra_memory[index+1] as u64)<<8);
    }
    pub fn load32(&self,address: u64) -> u64 {
        let index = (address - DRAM_BASE) as usize;         //little-endian
        return (self.dra_memory[index] as u64) 
            | ((self.dra_memory[index+1] as u64)<<8) 
            | ((self.dra_memory[index+2] as u64)<<16)
            | ((self.dra_memory[index+3] as u64)<<24);
    }
    pub fn load64(&self,address: u64) -> u64 {
        let index = (address - DRAM_BASE) as usize;
        return (self.dra_memory[index] as u64) 
            | ((self.dra_memory[index+1] as u64)<<8) 
            | ((self.dra_memory[index+2] as u64)<<16)
            | ((self.dra_memory[index+3] as u64)<<24)
            | ((self.dra_memory[index+4] as u64)<<32)
            | ((self.dra_memory[index+5] as u64)<<40)
            | ((self.dra_memory[index+6] as u64)<<48)
            | ((self.dra_memory[index+7] as u64)<<56) ;
    }
    
    pub fn write8(&mut self,address: u64 , value: u64){
        let index = (address - DRAM_BASE) as usize;
        self.dra_memory[index] = (value & 0xff) as u8;
    }
    pub fn write16(&mut self,address: u64 , value: u64){
        let index = (address - DRAM_BASE) as usize;
        self.dra_memory[index] = (value & 0xff) as u8;
        self.dra_memory[index+1] = ((value>>8) & 0xff) as u8;
    }
    pub fn write32(&mut self,address: u64 , value: u64){
        let index = (address - DRAM_BASE) as usize;
        self.dra_memory[index] = (value & 0xff) as u8;
        self.dra_memory[index+1] = ((value>>8) & 0xff) as u8;
        self.dra_memory[index+2] = ((value>>16) & 0xff) as u8;
        self.dra_memory[index+3] = ((value>>24) & 0xff) as u8;
    }
    pub fn write64(&mut self,address: u64 , value: u64){
        let index = (address - DRAM_BASE) as usize;
        self.dra_memory[index] = (value & 0xff) as u8;
        self.dra_memory[index+1] = ((value>>8) & 0xff) as u8;
        self.dra_memory[index+2] = ((value>>16) & 0xff) as u8;
        self.dra_memory[index+3] = ((value>>24) & 0xff) as u8;
        self.dra_memory[index+4] = ((value>>32) & 0xff) as u8;
        self.dra_memory[index+5] = ((value>>40) & 0xff) as u8;
        self.dra_memory[index+6] = ((value>>48) & 0xff) as u8;
        self.dra_memory[index+7] = ((value>>56) & 0xff) as u8;
    }
    
}    