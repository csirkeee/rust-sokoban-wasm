use crate::components::*;
use crate::constants::TILE_WIDTH;
use crate::resources::*;
use itertools::Itertools;
use macroquad::{
    clear_background, draw_text, draw_texture, load_texture, Color, BLACK, WHITE,
};
use specs::{Join, Read, ReadStorage, System, Write};
use std::{collections::HashMap};

pub struct RenderingSystem {}

impl RenderingSystem {
    pub fn draw_text(&mut self, text_string: &str, x: f32, y: f32) {
        draw_text(text_string, x, y, 20., BLACK);
    }

    pub fn get_image(&mut self, renderable: &Renderable, delta: f64) -> String {
        let path_index = match renderable.kind() {
            RenderableKind::Static => {
                // We only have one image, so we just return that
                0
            }
            RenderableKind::Animated => {
                // If we have multiple, we want to select the right one based on the delta time.
                // First we get the delta in milliseconds, we % by 1000 to get the milliseconds
                // only and finally we divide by 250 to get a number between 0 and 4. If it's 4
                // we technically are on the next iteration of the loop (or on 0), but we will let
                // the renderable handle this logic of wrapping frames.
                ((delta / 0.25) as usize) % 4
            }
        };

        renderable.path(path_index)
    }
}

// System implementation
impl<'a> System<'a> for RenderingSystem {
    // Data
    type SystemData = (
        Read<'a, Gameplay>,
        Read<'a, Time>,
        Write<'a, ImageStore>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (gameplay, time, mut image_store, positions, renderables) = data;

        // Clearing the screen (this gives us the backround colour)
        clear_background(Color::new(0.95, 0.95, 0.95, 1.0));

        // Get all the renderables with their positions.
        let rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        let mut rendering_batches: HashMap<u8, HashMap<String, Vec<(f32, f32)>>> = HashMap::new();

        // Iterate each of the renderables, determine which image path should be rendered
        // at which drawparams, and then add that to the rendering_batches.
        for (position, renderable) in rendering_data.iter() {
            // Load the image
            let image_path = self.get_image(renderable, time.delta);

            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;
            let z = position.z;

            // Add to rendering batches
            rendering_batches
                .entry(z)
                .or_default()
                .entry(image_path)
                .or_default()
                .push((x, y));
        }

        // Iterate spritebatches ordered by z and actually render each of them
        for (_z, group) in rendering_batches
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        {
            for (image_path, draw_params) in group {
                let &image = image_store.images.get(image_path).unwrap();

                for (x, y) in draw_params.iter() {
                    draw_texture(image, *x, *y, WHITE)
                }
            }
        }

        // Render any text
        self.draw_text(&gameplay.state.to_string(), 525.0, 80.0);
        self.draw_text(&gameplay.moves_count.to_string(), 525.0, 100.0);
        let fps = format!("FPS: {:}", time.frame_times.len());
        self.draw_text(&fps, 525.0, 120.0);
    }
}
