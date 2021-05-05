use crate::emulator::*;

impl super::Access {
    pub fn in_8(&self, addr: u16) -> Result<u8, EmuException> {
        let mut data: [u8; 1] = [0; 1];
        self.dev.in_portio(addr, &mut data);
        Ok(data[0])
    }

    pub fn out_8(&mut self, addr: u16, v: u8) -> Result<(), EmuException> {
        self.dev.out_portio(addr, &[v]);
        Ok(())
    }

    pub fn in_16(&self, addr: u16) -> Result<u16, EmuException> {
        let mut data: [u8; 2] = [0; 2];
        self.dev.in_portio(addr, &mut data);
        Ok(u16::from_le_bytes(data))
    }

    pub fn out_16(&mut self, addr: u16, v: u16) -> Result<(), EmuException> {
        self.dev.out_portio(addr, &v.to_le_bytes());
        Ok(())
    }

    pub fn in_32(&self, addr: u16) -> Result<u32, EmuException> {
        let mut data: [u8; 4] = [0; 4];
        self.dev.in_portio(addr, &mut data);
        Ok(u32::from_le_bytes(data))
    }

    pub fn out_32(&mut self, addr: u16, v: u32) -> Result<(), EmuException> {
        self.dev.out_portio(addr, &v.to_le_bytes());
        Ok(())
    }
}