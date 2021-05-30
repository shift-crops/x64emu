use std::{thread, time};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, Condvar};
use packed_struct::prelude::*;

use crate::interface::gui::InputEvent;

#[derive(Default)]
pub(super) struct KeyMouse {
    kcsr: Status,
    out_buf: u8,
    ccb: ControlCommand,
    ctrl_ram: [u8; 0x20],

    cmd: Option<Command>,
    keyboard_enable: bool,
    mouse_enable: bool,
}

enum Command { Encoder(u8), Controller(u8) }

#[derive(Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct Status {
    #[packed_field(bits="0")] ob_full: bool,
    #[packed_field(bits="1")] ib_full: bool,
    #[packed_field(bits="2")] f0:      u8,
    #[packed_field(bits="3")] f1:      u8,
    #[packed_field(bits="4")] st4:     u8,
    #[packed_field(bits="5")] st5:     u8,
    #[packed_field(bits="6")] st6:     u8,
    #[packed_field(bits="7")] st7:     u8,
}

#[derive(Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct ControlCommand {
    #[packed_field(bits="0")] ki_ena:  bool,
    #[packed_field(bits="1")] mi_ena:  bool,
    #[packed_field(bits="2")] sysf:    u8,
    #[packed_field(bits="3")] ig_klck: bool,
    #[packed_field(bits="4")] k_dis:   bool,
    #[packed_field(bits="5")] m_dis:   bool,
    #[packed_field(bits="6")] xlate:   bool,
}

impl KeyMouse {
    pub fn new(irq_key: super::IReq, irq_mouse: super::IReq, ch: Receiver<InputEvent>) -> KMAccess {
        let kmc: Arc<(Mutex<Self>, Condvar)> = Arc::new((Mutex::new(Default::default()), Condvar::new()));

        let _kmc = kmc.clone();
        thread::spawn(move || {
            loop {
                let ev = if let Ok(ev) = ch.recv() { ev } else { break; };
                let mut km = _kmc.0.lock().unwrap();

                match ev {
                    InputEvent::Keyboard(code) => {
                        if !km.keyboard_enable || km.ccb.k_dis { continue; }

                        if km.kcsr.ob_full {
                            km = _kmc.1.wait_timeout(km, time::Duration::from_millis(200)).unwrap().0;
                        }
                        km.write_outbuf(code, false);

                        if km.ccb.ki_ena {
                            irq_key.send_irq();
                        }
                    },
                    InputEvent::Mouse(codes) => {
                        if !km.mouse_enable || km.ccb.m_dis { continue; }

                        for c in codes.iter() {
                            if km.kcsr.ob_full {
                                km = _kmc.1.wait_timeout(km, time::Duration::from_millis(200)).unwrap().0;
                            }
                            km.write_outbuf(*c, true);

                            if km.ccb.mi_ena {
                                irq_mouse.send_irq();
                            }
                        }
                    },
                }
            }
        });

        KMAccess(kmc)
    }

    fn command_encoder(&mut self, cmd: u8, val: Option<u8>) -> () {
        match (cmd, val) {
            (0xed, None)|(0xf0, None)|(0xf3, None) => self.cmd = Some(Command::Encoder(cmd)),
            (0xed, Some(_)) => {},
            (0xee, _) => self.write_outbuf(0xee, false),
            (0xf0, Some(v)) => {
                if v == 0 {
                    self.write_outbuf(0x41, false)
                }
            },
            (0xf3, Some(_)) => {},
            (0xf4, _) => self.keyboard_enable = true,
            (0xf5, _) => self.keyboard_enable = false,
            _ => {},
        }
    }

    fn command_controller(&mut self, cmd: u8, val: Option<u8>) -> () {
        match (cmd, val) {
            (0x40..=0x7f, None)|(0xd1..=0xd4, None) => self.cmd = Some(Command::Controller(cmd)),
            (0x00..=0x3f, _) => {
                let data = self.read_ctrl_ram((cmd%0x20) as usize);
                self.write_outbuf(data, false);
            },
            (0x40..=0x7f, Some(v)) => self.write_ctrl_ram((cmd%0x20) as usize, v),
            (0xa7, _)              => self.ccb.m_dis = true,
            (0xa8, _)              => self.ccb.m_dis = false,
            (0xad, _)              => self.ccb.k_dis = true,
            (0xae, _)              => self.ccb.k_dis = false,
            (0xd1, Some(_))        => {},
            (0xd2, Some(_))        => {},
            (0xd3, Some(_))        => {},
            (0xd4, Some(v))        => self.command_mouse(v),
            _ => {},
        }
    }

    fn command_mouse(&mut self, cmd: u8) -> () {
        match cmd {
            0xf4 => self.mouse_enable = true,
            0xf5 => self.mouse_enable = false,
            _ => {},
        }
    }

    fn read_outbuf(&mut self) -> u8 {
        self.kcsr.ob_full = false;
        self.out_buf
    }

    fn write_outbuf(&mut self, v: u8, mouse: bool) -> () {
        self.kcsr.ob_full = true;
        self.kcsr.st5 = mouse as u8;
        self.out_buf = v;
    }

    fn read_ctrl_ram(&mut self, ofs: usize) -> u8 {
        match ofs {
            0 => self.ccb.pack().unwrap()[0],
            i @ 1..=0x1f => self.ctrl_ram[i],
            _ => 0,
        }
    }

    fn write_ctrl_ram(&mut self, ofs: usize, v: u8) -> () {
        match ofs {
            0 => self.ccb = ControlCommand::unpack(&[v]).unwrap(),
            i @ 1..=0x1f => self.ctrl_ram[i] = v,
            _ => {},
        }
    }
}

pub struct KMAccess(Arc<(Mutex<KeyMouse>, Condvar)>);

impl super::PortIO for KMAccess {
    fn in8(&self, addr: u16) -> u8 {
        let mut km = self.0.0.lock().unwrap();
        match addr {
            0x60 => {
                let v = km.read_outbuf();
                self.0.1.notify_one();
                v
            },
            0x64 => km.kcsr.pack().unwrap()[0],
            _    => 0,
        }
    }

    fn out8(&mut self, addr: u16, val: u8) -> () {
        let mut km = self.0.0.lock().unwrap();
        match addr {
            0x60 => {
                km.kcsr.f1 = 0;
                match km.cmd.take() {
                    Some(Command::Encoder(c))    => km.command_encoder(c, Some(val)),
                    Some(Command::Controller(c)) => km.command_controller(c, Some(val)),
                    None                         => km.command_encoder(val, None),
                }
            },
            0x64 => {
                km.kcsr.f1 = 1;
                km.command_controller(val, None);
            },
            _    => {},
        }
    }
}