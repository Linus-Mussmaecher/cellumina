use crate::automaton;

#[derive(Debug)]
pub(super) struct AutomatonModel {
    /// The contained automaton representing a cell state to draw.
    pub(super) cell_state: automaton::Automaton,
    /// Wether the simulation is currently paused, so only drawn and not progressed.
    pub(super) paused: bool,
    /// The current texture updated to the state of the automaton.
    pub(super) cell_state_texture: wgpu::Texture,
}

impl AutomatonModel {
    pub fn new(
        cell_state: automaton::Automaton,
        device: &wgpu::Device,
    ) -> (Self, wgpu::BindGroupLayout, wgpu::BindGroup) {
        let cell_state_texture = device.create_texture(&wgpu::TextureDescriptor {
            // the size of the texture
            size: wgpu::Extent3d {
                width: cell_state.dimensions().1,
                height: cell_state.dimensions().0,
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

        let cell_state_texture_view = cell_state_texture.create_view(&Default::default());
        let cell_state_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
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

        let cell_state_bind_group_layout =
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

        let cell_state_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &cell_state_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&cell_state_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&cell_state_sampler),
                },
            ],
        });

        (
            Self {
                cell_state,
                cell_state_texture,
                paused: false,
            },
            cell_state_bind_group_layout,
            cell_state_bind_group,
        )
    }

    pub(super) fn write_texture(&self, queue: &mut wgpu::Queue) {
        queue.write_texture(
            // copy destination
            wgpu::ImageCopyTextureBase {
                texture: &self.cell_state_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // actual pixel data
            &self.cell_state.create_image_buffer(),
            // internal layout
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * self.cell_state.dimensions().1),
                rows_per_image: Some(self.cell_state.dimensions().0),
            },
            // size as above
            wgpu::Extent3d {
                width: self.cell_state.dimensions().1,
                height: self.cell_state.dimensions().0,
                // ??
                depth_or_array_layers: 1,
            },
        );
    }

    pub(super) fn update(&mut self) -> bool {
        !self.paused && self.cell_state.next_step()
    }
}
