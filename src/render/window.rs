use winit::{
    event_loop::{ControlFlow, EventLoop},
    event::{Event, WindowEvent, KeyboardInput, ElementState, MouseButton},
    window::WindowBuilder,
};

#[derive(Debug)]
pub enum InputEvent {
    Key {code: u32, pressed: bool},
    Click {button: MouseButton, pressed: bool},
}

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
pub fn run_event_loop(window: Window, mut game_loop: impl FnMut(&mut Vec<InputEvent>) + 'static) {
    let mut event_queue = Vec::<InputEvent>::new();
    window.event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {event: window_event, window_id: _} => {
                match window_event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    },
                    WindowEvent::Resized(size) => {

                    },
                    WindowEvent::ScaleFactorChanged {new_inner_size, ..} => {

                    },
                    WindowEvent::KeyboardInput {input, device_id: _, is_synthetic: _} => {
                        event_queue.push(InputEvent::Key {
                            code: input.scancode,
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
            Event::MainEventsCleared => {
                game_loop(&mut event_queue);
            },
            _ => ()
        }
    });
}
