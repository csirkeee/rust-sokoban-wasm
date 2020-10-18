use macroquad::{get_time, next_frame, load_texture};
use specs::{RunNow, World, WorldExt};

mod audio;
mod components;
mod constants;
mod entities;
mod events;
mod map;
mod resources;
mod systems;

use crate::audio::*;
use crate::components::*;
use crate::map::*;
use crate::resources::*;
use crate::systems::*;
use specs::shred::FetchMut;
use std::ops::DerefMut;

struct Game {
    world: World,
}

impl Game {
    fn update(&mut self) {
        // Run input system
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }

        // Run gameplay state system
        {
            let mut gss = GameplayStateSystem {};
            gss.run_now(&self.world);
        }

        // Get and update time resource
        {
            let mut time = self.world.write_resource::<Time>();
            time.delta = get_time();
        }

        // Run event system
        {
            let mut es = EventSystem {};
            es.run_now(&self.world);
        }

        // Render game entities
        {
            let mut rs = RenderingSystem {};
            rs.run_now(&self.world);
        }
    }
}

async fn load_game_image(mut image_store: FetchMut<'_, ImageStore>, path: &str) {
    let ref mut image_map = image_store.images;
    if !image_map.contains_key(path) {
        image_map.insert(path.to_string(), load_texture(path).await);
    }
}


// Initialize the level
pub async fn initialize_level(world: &mut World) {
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . BB . . W
    W . . RB . . . W 
    W . P . . . . W
    W . . . . RS . W
    W . . BS . . . W
    W . . . . . . W
    W W W W W W W W
    ";

    load_game_image(world.fetch_mut::<ImageStore>(), "resources/images/box_blue_1.png").await;
    load_game_image(world.fetch_mut::<ImageStore>(), "resources/images/box_blue_2.png").await;
    load_game_image(world.fetch_mut::<ImageStore>(), "resources/images/box_red_1.png").await;
    load_game_image(world.fetch_mut::<ImageStore>(), "resources/images/box_red_2.png").await;
    load_game_image(world.fetch_mut::<ImageStore>(), "resources/images/box_spot_blue.png").await;
    load_game_image(world.fetch_mut::<ImageStore>(), "resources/images/box_spot_red.png").await;
    load_game_image(world.fetch_mut::<ImageStore>(), "resources/images/floor.png").await;
    load_game_image(world.fetch_mut::<ImageStore>(), "resources/images/player_1.png").await;
    load_game_image(world.fetch_mut::<ImageStore>(), "resources/images/player_2.png").await;
    load_game_image(world.fetch_mut::<ImageStore>(), "resources/images/player_3.png").await;
    load_game_image(world.fetch_mut::<ImageStore>(), "resources/images/wall.png").await;

    load_map(world, MAP.to_string());
}

#[macroquad::main("Sokoban")]
async fn main() {
    let mut world = World::new();
    register_components(&mut world);
    register_resources(&mut world);
    initialize_level(&mut world).await;

    // // Create a game context and event loop
    // let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
    //     .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
    //     .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
    //     .add_resource_path(path::PathBuf::from("./resources"));
    //
    // let (context, event_loop) = &mut context_builder.build()?;
    // initialize_sounds(&mut world, context);

    // Create the game state
    let game = &mut Game { world };

    // Run the main event loop
    loop {
        game.update();

        next_frame().await
    }
}
