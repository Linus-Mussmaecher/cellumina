use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ShaderInfo {
    pub w: u32,
    pub h: u32,
    pub cells_w: u32,
    pub cells_h: u32,
}

pub struct ShaderInfoContainer {
    pub info: ShaderInfo,
    pub info_bind_group: wgpu::BindGroup,
    buffer: wgpu::Buffer,
}

impl ShaderInfoContainer {
    pub fn create(device: &wgpu::Device) -> (Self, wgpu::BindGroupLayout) {
        let info = ShaderInfo {
            w: 0,
            h: 0,
            cells_w: 0,
            cells_h: 0,
        };

        let info_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let info_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Info Buffer"),
            contents: bytemuck::cast_slice(&[info]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        (
            Self {
                info,
                info_bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Info Bind Group"),
                    layout: &info_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: info_buffer.as_entire_binding(),
                    }],
                }),
                buffer: info_buffer,
            },
            info_layout,
        )
    }

    pub fn update(&mut self, w: u32, h: u32, cells_w: u32, cells_h: u32, queue: &wgpu::Queue) {
        self.info.w = w;
        self.info.h = h;
        self.info.cells_w = cells_w;
        self.info.cells_h = cells_h;
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.info]));
        println!("writing: {} {}", w, h);
    }
}
