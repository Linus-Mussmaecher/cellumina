use crate::automaton;

#[derive(Debug)]
pub(super) struct AutomatonModel {
    /// The contained automaton representing a cell state to draw.
    pub(super) cell_state: automaton::Automaton,
    /// Wether the simulation is currently paused, so only drawn and not progressed.
    pub(super) paused: bool,
    /// The current texture updated to the state of the automaton.
    pub(super) cell_state_texture: wgpu::Texture,
    /// The bind group used to draw the automaton's cells to the image.
    pub(super) cell_state_bind_group: wgpu::BindGroup,
}

impl AutomatonModel {
    pub fn new(
        cell_state: automaton::Automaton,
        device: &wgpu::Device,
    ) -> (Self, wgpu::BindGroupLayout) {
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
                cell_state_bind_group,
                paused: false,
            },
            cell_state_bind_group_layout,
        )
    }
}
