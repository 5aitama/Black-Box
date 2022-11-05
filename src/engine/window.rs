use raw_window_handle::{HasRawWindowHandle, HasRawDisplayHandle};
use winit::{event::{WindowEvent, KeyboardInput, ElementState, VirtualKeyCode, Event}, event_loop::ControlFlow};


pub struct Window {
    event_loop: winit::event_loop::EventLoop<()>,
    window: winit::window::Window,
}

unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.window.raw_window_handle()
    }
}

unsafe impl HasRawDisplayHandle for Window {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        self.window.raw_display_handle()
    }
}

impl Window {
    /// Create a new [Window].
    pub fn new() -> Self {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::Window::new(&event_loop).unwrap();
        
        window.set_resizable(false);
        window.set_title("[Graphics Engine]");
        
        Self {
            event_loop,
            window,
        }
    }

    /// Get the window size in pixel.
    pub fn size(&self) -> (u32, u32) {
        let size = self.window.inner_size();
        (size.width, size.height)
    }

    pub fn run<C, R>(self, mut update_callback: C, mut resize_callback: R)
    where
        C: FnMut() + 'static,
        R: FnMut((u32, u32)) + 'static,
    {
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent { ref event, window_id } => {
                    if window_id != self.window.id() { return; }
                    
                    match event {
                        WindowEvent::CloseRequested | WindowEvent::KeyboardInput { 
                            input: KeyboardInput { 
                                state: ElementState::Pressed, 
                                virtual_keycode: Some(VirtualKeyCode::Escape), 
                                ..
                            }, 
                            .. 
                        } => *control_flow = ControlFlow::Exit,

                        WindowEvent::Resized(physical_size) => {
                            let w = physical_size.width;
                            let h = physical_size.height;

                            resize_callback((w, h));
                        },

                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            let w = new_inner_size.width;
                            let h = new_inner_size.height;

                            resize_callback((w, h));
                        },

                        _ => {},
                    }
                },

                Event::RedrawRequested(id) => {
                    if id != self.window.id() { return; }

                    update_callback();
                },

                Event::MainEventsCleared => {
                    self.window.request_redraw();
                },

                _ => {}
            }
        });
    }
    
}