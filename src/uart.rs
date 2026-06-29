
pub const UART_BASE: u64 = 0x1000_0000;    //QEMU convention
pub const UART_SIZE: u64 = 0x100;

pub const UART_RHR: u64 = UART_BASE + 0;   // receive holding register - not used (output-only)
pub const UART_THR: u64 = UART_BASE + 0;   // transmit holding register - write a byte to print it
pub const UART_LSR: u64 = UART_BASE + 5;   // line status register - read returns "ready"
// same address: read -> RHR , write -> THR 
pub struct Uart {
    // output-only teleprinter, no internal state needed
}

impl Uart {
    pub fn new() -> Self {
        Self {}
    }

    pub fn load(&self, address: u64, size: u64) -> Result<u64, ()> {
        if size != 8 {
            return Err(()); // UART registers are only one byte wide
        }
        match address {
            UART_LSR => Ok(0x60), // bits 5 and 6 set = "ready to transmit"
            UART_RHR => Ok(0),    // no input, nothing received
            _ => Ok(0),           // other registers read as 0
        }
    }

    pub fn store(&mut self, address: u64, value: u64, size: u64) -> Result<(), ()> {
        if size != 8 {
            return Err(()); 
        }
        match address {
            UART_THR => {
                let byte = (value & 0xff) as u8;
                print!("{}", byte as char);
                Ok(())
            }
            _ => Ok(()), // writes to other registers ignored
        }
    }
}
