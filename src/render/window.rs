use crate::input::InputEvent;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState, DeviceEvent},
    window::WindowBuilder,
};

pub struct Window {
    pub window: winit::window::Window,
    event_loop: EventLoop<()>,
}


impl Window {
    pub fn new(title: &str) -> Window {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .build(&event_loop)
            .expect("Failed to initialize window");

        Window {
            window,
            event_loop,
        }
    }
}

// todo: framerate limit: https://github.com/gfx-rs/wgpu/blob/master/wgpu/examples/framework.rs
pub fn run_event_loop(window: Window, mut game_frame: impl FnMut(&mut Vec<InputEvent>, Option<(u32, u32)>) -> bool + 'static) {
    let mut event_queue = Vec::<InputEvent>::new();
    let mut resize: Option<(u32, u32)> = None;
    window.event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {event: window_event, window_id: _} => {
                match window_event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    },
                    WindowEvent::Resized(size) => {
                        resize = Some((size.width, size.height));
                    },
                    WindowEvent::ScaleFactorChanged {new_inner_size, ..} => {
                        resize = Some((new_inner_size.width, new_inner_size.height));
                    },
                    WindowEvent::KeyboardInput {input, device_id: _, is_synthetic: _} => {
                        event_queue.push(InputEvent::Key {
                            code: input.scancode,
                            key: input.virtual_keycode,
                            pressed: input.state == ElementState::Pressed
                        });
                    },
                    WindowEvent::MouseInput {state, button, device_id: _, ..} => {
                        event_queue.push(InputEvent::Click {
                            button,
                            pressed: state == ElementState::Pressed
                        });
                    },
                    _ => ()
                }
            },
            Event::DeviceEvent {event: device_event, device_id: _} => {
                match device_event {
                    DeviceEvent::MouseMotion {delta} => {
                        event_queue.push(InputEvent::Mouse {
                            delta
                        });
                    },
                    _ => ()
                }
            },
            Event::MainEventsCleared => {
                if !game_frame(&mut event_queue, resize) {
                    *control_flow = ControlFlow::Exit;
                }
                resize = None;
            },
            _ => ()
        }
    });
}
