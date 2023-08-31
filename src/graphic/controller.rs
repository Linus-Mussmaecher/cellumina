/// A part of the MVC pattern, describing the state of various input devices of a live-run automaton.
#[derive(Debug, Clone)]
pub(super) struct AutomatonController {
    /// The cell the user's mouse is currently hovering.
    hovered_cell: Option<(u32, u32)>,
    /// The current state of the main mouse button.
    mouse_down: bool,
    /// The current state of the Ctrl-Key
    ctrl_down: bool,
    /// The char the currently hovered cell is replaced with on mouse click.
    replacement_char: char,
    /// The keymap used to convert from VirtualKeyCode to character
    keymap: std::collections::HashMap<winit::event::VirtualKeyCode, char>,
}

impl AutomatonController {
    /// Creates a new AutomatonController with default state.
    pub fn new() -> Self {
        Self {
            hovered_cell: None,
            mouse_down: false,
            ctrl_down: false,
            replacement_char: 'X',
            keymap: get_keymap(),
        }
    }

    /// Modifies the passed model as orderd by the user input.
    pub(crate) fn modify(&self, model: &mut super::AutomatonModel) -> bool {
        if self.mouse_down {
            if let Some((row, col)) = self.hovered_cell {
                return model
                    .cell_state
                    .set_cell(row, col, crate::char_to_id(self.replacement_char))
                    .unwrap_or_else(|err| {
                        log::error!("Could not set cell state: {}.", err);
                        false
                    });
            }
        }
        false
    }

    /// Handles a window event to update input state. If the event is not used, false is returned.
    pub(crate) fn handle_event(
        &mut self,
        model: &mut super::AutomatonModel,
        config: &wgpu::SurfaceConfiguration,
        event: &winit::event::WindowEvent<'_>,
    ) -> bool {
        match event {
            // Check for Keyboard events
            winit::event::WindowEvent::KeyboardInput {
                input:
                    winit::event::KeyboardInput {
                        virtual_keycode,
                        state: winit::event::ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                // Differ based on keycode
                match virtual_keycode {
                    // S: If control is down, try to save
                    Some(winit::event::VirtualKeyCode::S) if self.ctrl_down => {
                        log::info!("Attempting to save current state to file.");
                        let (rows, cols) = model.cell_state.state.size();
                        match native_dialog::FileDialog::new()
                            .set_location("~")
                            .set_filename("cellumina_output")
                            .add_filter("Cellumina Text", &["txt"])
                            .add_filter("PNG Image", &["png"])
                            .add_filter("JPEG Image", &["jpeg"])
                            .add_filter("ICO image", &["ico"])
                            .add_filter("BMP Image", &["bmp"])
                            .show_save_single_file()
                        {
                            Err(e) => log::error!("File Dialog Error: {e}"),
                            Ok(pathbuff_option) => match pathbuff_option {
                                None => log::info!("File Dialog aborted."),
                                Some(pathbuffer) => {
                                    match pathbuffer.extension().and_then(std::ffi::OsStr::to_str) {
                                        Some("png") | Some("jpeg") | Some("ico") | Some("bmp") => {
                                            if let Err(e) = image::save_buffer(
                                                pathbuffer,
                                                &model.cell_state.create_image_buffer(),
                                                cols as u32,
                                                rows as u32,
                                                image::ColorType::Rgba8,
                                            ) {
                                                log::error!(
                                                    "Writing automaton to image file failed: {e}"
                                                );
                                            }
                                        }
                                        Some("txt") | None => {
                                            if let Err(e) = std::fs::write(
                                                pathbuffer,
                                                model.cell_state.state.iter().fold(
                                                    String::with_capacity((cols + 1) * rows),
                                                    |mut container, &cell| {
                                                        if container.len() % (cols + 1) == cols {
                                                            container.push('\n');
                                                        }
                                                        container.push(crate::id_to_char(cell));
                                                        container
                                                    },
                                                ),
                                            ) {
                                                log::error!(
                                                    "Writing automaton to text file failed: {e}"
                                                )
                                            }
                                        }
                                        Some(ext) => {
                                            log::error!(
                                                "Detected unsupported file extension: {}",
                                                ext
                                            );
                                        }
                                    }
                                }
                            },
                        }

                        true
                    }
                    // Return pauses and unpauses.
                    Some(winit::event::VirtualKeyCode::Return) => {
                        log::info!(
                            "Model simulation {}.",
                            if model.paused { "unpaused" } else { "paused" }
                        );
                        model.paused = !model.paused;
                        true
                    }
                    // All other chars (including S): Set the replacement char
                    Some(code) => {
                        self.replacement_char = self.keymap.get(code).copied().unwrap_or(' ');
                        log::info!("Replacement Character set to {}.", self.replacement_char);
                        true
                    }
                    // Else, do nothing
                    None => false,
                }
            }
            // Keep tabs on the CTRL key.
            winit::event::WindowEvent::ModifiersChanged(state) => {
                std::mem::replace(&mut self.ctrl_down, state.ctrl()) != state.ctrl()
            }
            // Permantly know what cell the cursor is hovering
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                // calculate the height and width of a cell if the state was stretched to the whole window
                let pixels_per_col = config.width as f64 / model.cell_state.dimensions().1 as f64; // pixels per cell
                let pixels_per_row = config.height as f64 / model.cell_state.dimensions().0 as f64; // pixels per cell

                // since the state is only stretched until either direction reaches the window borders, the true side length of a cell is the minimum
                let pixels_per_cell = pixels_per_col.min(pixels_per_row);

                let (cell_row, cell_col) = (
                    ((position.y - config.height as f64 / 2.) / pixels_per_cell
                        + model.cell_state.dimensions().0 as f64 / 2.),
                    ((position.x - config.width as f64 / 2.) / pixels_per_cell
                        + model.cell_state.dimensions().1 as f64 / 2.),
                );

                if 0. <= cell_col
                    && cell_col < model.cell_state.dimensions().1 as f64
                    && 0. <= cell_row
                    && cell_row < model.cell_state.dimensions().0 as f64
                {
                    self.hovered_cell = Some((cell_row as u32, cell_col as u32));
                } else {
                    self.hovered_cell = None
                }

                true
            }
            // Mouse click set the cell state.
            winit::event::WindowEvent::MouseInput {
                state,
                button: winit::event::MouseButton::Left,
                ..
            } => {
                match state {
                    winit::event::ElementState::Pressed => self.mouse_down = true,
                    winit::event::ElementState::Released => self.mouse_down = false,
                }
                true
            }

            _ => false,
        }
    }
}

