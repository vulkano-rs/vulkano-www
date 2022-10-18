pub mod app;
pub mod render;

use std::time::Instant;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

use crate::app::App;

fn main() {
    let event_loop = EventLoop::new();
    let mut app = App::start(&event_loop);

    let mut previous_frame_time = Instant::now();
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
        Event::WindowEvent {
            event: WindowEvent::KeyboardInput { input, .. },
            ..
        } => {
            if let Some(key_code) = input.virtual_keycode {
                app.handle_keyboard_input(key_code, input.state)
            }
        }
        Event::MainEventsCleared => {
            let this_frame_time = Instant::now();
            let duration_from_last_frame = this_frame_time - previous_frame_time;

            app.update(&duration_from_last_frame);

            previous_frame_time = this_frame_time;
        }
        _ => (),
    });
}
