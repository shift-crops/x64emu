extern crate mini_gl_fb;

use std::time;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use mini_gl_fb::{get_fancy, config};
use mini_gl_fb::core::BufferFormat;
use mini_gl_fb::glutin::event::*;
use mini_gl_fb::glutin::event_loop::*;
use mini_gl_fb::glutin::dpi::*;

pub struct ImageBuffer {
    pub buf: Vec<[u8; 3]>,
    pub size: (u32, u32),
}

pub enum InputEvent { Keyboard(u8), Mouse([u8; 3]) }

pub struct GUI {
    pub imgbuf: Arc<Mutex<ImageBuffer>>,
    km_tx: Sender<InputEvent>,
    grab: bool,
}

impl GUI {
    pub fn new() -> (Self, Receiver<InputEvent>) {
        let (km_tx, km_rx): (Sender<InputEvent>, Receiver<InputEvent>) = mpsc::channel();

        let ib = ImageBuffer {
            buf: vec![[0, 0, 0]; 1],
            size: (1, 1),
        };

        let gui = Self {
            imgbuf: Arc::new(Mutex::new(ib)),
            km_tx,
            grab: false,
        };
        (gui, km_rx)
    }

    pub fn persistent(mut self, width: u32, height: u32) -> () {
        let event_loop = EventLoop::new();
        let config = config! {
            window_title: "x64emu".to_string(),
            window_size: LogicalSize::from((width, height)),
            resizable: true,
            invert_y: false,
        };
        let mut fb = get_fancy(config, &event_loop);
        fb.change_buffer_format::<u8>(BufferFormat::RGB);

        let mut mb_state: [u8;2] = Default::default();
        let mut m_pos: (f64, f64) = Default::default();

        let mut resume = time::Instant::now();
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::WaitUntil(resume);

            match &event {
                Event::LoopDestroyed => return,
                Event::NewEvents(StartCause::ResumeTimeReached { .. }) =>  {
                    resume = time::Instant::now() + time::Duration::from_millis(100);
                    let imgbuf = self.imgbuf.lock().unwrap();
                    fb.resize_buffer(imgbuf.size.0, imgbuf.size.1);
                    fb.update_buffer(&imgbuf.buf);
                },
                Event::WindowEvent { event, .. } => match &event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(ps) => {
                        fb.resize_viewport(ps.width, ps.height);
                        fb.redraw();
                    },
                    WindowEvent::MouseInput{ .. } if !self.grab => {
                        let window = fb.internal.context.window();
                        window.set_cursor_visible(false);
                        window.set_cursor_grab(true).unwrap();
                        window.set_title("x64emu (press right control to release mouse)");
                        self.grab = true;
                    },
                    _ => {}
                },
                Event::DeviceEvent { event, .. } if self.grab => {
                    match &event {
                        DeviceEvent::Key(input) => {
                            if let Some(VirtualKeyCode::RControl) = input.virtual_keycode {
                                let window = fb.internal.context.window();
                                window.set_cursor_grab(false).unwrap();
                                window.set_cursor_visible(true);
                                window.set_title("x64emu");
                                self.grab = false;
                            }
                            let scancode = input.scancode as u8 + if let ElementState::Pressed = input.state { 0 } else { 0x80 };
                            self.km_tx.send(InputEvent::Keyboard(scancode)).unwrap();
                            //println!("{:x?}", input);
                        },
                        DeviceEvent::MouseMotion { delta } => {
                            let delta = if delta.0.fract() != 0.0 || delta.1.fract() != 0.0 {
                                let d = (((delta.0 - m_pos.0)/20.0) as i8, ((m_pos.1 - delta.1)/20.0) as i8);
                                m_pos = *delta;
                                d
                            } else {
                                (delta.0 as i8, -delta.1 as i8)
                            };
                            //println!("{:x?}", delta);

                            let sx = if delta.0 < 0 { 1 } else { 0 };
                            let sy = if delta.1 < 0 { 1 } else { 0 };
                            self.km_tx.send(InputEvent::Mouse([(sy<<5) + (sx<<4) + (1<<3) + (mb_state[1]<<1) + mb_state[0], delta.0 as u8, delta.1 as u8])).unwrap();
                        },
                        DeviceEvent::Button { button, state } => {
                            mb_state[(button/2) as usize] = if let ElementState::Pressed = state { 1 } else { 0 };
                            self.km_tx.send(InputEvent::Mouse([(1<<3) + (mb_state[1]<<1) + mb_state[0], 0, 0])).unwrap();
                        },
                        _ => {}
                    }
                },
                _ => {},
            }
        });
    }
}