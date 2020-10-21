use crate::components::*;
use specs::{Builder, World, WorldExt};

// Create a wall entity
pub fn create_wall(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable::new_static(
            "resources/images/wall.png".to_string(),
        ))
        .with(Wall {})
        .with(Immovable)
        .build();
}

pub fn create_floor(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 5, ..position })
        .with(Renderable::new_static(
            "resources/images/floor.png".to_string(),
        ))
        .build();
}

pub fn create_box(world: &mut World, position: Position, colour: BoxColour) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable::new_animated(vec![
            format!("resources/images/box_{}_1.png", colour),
            format!("resources/images/box_{}_2.png", colour),
        ]))
        .with(Box { colour })
        .with(Movable)
        .build();
}

pub fn create_box_spot(world: &mut World, position: Position, colour: BoxColour) {
    world
        .create_entity()
        .with(Position { z: 9, ..position })
        .with(Renderable::new_static(format!(
            "resources/images/box_spot_{}.png",
            colour
        )))
        .with(BoxSpot { colour })
        .build();
}

pub fn create_player(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable::new_animated(vec![
            "resources/images/player_1.png".to_string(),
            "resources/images/player_2.png".to_string(),
            "resources/images/player_3.png".to_string(),
        ]))
        .with(Player {})
        .with(Movable)
        .build();
}
