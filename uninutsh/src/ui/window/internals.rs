use gl::types::GLuint;
use glutin::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::{Window, WindowBuilder},
    ContextBuilder, ContextWrapper, PossiblyCurrent,
};
use std::{ffi::c_void, time::{Duration, Instant}};

#[derive(Debug, Clone, Copy)]
pub enum CustomEvent {}

pub struct Data {
    texture: GLuint,
    framebuffer: GLuint,
    event_loop: Option<EventLoop<CustomEvent>>,
    proxy: Option<EventLoopProxy<CustomEvent>>,
    gl_window: ContextWrapper<PossiblyCurrent, Window>,
}

fn panic_gl(string: &str) {
    let error = unsafe { gl::GetError() };
    match error {
        gl::NO_ERROR => {}
        gl::INVALID_ENUM => {
            panic!("gl::INVALID_ENUM {}", string);
        }
        gl::INVALID_VALUE => {
            panic!("gl::INVALID_VALUE {}", string);
        }
        gl::INVALID_OPERATION => {
            panic!("gl::INVALID_OPERATION {}", string);
        }
        gl::INVALID_FRAMEBUFFER_OPERATION => {
            panic!("gl::INVALID_FRAMEBUFFER_OPERATION {}", string);
        }
        gl::OUT_OF_MEMORY => {
            panic!("gl::OUT_OF_MEMORY {}", string);
        }
        gl::STACK_UNDERFLOW => {
            panic!("gl::STACK_UNDERFLOW {}", string);
        }
        gl::STACK_OVERFLOW => {
            panic!("gl::STACK_OVERFLOW {}", string);
        }
        _ => {
            panic!("Unknown error {}", string);
        }
    }
}

impl Data {
    pub fn new(title: &str) -> Data {
        let el = EventLoop::<CustomEvent>::with_user_event();
        let proxy = Some(el.create_proxy());
        let monitor = el.primary_monitor().unwrap();
        let width = 1280; //
        let height = 720;
        let x = monitor.size().width / 2 - width / 2;
        let y = monitor.size().height / 2 - height / 2;
        let window_builder = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(PhysicalSize::new(width, height))
            .with_position(PhysicalPosition::new(x, y))
            .with_min_inner_size(PhysicalSize::new(512, 256));
        let context_builder = ContextBuilder::new()
            .build_windowed(window_builder, &el)
            .unwrap();

        // It is essential to make the context current before calling `gl::load_with`.
        let gl_window = unsafe { context_builder.make_current().unwrap() };
        let event_loop = Some(el);
        // Load the OpenGL function pointers
        gl::load_with(|symbol| gl_window.get_proc_address(symbol));
        let mut framebuffer = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut framebuffer);
            panic_gl("gl::GenFramebuffers");
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, framebuffer);
            panic_gl("gl::BindFramebuffer");
        }
        let mut texture = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            panic_gl("gl::GenTextures");

            gl::BindTexture(gl::TEXTURE_2D, texture);
            panic_gl("gl::BindTexture");
        }
        Data {
            texture,
            framebuffer,
            event_loop,
            proxy,
            gl_window,
        }
    }
    pub fn event_loop(mut self, mut window: super::Window) {
        let event_loop = self.event_loop.take().unwrap();
        let _proxy = self.proxy.take().unwrap();
        let sprite_width = window.graphics_width().expect("Can not find graphics object") as i32;
        let sprite_height = window.graphics_height().expect("Can not find graphics object") as i32;
        let size = self.gl_window.window().inner_size();
        window.update_rectangle(size.width as i32, size.height as i32);
        let mut event_handler = window.handler.take();
        let pixels = window.pixels().expect("can not take the pixels");
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                sprite_width,
                sprite_height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                pixels.as_ptr() as *const c_void,
            );
            panic_gl("gl::TexImage2D");

            gl::FramebufferTexture2D(
                gl::READ_FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                self.texture,
                0,
            );
            panic_gl("gl::FramebufferTexture2D");
        }
        window.return_pixels(Some(pixels));
        let mut last_update_instant = Instant::now();
        event_loop.run(move |event, _el_window_target, control_flow| {
            match event {
                Event::UserEvent(_event) => (),
                Event::LoopDestroyed => {
                    return;
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => match &mut event_handler {
                        Some(handler) => {
                            handler.handle_event(super::WindowEvent::Exit, &mut window);
                        }
                        None => {}
                    },
                    WindowEvent::Resized(size) => {
                        window.update_rectangle(size.width as i32, size.height as i32);
                    }
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    match &mut event_handler {
                        Some(handler) => {
                            handler.handle_event(super::WindowEvent::Draw, &mut window);
                        }
                        None => {}
                    }
                    let pixels = window.pixels().expect("can not take the pixels for updating");
                    unsafe {
                        gl::ClearColor(0., 0., 0., 1.);
                        panic_gl("gl::ClearColor");

                        gl::Clear(gl::COLOR_BUFFER_BIT);
                        panic_gl("gl::Clear");

                        gl::TexSubImage2D(
                            gl::TEXTURE_2D,
                            0,
                            0,
                            0,
                            sprite_width,
                            sprite_height,
                            gl::RGBA,
                            gl::UNSIGNED_BYTE,
                            pixels.as_ptr() as *const c_void,
                        );
                        panic_gl("gl::TexSubImage2D");
                        window.return_pixels(Some(pixels));

                        gl::BlitNamedFramebuffer(
                            self.framebuffer,
                            0,
                            0,
                            0,
                            sprite_width,
                            sprite_height,
                            window.rectangle.position.x,
                            window.rectangle.position.y,
                            window.rectangle.position.x + window.rectangle.size.x,
                            window.rectangle.position.y + window.rectangle.size.y,
                            gl::COLOR_BUFFER_BIT,
                            gl::NEAREST,
                        );
                        panic_gl("gl::BlitNamedFramebuffer");
                    }
                    self.gl_window.swap_buffers().unwrap();
                    //println!("redraw")
                }
                _ => (),
            }
            let delta = last_update_instant.elapsed();
            if delta >= Duration::from_millis(16) && !window.must_close {
                match &mut event_handler {
                    Some(handler) => {
                        handler.handle_event(super::WindowEvent::Update(delta), &mut window);
                    }
                    None => {}
                }

                last_update_instant = Instant::now();
            }
            if window.must_redraw {
                self.gl_window.window().request_redraw();
                window.must_redraw = false;
            }
            if window.must_close {
                *control_flow = ControlFlow::Exit;
            }
            match control_flow {
                ControlFlow::Exit => (),
                _ => {
                    *control_flow =
                        ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(1));
                }
            }
        });
    }
}
