use winit::event_loop::EventLoop;

use crate::render::RenderLoop;

pub struct App {
    render_loop: RenderLoop,
}

impl App {
    pub fn start(event_loop: &EventLoop<()>) -> Self {
        Self {
            render_loop: RenderLoop::new(event_loop),
        }
    }

    pub fn update(&mut self) {
        self.render_loop.update();
    }

    pub fn handle_window_resize(&mut self) {
        self.render_loop.handle_window_resize()
    }
}
