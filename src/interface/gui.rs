extern crate mini_gl_fb;

use std::{thread, time};
use std::sync::{Arc, RwLock};
use mini_gl_fb::core::BufferFormat;
use mini_gl_fb::glutin::event::*;
use mini_gl_fb::glutin::event_loop::*;

pub struct GUI {
    size: (usize, usize),
    grab: bool,
}

impl GUI {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            size: (width, height),
            grab: false,
        }
    }

    pub fn test(mut self) -> () {
        let (event_loop, mut fb) = mini_gl_fb::gotta_go_fast("x64emu", self.size.0 as f64, self.size.1 as f64);

        fb.change_buffer_format::<u8>(BufferFormat::RGB);
        fb.set_resizable(true);

        let buffer :Arc<RwLock<Vec<[u8; 3]>>> = Arc::new(RwLock::new(vec![[0, 0, 0]; 320 * 200]));
        let buf = Arc::clone(&buffer);
        std::thread::spawn(move || {
            let mut c :u8 = 0;
            loop {
                thread::sleep(time::Duration::from_millis(1000));
                c = c.wrapping_add(20);
                for i in 0..320*200 {
                    buf.write().unwrap()[i] = [0, 0, c];
                }
            }
        });

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::WaitUntil(std::time::Instant::now() + time::Duration::from_millis(50));

            match &event {
                Event::LoopDestroyed => return,
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
                Event::DeviceEvent { event, .. } if self.grab => match &event {
                    DeviceEvent::Key(input) => {
                        if let Some(VirtualKeyCode::RControl) = input.virtual_keycode {
                            let window = fb.internal.context.window();
                            window.set_cursor_grab(false).unwrap();
                            window.set_cursor_visible(true);
                            window.set_title("x64emu");
                            self.grab = false;
                        }
                        println!("{:x?}", input);
                    },
                    DeviceEvent::MouseMotion { delta } => {
                        println!("{:x?}", delta);
                    },
                    _ => {}
                },
                Event::NewEvents(cause) => match cause {
                    StartCause::ResumeTimeReached { .. } => fb.update_buffer(&buffer.read().unwrap()),
                    _ => {},
                },
                _ => {},
            }
        });
    }
}