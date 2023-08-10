use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::cell_state::CellState;
use crate::shader_info::ShaderInfoContainer;

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    render_pipeline: wgpu::RenderPipeline,

    info: ShaderInfoContainer,

    // buffers
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,

    cell_state: CellState,
}

// Tree: instance  -> surface  -> device
//                             -> queue
//                 -> surface

/// Vertices forming the corners of a rectangle
const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1., -1., 0.0],
        tex_coords: [0., 1.],
    },
    Vertex {
        position: [1., -1., 0.0],
        tex_coords: [1., 1.],
    },
    Vertex {
        position: [-1., 1., 0.0],
        tex_coords: [0., 0.],
    },
    Vertex {
        position: [1., 1., 0.0],
        tex_coords: [1., 0.],
    },
];

// indices to draw this rectangle of two triangles.
const INDICES: &[u16] = &[0, 1, 2, 1, 3, 2];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            // how far are two elements in the buffer from each other
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            // is an element in this buffer a single vertex or a single instance?
            step_mode: wgpu::VertexStepMode::Vertex,
            // what are the contents of this buffer?
            attributes: &Self::ATTRIBS,
        }
    }
}

impl State {
    pub(crate) async fn new(window: Window) -> Self {
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
        // |    Everything above can be kept for most kinds of drawing   |
        // |                                                             |
        // +-------------------------------------------------------------+

        // Create the state of cells
        let (cell_state, cells_bind_group_layout) =
            CellState::new_from_file(&device, "./src/sand_init.txt");

        let (info, info_bind_group_layout) = ShaderInfoContainer::create(&device);

        // do a one-time-write
        //cell_state.write(&queue);

        // create & compile the shaders
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        // create the pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(
                &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&cells_bind_group_layout, &info_bind_group_layout], //&[&info_bind_group_layout],
                    push_constant_ranges: &[],
                }),
            ),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
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
            contents: bytemuck::cast_slice(VERTICES),
            // vertex buffer or index buffer?
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            cell_state,
            info,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.info.update(
                new_size.width,
                new_size.height,
                self.cell_state.size().0,
                self.cell_state.size().1,
                &self.queue,
            );
        }
    }

    pub(crate) fn input(&mut self, event: &winit::event::WindowEvent) -> bool {
        if let winit::event::WindowEvent::KeyboardInput {
            input:
                winit::event::KeyboardInput {
                    virtual_keycode,
                    state: winit::event::ElementState::Pressed,
                    ..
                },
            ..
        } = event
        {
            match virtual_keycode {
                Some(winit::event::VirtualKeyCode::Space) => {}
                Some(_) => {}
                None => {}
            }
        }

        false
    }

    pub(crate) fn update(&mut self) {
        if self.cell_state.update() {
            self.cell_state.write(&self.queue);
        }
    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
            render_pass.set_bind_group(0, &self.cell_state.cells_bind_group, &[]);
            render_pass.set_bind_group(1, &self.info.info_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            //render_pass.draw(0..3, 0..1);
            render_pass.draw_indexed(
                // number of indices
                0..INDICES.len() as u32,
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
