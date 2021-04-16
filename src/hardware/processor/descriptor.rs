#[derive(Debug, Default)]
pub struct DTRegisters {
    pub gdtr: DescTbl,
    pub idtr: DescTbl,
    pub ldtr: DescTblSel,
    pub tr:   DescTblSel,
}

#[derive(Debug)]
pub struct DescTbl {
    pub base:  u64, 
    pub limit: u32, 
}

impl DescTbl {
    pub fn get(&self) -> (u64, u32) {
        (self.base, self.limit)
    }
}

impl Default for DescTbl {
    fn default() -> Self {
        Self{ base: 0, limit: 0xffff, }
    }
}

#[derive(Debug, Default)]
pub struct DescTblSel {
    pub selector:   u16, 
    //pub attr:       u16, 
    pub cache:      DescTbl, 
}