/// Returns a basic keymap mapping VirtualKeyCodes to chars.
fn get_keymap() -> std::collections::HashMap<winit::event::VirtualKeyCode, char> {
    std::collections::HashMap::from([
        (winit::event::VirtualKeyCode::Space, ' '),
        (winit::event::VirtualKeyCode::A, 'A'),
        (winit::event::VirtualKeyCode::B, 'B'),
        (winit::event::VirtualKeyCode::C, 'C'),
        (winit::event::VirtualKeyCode::D, 'D'),
        (winit::event::VirtualKeyCode::E, 'E'),
        (winit::event::VirtualKeyCode::F, 'F'),
        (winit::event::VirtualKeyCode::G, 'G'),
        (winit::event::VirtualKeyCode::H, 'H'),
        (winit::event::VirtualKeyCode::I, 'I'),
        (winit::event::VirtualKeyCode::J, 'J'),
        (winit::event::VirtualKeyCode::K, 'K'),
        (winit::event::VirtualKeyCode::L, 'L'),
        (winit::event::VirtualKeyCode::M, 'M'),
        (winit::event::VirtualKeyCode::N, 'N'),
        (winit::event::VirtualKeyCode::O, 'O'),
        (winit::event::VirtualKeyCode::P, 'P'),
        (winit::event::VirtualKeyCode::Q, 'Q'),
        (winit::event::VirtualKeyCode::R, 'R'),
        (winit::event::VirtualKeyCode::S, 'S'),
        (winit::event::VirtualKeyCode::T, 'T'),
        (winit::event::VirtualKeyCode::U, 'U'),
        (winit::event::VirtualKeyCode::V, 'V'),
        (winit::event::VirtualKeyCode::W, 'W'),
        (winit::event::VirtualKeyCode::X, 'X'),
        (winit::event::VirtualKeyCode::Y, 'Y'),
        (winit::event::VirtualKeyCode::Z, 'Z'),
        (winit::event::VirtualKeyCode::Key1, '1'),
        (winit::event::VirtualKeyCode::Key2, '2'),
        (winit::event::VirtualKeyCode::Key3, '3'),
        (winit::event::VirtualKeyCode::Key4, '4'),
        (winit::event::VirtualKeyCode::Key5, '5'),
        (winit::event::VirtualKeyCode::Key6, '6'),
        (winit::event::VirtualKeyCode::Key7, '7'),
        (winit::event::VirtualKeyCode::Key8, '8'),
        (winit::event::VirtualKeyCode::Key9, '9'),
        (winit::event::VirtualKeyCode::Key0, '0'),
    ])
}
