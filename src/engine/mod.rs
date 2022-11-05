pub mod window;
pub mod renderer;
pub mod renderers;

use std::sync::{Arc, Mutex};

use crate::engine::{ renderer::RendererTrait, window::Window };

pub struct Engine<R: RendererTrait + 'static> {
    window: Window,
    renderer: Arc<Mutex<R>>,
    update_callback: Option<Box<dyn FnMut() + 'static>>,
    render_callback: Option<Box<dyn FnMut(&mut R) + 'static>>,
}

impl<R: RendererTrait + 'static> Engine<R> {
    pub fn new() -> Self {
        let window = Window::new();
        let renderer = R::new(&window, window.size());

        Self {
            window,
            renderer: Arc::new(Mutex::new(renderer)),
            update_callback: None,
            render_callback: None,
        }
    }

    pub fn with_renderer_mut<T, F: FnMut(&mut R) -> T>(&mut self, mut f: F) -> T {
        let mut renderer = self.renderer.lock().unwrap();

        f(&mut renderer)
    }

    pub fn with_renderer_ref<T, F: Fn(&R) -> T>(&self, f: F) -> T {
        let renderer = self.renderer.lock().unwrap();

        f(&renderer)
    }

    pub fn set_on_update_callback<C: FnMut() + 'static>(&mut self, callback: C) {
        self.update_callback = Some(Box::new(callback));
    }

    pub fn set_on_render_callback<C: FnMut(&mut R) + 'static>(&mut self, callback: C) {
        self.render_callback = Some(Box::new(callback));
    }

    pub fn run(self) {
        let renderer = self.renderer.clone();
        let mut on_update_callback = self.update_callback.unwrap_or(Box::new(|| {}));
        let mut on_render_callback = self.render_callback.unwrap_or(Box::new(|_| {}));

        let update_callback = move || {
            let mut renderer = renderer.lock().unwrap();

            on_update_callback.as_mut()();
            renderer.render_begin();
            
            renderer.render();
            on_render_callback.as_mut()(&mut renderer);

            renderer.render_end();
        };
        
        let renderer = self.renderer.clone();
        let resize_callback = move |new_size: (u32, u32)| {
            renderer.lock().unwrap().resize(new_size);

            let (width, height) = new_size;
            println!("The new window size is {}x{}", width, height);
        };

        self.window.run(update_callback, resize_callback);
    }
}