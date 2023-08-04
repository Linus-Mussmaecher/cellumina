use std::time::{Duration, Instant};

/// A struct that represents the drawable state of the cellular automaton
pub struct CellState {
    /// The current cell state
    cells: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    /// The current texture
    texture: wgpu::Texture,
    /// The bind group used to draw the cell to the image.
    pub cells_bind_group: wgpu::BindGroup,
    /// The size of the automaton
    dimensions: (u32, u32),
    /// The time between two rule applicatoins
    interval: Duration,
    /// The last time the cell state was transformed.
    last_step: Instant,
}

impl CellState {
    /// Creates a new cell state full of black cells.
    pub fn new(device: &wgpu::Device) -> (Self, wgpu::BindGroupLayout) {
        let dimensions = (200, 100);
        let mut cells =
            image::ImageBuffer::from_pixel(dimensions.0, dimensions.1, image::Rgba([0; 4]));

        for (index, pixel) in cells.pixels_mut().enumerate() {
            if index < 200 {
                *pixel = image::Rgba([255; 4]);
            }
        }

        // a texture - note that this is more of a 'storage location' and does not know anything of the bytes yet! Only the size needs to fit.
        let cells_texture = device.create_texture(&wgpu::TextureDescriptor {
            // the size of the texture
            size: wgpu::Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                // ??
                depth_or_array_layers: 1,
            },
            // ??
            mip_level_count: 1,
            // For displaying, will only be samples once?
            sample_count: 1,
            // not a 3D-object
            dimension: wgpu::TextureDimension::D2,
            // we converted to rgba8 above
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            // TEXTURE_BINDING = use in shaders, COPY_DST: data will be copied here
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("diffuse texture"),
            // might want to support additional view formats
            view_formats: &[],
        });

        let cells_texture_view = cells_texture.create_view(&Default::default());
        let cells_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            // what to do with coordinates outside the texture
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            // what to do when multiple pixels draw from one texture pixel
            mag_filter: wgpu::FilterMode::Nearest,
            // what to do when multiple texture pixels fit on one actual pixel
            min_filter: wgpu::FilterMode::Nearest,
            // whatever a mipmap is
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let cells_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        // what shaders this is used in
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            // ??
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            // 2D
                            view_dimension: wgpu::TextureViewDimension::D2,
                            // wether to use multiple samples
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let cells_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &cells_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&cells_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&cells_sampler),
                },
            ],
        });

        (
            Self {
                cells,
                texture: cells_texture,
                cells_bind_group,
                dimensions,
                interval: Duration::from_secs_f32(0.5),
                last_step: Instant::now(),
            },
            cells_bind_group_layout,
        )
    }

    pub fn write(&self, queue: &wgpu::Queue) {
        queue.write_texture(
            // copy destination
            wgpu::ImageCopyTextureBase {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // actual pixel data
            &self.cells,
            // internal layout
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * self.dimensions.0),
                rows_per_image: Some(self.dimensions.1),
            },
            // size as above
            wgpu::Extent3d {
                width: self.dimensions.0,
                height: self.dimensions.1,
                // ??
                depth_or_array_layers: 1,
            },
        );
    }

    /// Applies all rules to update
    pub fn update(&mut self) {
        if Instant::now() - self.last_step < self.interval {
            return;
        }
        self.last_step = Instant::now();

        let mut next_cells = self.cells.clone();

        for (index, pixel) in next_cells.pixels_mut().enumerate() {
            *pixel = *self.cells.get_pixel(
                index as u32 % self.dimensions.0,
                (index as u32 / self.dimensions.0).wrapping_sub(1) % self.dimensions.1,
            );
        }

        self.cells = next_cells;
    }
}
