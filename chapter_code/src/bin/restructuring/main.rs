pub mod app;
pub mod render;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

use crate::app::App;

fn main() {
    let event_loop = EventLoop::new();
    let mut app = App::start(&event_loop);

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        Event::WindowEvent {
            event: WindowEvent::Resized(_),
            ..
        } => {
            app.handle_window_resize();
        }
        Event::MainEventsCleared => {
            app.update();
        }
        _ => (),
    });
}
