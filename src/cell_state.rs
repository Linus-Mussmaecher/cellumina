use std::{
    path::Path,
    time::{Duration, Instant},
};

use crate::rules;

pub type CellGrid = grid::Grid<char>;

/// A struct that represents the drawable state of the cellular automaton
pub struct CellState {
    /// The current cell state
    cell_grid: CellGrid,
    /// The current texture
    texture: wgpu::Texture,
    /// The bind group used to draw the cell to the image.
    pub cells_bind_group: wgpu::BindGroup,
    /// The time between two rule applicatoins
    interval: Duration,
    /// The last time the cell state was transformed.
    last_step: Instant,

    avg_time: Duration,
    avg_weight: u32,

    /// The rule
    rule: Box<dyn rules::Rule>,
}

impl CellState {
    /// Turns a grid into a usable image to be turned into a texture.
    /// Rows of the grid turn into image height, columns into width.
    pub fn grid_to_texture(grid: &CellGrid) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        image::ImageBuffer::from_fn(grid.size().1 as u32, grid.size().0 as u32, |x, y| {
            image::Rgba(match grid[y as usize][x as usize] {
                ' ' => [0; 4],             //[255; 4],
                'X' => [86, 181, 78, 255], //[232, 212, 100, 255],
                'F' => [235, 64, 52, 255],
                'A' => [235, 125, 125, 255],
                'S' => [185, 23, 45, 255],
                _ => [0; 4],
            })
        })
    }

    /// Returns the size of the underlying grid as a two element tuple.
    /// First element are the number of rows and the second the columns.
    pub fn size(&self) -> (u32, u32) {
        (
            self.cell_grid.size().0 as u32,
            self.cell_grid.size().1 as u32,
        )
    }

    /// Creates a new cellular automaton from an initial state
    pub fn new_from_file(
        device: &wgpu::Device,
        path: impl AsRef<Path>,
    ) -> (Self, wgpu::BindGroupLayout) {
        // read file
        let content = std::fs::read_to_string(path).expect("Could not read file.");
        // split into lines
        let lines: Vec<&str> = content.split('\n').collect();
        // get number of columns (chars in largest line)
        // subtracting one from each line because of leftover newline
        let cols = lines
            .iter()
            .map(|line| line.len().saturating_sub(1))
            .max()
            .unwrap_or_default();

        // create grid to hold data
        let mut grid = grid::Grid::<char>::new(0, cols);

        // iterate over lines and add them to the grid
        for line in lines {
            // create char vector
            let mut chars: Vec<char> = line.replace('\r', "").chars().collect();
            // make sure vector is neither to large nor to small
            chars.resize(cols, ' ');
            // push to the grid
            grid.push_row(chars);
        }

        //TODO: Switch to log?
        println!(
            "Initializing Cellular automaton of size {} x {}.",
            grid.size().1,
            grid.size().0
        );

        // use basic contstructor with created grid
        Self::new(device, grid)
    }

    /// Creates a new cell state full of black cells.
    pub fn new(device: &wgpu::Device, cell_grid: CellGrid) -> (Self, wgpu::BindGroupLayout) {
        // a texture - note that this is more of a 'storage location' and does not know anything of the bytes yet! Only the size needs to fit.
        let cells_texture = device.create_texture(&wgpu::TextureDescriptor {
            // the size of the texture
            size: wgpu::Extent3d {
                width: cell_grid.size().1 as u32,
                height: cell_grid.size().0 as u32,
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
                cell_grid,
                texture: cells_texture,
                cells_bind_group,
                interval: Duration::from_secs_f32(0.1),
                last_step: Instant::now(),
                rule: Box::new(rules::pattern_rule::PatternRule::new_sand()),
                avg_time: Duration::ZERO,
                avg_weight: 0,
            },
            cells_bind_group_layout,
        )
    }

    /// Writes a texture corresponding to this cell_states grid to a texture buffer (as created in the constructor).
    pub fn write(&mut self, queue: &wgpu::Queue) {
        queue.write_texture(
            // copy destination
            wgpu::ImageCopyTextureBase {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // actual pixel data
            &Self::grid_to_texture(&self.cell_grid),
            // internal layout
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * self.cell_grid.size().1 as u32),
                rows_per_image: Some(self.cell_grid.size().0 as u32),
            },
            // size as above
            wgpu::Extent3d {
                width: self.cell_grid.size().1 as u32,
                height: self.cell_grid.size().0 as u32,
                // ??
                depth_or_array_layers: 1,
            },
        );
    }

    /// Checks if another update is scheduled and applies it.
    /// Returns wether an update was applied.
    pub fn update(&mut self) -> bool {
        if Instant::now() - self.last_step < self.interval {
            return false;
        }
        self.last_step = Instant::now();

        self.rule.transform(&mut self.cell_grid);

        let last_time = self.last_step.elapsed();

        self.avg_time = (self.avg_time * self.avg_weight + last_time) / (self.avg_weight + 1);
        self.avg_weight += 1;

        println!(
            "Time: {:.4}s | {:.4}s ({:.4} fps)",
            last_time.as_secs_f32(),
            self.avg_time.as_secs_f32(),
            1. / self.avg_time.as_secs_f32(),
        );

        true
    }
}
