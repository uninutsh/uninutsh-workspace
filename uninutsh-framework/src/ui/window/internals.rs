use glutin::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::{Window, WindowBuilder},
    ContextBuilder, ContextWrapper, PossiblyCurrent,
};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub enum CustomEvent {}

pub struct Data {
    event_loop: Option<EventLoop<CustomEvent>>,
    proxy: Option<EventLoopProxy<CustomEvent>>,
    gl_window: ContextWrapper<PossiblyCurrent, Window>,
}

impl Data {
    pub fn new(title: &str) -> Data {
        let el = EventLoop::<CustomEvent>::with_user_event();
        let proxy = Some(el.create_proxy());
        let monitor = el.primary_monitor().unwrap();
        let width = 1024; //
        let height = 512;
        let x = monitor.size().width / 2 - width / 2;
        let y = monitor.size().height / 2 - height / 2;
        let window_builder = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(PhysicalSize::new(width, height))
            .with_position(PhysicalPosition::new(x, y));
        let context_builder = ContextBuilder::new()
            .build_windowed(window_builder, &el)
            .unwrap();

        // It is essential to make the context current before calling `gl::load_with`.
        let gl_window = unsafe { context_builder.make_current().unwrap() };
        let event_loop = Some(el);
        // Load the OpenGL function pointers
        gl::load_with(|symbol| gl_window.get_proc_address(symbol));
        Data {
            event_loop,
            proxy,
            gl_window,
        }
    }
    pub fn event_loop(mut self, _window: super::Window) {
        let event_loop = self.event_loop.take().unwrap();
        let _proxy = self.proxy.take().unwrap();
        /*
        std::thread::spawn(move || {
            // Wake up the `event_loop` once every second and dispatch a custom event
            // from a different thread.
            loop {
                std::thread::sleep(std::time::Duration::from_millis(20));
                proxy.send_event(CustomEvent::Timer).ok();
            }
        });
        */
        let mut last_update_instant = Instant::now();
        event_loop.run(move |event, _el_window_target, control_flow| {
            //_el_window_target;
            //println!("event received");

            match event {
                Event::UserEvent(_event) => (),
                Event::LoopDestroyed => {
                    return;
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    unsafe {
                        gl::ClearColor(1., 1., 1., 1.);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                    }
                    self.gl_window.swap_buffers().unwrap();
                }
                _ => (),
            }
            if last_update_instant.elapsed() >= Duration::from_millis(20) {
                println!("update {}", last_update_instant.elapsed().as_millis());
                self.gl_window.window().request_redraw();
                last_update_instant = Instant::now();
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
