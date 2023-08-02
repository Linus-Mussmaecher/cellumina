use std::time::Instant;

use wgpu::util::DeviceExt;
use winit::window::Window;

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    render_pipeline: wgpu::RenderPipeline,

    pause: bool,

    // buffers
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,

    // info
    last_time: Instant,
    progress_speed: f32,
    info: ShaderInfo,
    info_buffer: wgpu::Buffer,
    info_bind_group: wgpu::BindGroup,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct ShaderInfo {
    time: f32,
    w: u32,
    h: u32,
}

/// Tree: instance  -> surface  -> device
///                             -> queue
///                 -> surface
const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, -1.0, 0.0],
    },
    Vertex {
        position: [3.0, -1.0, 0.0],
    },
    Vertex {
        position: [-1.0, 3.0, 0.0],
    },
];

const INDICES: &[u16] = &[0, 1, 2];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            // how far are two elements in the buffer from each other
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            // is an element in this buffer a single vertex or a single instance?
            step_mode: wgpu::VertexStepMode::Vertex,
            // what are the contents of this buffer?
            attributes: &Self::ATTRIBS,
            // &[wgpu::VertexAttribute {
            //     // starting point
            //     offset: 0,
            //     // where to store this? corresponds to attributes of struct
            //     shader_location: 0,
            //     // basically just the data type
            //     format: wgpu::VertexFormat::Float32x3,
            // }],
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

        // create & compile the shaders
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let info = ShaderInfo {
            time: 286.0, //96.0,
            w: size.width,
            h: size.height,
        };

        let info_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Info Buffer"),
            contents: bytemuck::cast_slice(&[info]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let info_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Info Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let info_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Info Bind Group"),
            layout: &info_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: info_buffer.as_entire_binding(),
            }],
        });

        // create the pipeline

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(
                &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&info_bind_group_layout],
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
            pause: false,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            last_time: Instant::now(),
            progress_speed: 1.0,
            info,
            info_buffer,
            info_bind_group,
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
                Some(winit::event::VirtualKeyCode::Left) => {
                    self.info.time -= 1.0 * self.progress_speed;
                }
                Some(winit::event::VirtualKeyCode::Right) => {
                    self.info.time += 1.0 * self.progress_speed;
                }
                Some(winit::event::VirtualKeyCode::Down) => {
                    self.info.time -= 0.1 * self.progress_speed;
                }
                Some(winit::event::VirtualKeyCode::Up) => {
                    self.info.time += 0.1 * self.progress_speed;
                }
                Some(winit::event::VirtualKeyCode::Equals)
                | Some(winit::event::VirtualKeyCode::Plus) => {
                    self.progress_speed *= 1.5;
                }
                Some(winit::event::VirtualKeyCode::Minus) => {
                    self.progress_speed *= 2.0 / 3.0;
                }
                Some(winit::event::VirtualKeyCode::P) => {
                    self.progress_speed = 1.0;
                }
                Some(winit::event::VirtualKeyCode::Space) => {
                    self.pause = !self.pause;
                    println!("{}", self.info.time);
                }
                Some(_) => {}
                None => {}
            }
        }

        false
    }

    pub(crate) fn update(&mut self) {
        self.info.w = self.size.width;
        self.info.h = self.size.height;
        let now = std::time::Instant::now();
        if !self.pause {
            self.info.time += (now.checked_duration_since(self.last_time))
                .unwrap_or_default()
                .as_secs_f32()
                * self.progress_speed;
        }
        self.last_time = now;
        self.queue
            .write_buffer(&self.info_buffer, 0, bytemuck::cast_slice(&[self.info]));
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
            render_pass.set_bind_group(0, &self.info_bind_group, &[]);
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
