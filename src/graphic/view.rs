use wgpu::util::DeviceExt;

use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
    window::WindowBuilder,
};

use super::vertex;
use crate::automaton;

/// A struct that holds an automaton and a lot of WebGL-State and can run and display that automaton.
#[derive(Debug)]
pub(super) struct AutomatonView {
    // ----- MODEL -----
    model: super::AutomatonModel,

    // ----- VIEW -----
    /// The WebGL Surface.
    surface: wgpu::Surface,
    /// The WebGL Device.
    device: wgpu::Device,
    /// The WebGL Queue.
    queue: wgpu::Queue,
    /// The WebGL Config.
    config: wgpu::SurfaceConfiguration,
    /// The winit-window being drawn to.
    window: Window,
    /// The WebGL Render Pipeline.
    render_pipeline: wgpu::RenderPipeline,

    /// The current vertex buffer. Should always contain 4 Vertices forming a rectangle, but their positions may change.
    vertex_buffer: wgpu::Buffer,
    /// The current index buffer (should not change, as we always draw a rectangle).
    index_buffer: wgpu::Buffer,

    // ----- CONTROLLER -----
    /// The AutomatonModifier that deals with user interaction with the cell state.
    modifier: super::AutomatonController,
}

impl AutomatonView {
    /// Creates a new AutomatonDisplayer to draw the passed automaton to the passed window.
    async fn new(window: Window, automaton: automaton::Automaton) -> Self {
        // +-------------------------------------------------------------+
        // |                                                             |
        // |                   GENERAL SETUP                             |
        // |    Everything below can be kept for most kinds of drawing   |
        // |                                                             |
        // +-------------------------------------------------------------+

        // steal window size
        let size = window.inner_size();

        // create the instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // create the surface unsafely
        let surface =
            unsafe { instance.create_surface(&window) }.expect("Could not create surface.");

        // create adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Could not create adapter.");

        // create device & queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None, // Trace path
            )
            .await
            .expect("Could not create device/queue.");

        // get capabilities of surface
        let surface_caps = surface.get_capabilities(&adapter);
        // find an srgb surface format
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats.first().copied().unwrap());

        // create surface config
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes.first().copied().unwrap(),
            alpha_mode: surface_caps.alpha_modes.first().copied().unwrap(),
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        // +-------------------------------------------------------------+
        // |                                                             |
        // |     Creating texture and bind groups for automaton state    |
        // |                                                             |
        // +-------------------------------------------------------------+

        let (model, cell_state_bind_group_layout) = super::AutomatonModel::new(automaton, &device);

        // +-------------------------------------------------------------+
        // |                                                             |
        // |         Creating shader, render pipeline and buffers        |
        // |                                                             |
        // +-------------------------------------------------------------+

        // create & compile the shaders
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        // create the pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(
                &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&cell_state_bind_group_layout], //&[&info_bind_group_layout],
                    push_constant_ranges: &[],
                }),
            ),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[vertex::Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None,
        });

        // create the buffer
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            // name
            label: Some("Vertex Buffer"),
            // actual contents
            contents: bytemuck::cast_slice(&vertex::VERTICES),
            // vertex buffer or index buffer?
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(vertex::INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            // Model
            model,
            // View
            surface,
            device,
            queue,
            config,
            window,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            // Controller
            modifier: super::AutomatonController::new(),
        }
    }

    /// Sets the physical window size whereever needed and also calculates the maximum rectangle with the same side length ratio as the contained automaton
    /// still containable in this window and sets the vertex positions of the vertex buffer to the corners of that rectangle.
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        // update a lot of stuff
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);

        // get new vertex positions to keep ratio of display consistent
        let mut vertices = vertex::VERTICES;
        // Calculate ratios
        let cell_ratio = self.model.cell_state.dimensions().1 as f32
            / self.model.cell_state.dimensions().0 as f32;
        let win_ratio = new_size.width as f32 / new_size.height as f32;

        // Based on the larger ratio, make the rectangle thinner or lower.
        if cell_ratio > win_ratio {
            for v in vertices.iter_mut() {
                v.position[1] *= win_ratio / cell_ratio;
            }
        } else {
            for v in vertices.iter_mut() {
                v.position[0] *= cell_ratio / win_ratio;
            }
        }

        // update the vertex buffer
        self.queue
            .write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
    }

    /// Attempts to update the automaton, and if an update has happened writes its state to the buffers.
    fn update(&mut self) {
        if self.modifier.modify(&mut self.model)
            | (!self.model.paused && self.model.cell_state.next_step())
        {
            self.queue.write_texture(
                // copy destination
                wgpu::ImageCopyTextureBase {
                    texture: &self.model.cell_state_texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                // actual pixel data
                &self.model.cell_state.create_image_buffer(),
                // internal layout
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * self.model.cell_state.dimensions().1),
                    rows_per_image: Some(self.model.cell_state.dimensions().0),
                },
                // size as above
                wgpu::Extent3d {
                    width: self.model.cell_state.dimensions().1,
                    height: self.model.cell_state.dimensions().0,
                    // ??
                    depth_or_array_layers: 1,
                },
            );
        }
    }

    /// Handles all sorts of window events.
    fn window_events(&mut self, control_flow: &mut ControlFlow, event: &WindowEvent<'_>) {
        match event {
            // close requested => close
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            // resize requested => resize
            WindowEvent::Resized(physical_size) => {
                self.resize(*physical_size);
            }
            // different kind of resize requested => still resize
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                // new_inner_size is &&mut so we have to dereference it twice
                self.resize(**new_inner_size);
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
                // switch its fullscreen state
                if self.window.fullscreen().is_none() {
                    self.window
                        .set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
                } else {
                    self.window.set_fullscreen(None);
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
                self.window.set_fullscreen(None);
            }
            _ => {}
        }
    }

    /// Renders the currently stored automaton state to the window.
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // get the current 'framebuffer'
        let output = self.surface.get_current_texture()?;
        // create a 'view' = definition how render code interacts with this texture
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        //create a command encoder
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // create a render pass that clears the screen
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                // id, bascially
                label: Some("Render Pass"),
                // what to do with color
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    // view from earlier
                    view: &view,
                    // no multisampling yet
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.6,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                // what to do with depth
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            //render_pass.set_bind_group(0, &self.info_bind_group, &[]);
            render_pass.set_bind_group(0, &self.model.cell_state_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            //render_pass.draw(0..3, 0..1);
            render_pass.draw_indexed(
                // number of indices
                0..vertex::INDICES.len() as u32,
                // ??
                0,
                // how many instances?
                0..1,
            );
        }

        // submit this pass to the command queue
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

/// Creates an [AutomatonDisplayer] for the passed [automaton::Automaton], creates a window
pub(crate) async fn run_live(automaton: automaton::Automaton) {
    let event_loop = EventLoop::new();

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

    let mut view = AutomatonView::new(window, automaton).await;

    event_loop.run(move |event, _event_loop_window_target, control_flow| {
        match event {
            // Window events
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == view.window.id() => {
                // first try to handle by the drawing state
                if !view
                    .modifier
                    .handle_event(&mut view.model, &view.config, event)
                {
                    // then handle events concerning the actual window
                    view.window_events(control_flow, event);
                }
            }
            Event::RedrawRequested(window_id) if window_id == view.window.id() => {
                view.update();
                match view.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => {
                        view.resize(PhysicalSize::new(view.config.width, view.config.height))
                    }
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
