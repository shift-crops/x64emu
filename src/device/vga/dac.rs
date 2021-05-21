use packed_struct::prelude::*;

#[derive(Debug)]
pub(super) struct DAC {
    pub pdmr: u8,
    pub dsr:  State,
    pub prir: u8,
    pub pwir: u8,

    palette: [RGB; 0x100],
    progress: RGBSel,
}

impl Default for DAC {
    fn default() -> Self {
        Self{
            pdmr: 0,
            dsr: Default::default(),
            prir: 0,
            pwir: 0,
            palette: [Default::default(); 0x100],
            progress: RGBSel::Red,
        }
    }
}

impl DAC {
    pub fn set_write_idx(&mut self, v: u8) -> () {
        self.pwir = v;
        self.dsr.stat = 0;
        self.progress.reset();
    }

    pub fn set_read_idx(&mut self, v: u8) -> () {
        self.prir = v;
        self.dsr.stat = 3;
        self.progress.reset();
    }

    pub fn read_palette(&mut self) -> u8 {
        let rgb = self.palette[self.prir as usize];
        let c = rgb.0[self.progress as usize];

        self.progress.next();
        if let RGBSel::Red = self.progress {
            self.prir = self.prir.wrapping_add(1);
        }
        c.pack().unwrap()[0]
    }

    pub fn write_palette(&mut self, v: u8) -> () {
        let mut rgb = self.palette[self.pwir as usize];
        rgb.0[self.progress as usize] = Color::unpack(&[v]).unwrap();

        self.progress.next();
        if let RGBSel::Red = self.progress {
            self.pwir = self.pwir.wrapping_add(1);
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct RGB([Color; 3]);

impl Into<[u8; 3]> for &RGB {
    fn into(self) -> [u8; 3] {
        let mut rgb = [0u8; 3];
        for i in 0..3 {
            rgb[i] = self.0[i].v << 2;
        }
        rgb
    }
}

#[derive(Debug, Default, Clone, Copy, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct Color {
    #[packed_field(bits="0:6")] v: u8,
}

#[derive(Debug, Clone, Copy)] #[repr(usize)]
enum RGBSel { Red, Green, Blue }

impl RGBSel {
    fn next(&mut self) -> () {
        *self = match self {
            Self::Red   => Self::Green,
            Self::Green => Self::Blue,
            Self::Blue  => Self::Red,
        };
    }

    fn reset(&mut self) -> () {
        *self = Self::Red;
    }
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct State {
    #[packed_field(bits="0:1")] stat: u8,
}