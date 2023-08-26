/// Contains the [vertex::Vertex] struct and some related const objects.
mod vertex;
/// Contains the [displayer::AutomatonDisplayer] struct.
mod view;
use view::AutomatonView;

mod controller;
use controller::AutomatonController;

mod model;
use model::AutomatonModel;

use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::automaton;

/// Creates an [AutomatonDisplayer] for the passed [automaton::Automaton], creates a window
pub(crate) async fn run_live(automaton: automaton::Automaton) {
    let event_loop = EventLoop::new();

    log::info!("Starting window initialization.");

    let window = WindowBuilder::new()
        .with_inner_size(winit::dpi::Size::Physical(winit::dpi::PhysicalSize {
            width: 630,
            height: 500,
        }))
        // for now
        .with_resizable(true)
        .with_title("Cellumina")
        .build(&event_loop)
        .expect("Could not init window.");

    log::info!("Created window.");

    let (mut view, mut model) = AutomatonView::create_view_model(window, automaton).await;

    log::info!("Created view and model.");

    let mut controller = AutomatonController::new();

    log::info!("Created controller.");

    log::info!("Initializing event loop. Starting simulation.");

    event_loop.run(move |event, _event_loop_window_target, control_flow| {
        match event {
            // Window events
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == view.window.id() => {
                // first try to handle by the drawing state
                if !controller.handle_event(&mut model, &view.config, event) {
                    // then handle events concerning the actual window
                    view.window_events(control_flow, event, model.cell_state.dimensions());
                }
            }
            Event::RedrawRequested(window_id) if window_id == view.window.id() => {
                if model.update() || controller.modify(&mut model) {
                    model.write_texture(&mut view.queue);
                }

                match view.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => view.resize(
                        PhysicalSize::new(view.config.width, view.config.height),
                        model.cell_state.dimensions(),
                    ),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => log::error!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                view.window.request_redraw();
            }
            _ => {}
        }
    });
}
