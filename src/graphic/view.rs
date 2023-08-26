use wgpu::util::DeviceExt;

use winit::{event::*, event_loop::ControlFlow, window::Window};

use super::vertex;
use crate::automaton;

/// A part of the MVC pattern, describing the OpenGL state and windoww of the view of a live-run automaton.
#[derive(Debug)]
pub(super) struct AutomatonView {
    // ----- VIEW -----
    /// The WebGL Surface.
    surface: wgpu::Surface,
    /// The WebGL Device.
    device: wgpu::Device,
    /// The WebGL Queue.
    pub(super) queue: wgpu::Queue,
    /// The WebGL Config.
    pub(super) config: wgpu::SurfaceConfiguration,
    /// The winit-window being drawn to.
    pub(super) window: Window,
    /// The WebGL Render Pipeline.
    render_pipeline: wgpu::RenderPipeline,

    /// The current vertex buffer. Should always contain 4 Vertices forming a rectangle, but their positions may change.
    vertex_buffer: wgpu::Buffer,
    /// The current index buffer (should not change, as we always draw a rectangle).
    index_buffer: wgpu::Buffer,
    /// The bind group used to draw the automaton's cells to the image.
    cell_state_bind_group: wgpu::BindGroup,
}

impl AutomatonView {
    /// Creates a new AutomatonDisplayer to draw the passed automaton to the passed window.
    pub(super) async fn create_view_model(
        window: Window,
        automaton: automaton::Automaton,
    ) -> (Self, super::AutomatonModel) {
        // +-------------------------------------------------------------+
        // |                                                             |
        // |                   GENERAL SETUP                             |
        // |    Everything below can be kept for most kinds of drawing   |
        // |                                                             |
        // +-------------------------------------------------------------+

        log::info!("Starting WGPU setup");

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

        let (model, cell_state_bind_group_layout, cell_state_bind_group) =
            super::AutomatonModel::new(automaton, &device);

        // +-------------------------------------------------------------+
        // |                                                             |
        // |         Creating shader, render pipeline and buffers        |
        // |                                                             |
        // +-------------------------------------------------------------+

        log::info!("Creating shader.");

        // create & compile the shaders
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        log::info!("Creating render pipeline.");

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

        log::info!("Creating vertex & index buffers.");

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

        (
            Self {
                surface,
                device,
                queue,
                config,
                window,
                render_pipeline,
                vertex_buffer,
                index_buffer,
                cell_state_bind_group,
            },
            model,
        )
    }

    /// Sets the physical window size whereever needed and also calculates the maximum rectangle with the same side length ratio as the contained automaton
    /// still containable in this window and sets the vertex positions of the vertex buffer to the corners of that rectangle.
    pub(super) fn resize(
        &mut self,
        new_size: winit::dpi::PhysicalSize<u32>,
        model_dimensions: (u32, u32),
    ) {
        // update a lot of stuff
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);

        // get new vertex positions to keep ratio of display consistent
        let mut vertices = vertex::VERTICES;
        // Calculate ratios
        let cell_ratio = model_dimensions.1 as f32 / model_dimensions.0 as f32;
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

    /// Handles all sorts of window events that are not related to input affecting the model (these are handled by the controller)
    /// but instead directly affecting the window and view state.
    pub(super) fn window_events(
        &mut self,
        control_flow: &mut ControlFlow,
        event: &WindowEvent<'_>,
        model_dimensions: (u32, u32),
    ) {
        match event {
            // close requested => close
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            // resize requested => resize
            WindowEvent::Resized(physical_size) => {
                self.resize(*physical_size, model_dimensions);
            }
            // different kind of resize requested => still resize
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                // new_inner_size is &&mut so we have to dereference it twice
                self.resize(**new_inner_size, model_dimensions);
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
    pub(super) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
            render_pass.set_bind_group(0, &self.cell_state_bind_group, &[]);
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
