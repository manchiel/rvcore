use crate::dram::*;
use crate::uart::*;

pub struct Bus{
    dram: Dram,
    uart: Uart
    //TODO: other peripheral devices 
}

impl Bus{
    
    pub fn new(code: Vec<u8>) -> Self{
        Self{ 
              dram: Dram::new(code),
              uart: Uart::new()
            }
    }

    pub fn load(&self, address: u64,size: u64) -> Result<u64,()>{

        if UART_BASE <= address && address < UART_BASE + UART_SIZE {
            return self.uart.load(address, size);
        }
        else if address >= DRAM_BASE{
            return self.dram.load(address,size);
        }           
        Err(())
    }
    pub fn store(&mut self, address: u64, value: u64, size: u64) -> Result<(),()>{

        if UART_BASE <= address && address < UART_BASE + UART_SIZE {
            return self.uart.store(address,value, size);
        }
        else if address > DRAM_BASE{
            return self.dram.store(address,value,size);
        }         
        Err(())
    }
}