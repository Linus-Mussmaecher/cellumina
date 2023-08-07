// #![cfg_attr(
//     all(target_os = "windows", not(debug_assertions),),
//     windows_subsystem = "windows"
// )]

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod cell_state;
mod rule;
mod state;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_inner_size(winit::dpi::Size::Physical(winit::dpi::PhysicalSize {
            width: 630,
            height: 500,
        }))
        // for now
        .with_resizable(false)
        .with_title("Cellumina")
        //.with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)))
        .build(&event_loop)
        .expect("Could not init window.");

    let mut state = state::State::new(window).await;

    event_loop.run(move |event, _event_loop_window_target, control_flow| {
        match event {
            // Window events
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                // first try to handle by the drawing state
                if !state.input(event) {
                    // then handle events concerning the actual window
                    window_events(&mut state, control_flow, event);
                }
            }
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                state.window().request_redraw();
            }
            _ => {}
        }
    });
}

fn window_events(
    state: &mut state::State,
    control_flow: &mut ControlFlow,
    event: &WindowEvent<'_>,
) {
    match event {
        // close requested => close
        WindowEvent::CloseRequested => {
            *control_flow = ControlFlow::Exit;
        }
        // resize requested => resize
        WindowEvent::Resized(physical_size) => {
            state.resize(*physical_size);
        }
        // different kind of resize requested => still resize
        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
            // new_inner_size is &&mut so we have to dereference it twice
            state.resize(**new_inner_size);
        }
        // handle all sorts of keyboard input
        // F11 => Switch fullscreen
        WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    virtual_keycode: Some(VirtualKeyCode::F11),
                    state: ElementState::Pressed,
                    ..
                },
            ..
        } => {
            // get the window
            let window = state.window();
            // switch its fullscreen state
            if window.fullscreen().is_none() {
                window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
            } else {
                window.set_fullscreen(None);
            }
        }
        // Escape => Exit fullscreen
        WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    state: ElementState::Pressed,
                    ..
                },
            ..
        } => {
            state.window().set_fullscreen(None);
        }
        _ => {}
    }
}
