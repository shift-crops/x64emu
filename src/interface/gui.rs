extern crate mini_gl_fb;

use std::time;
use std::sync::{Arc, Mutex};
use mini_gl_fb::{get_fancy, config};
use mini_gl_fb::core::BufferFormat;
use mini_gl_fb::glutin::event::*;
use mini_gl_fb::glutin::event_loop::*;
use mini_gl_fb::glutin::dpi::*;


pub struct GUI {
    pub buffer: Arc<Mutex<(Vec<[u8; 3]>, (u32, u32))>>,
    size: (u32, u32),
    grab: bool,
}

impl GUI {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            buffer: Arc::new(Mutex::new((vec![[0, 0, 0]; (width*height) as usize], (width, height)))),
            size: (width, height),
            grab: false,
        }
    }

    pub fn persistent(mut self) -> () {
        let event_loop = EventLoop::new();
        let config = config! {
            window_title: "x64emu".to_string(),
            //window_size: LogicalSize::from(self.size),
            window_size: LogicalSize::from((self.size.0*4, self.size.1*4)),
            resizable: true,
            invert_y: false,
        };
        let mut fb = get_fancy(config, &event_loop);

        fb.change_buffer_format::<u8>(BufferFormat::RGB);

        let mut resume = time::Instant::now();
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::WaitUntil(resume);

            match &event {
                Event::LoopDestroyed => return,
                Event::NewEvents(StartCause::ResumeTimeReached { .. }) =>  {
                    resume = time::Instant::now() + time::Duration::from_millis(100);
                    let buffer = self.buffer.lock().unwrap();
                    fb.resize_buffer(buffer.1.0, buffer.1.1);
                    fb.update_buffer(&buffer.0);
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
                            println!("{:x?}", input);
                        },
                        DeviceEvent::MouseMotion { delta } => {
                            println!("{:x?}", delta);
                        },
                        _ => {}
                    }
                },
                _ => {},
            }
        });
    }
}