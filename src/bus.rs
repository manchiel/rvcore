use crate::dram::*;

pub struct Bus{
    dram: Dram
    //TODO: other peripheral devices 
}

impl Bus{
    
    pub fn new(code: Vec<u8>) -> Self{
        Self{ dram: Dram::new(code) }
    }

    pub fn load(&self, address: u64,size: u64) -> Result<u64,()>{
        if address >= DRAM_BASE{
            return self.dram.load(address,size);
        }           
        Err(())
    }
    pub fn store(&mut self, address: u64, value: u64, size: u64) -> Result<(),()>{
        if address > DRAM_BASE{
            return self.dram.store(address,value,size);
        }         
        Err(())
    }
}