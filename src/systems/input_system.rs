use crate::components::*;
use crate::constants::*;
use crate::events::{EntityMoved, Event};
use crate::resources::{EventQueue, Gameplay};
use macroquad::{is_key_pressed, KeyCode};
use specs::world::Index;
use specs::{Entities, Join, ReadStorage, System, Write, WriteStorage};
use std::collections::HashMap;

pub struct InputSystem {}

type InputSystemData<'a> = (
    Write<'a, EventQueue>,
    Write<'a, Gameplay>,
    Entities<'a>,
    WriteStorage<'a, Position>,
    ReadStorage<'a, Player>,
    ReadStorage<'a, Movable>,
    ReadStorage<'a, Immovable>,
);

// System implementation
impl<'a> System<'a> for InputSystem {
    // Data
    type SystemData = InputSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {


        for &key in [KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right].iter() {
            if is_key_pressed(key) {
                self.handle_button_press(&mut data, key)
            }
        }
    }
}

impl InputSystem {
    fn handle_button_press(&mut self, data: &mut InputSystemData, key: KeyCode) {
        println!("Key pressed: {:?}", key);

        let (
            events,
            gameplay,
            entities,
            positions,
            players,
            movables,
            immovables,
        ) = data;

        let mut to_move = Vec::new();

        for (position, _player) in (&*positions, &*players).join() {
            // get all the movables and immovables
            let mov: HashMap<(u8, u8), Index> = (&*entities, &*movables, &*positions)
                .join()
                .map(|t| ((t.2.x, t.2.y), t.0.id()))
                .collect::<HashMap<_, _>>();
            let immov: HashMap<(u8, u8), Index> = (&*entities, &*immovables, &*positions)
                .join()
                .map(|t| ((t.2.x, t.2.y), t.0.id()))
                .collect::<HashMap<_, _>>();

            // Now iterate through current position to the end of the map
            // on the correct axis and check what needs to move.
            let (start, end, is_x) = match key {
                KeyCode::Up => (position.y, 0, false),
                KeyCode::Down => (position.y, MAP_HEIGHT, false),
                KeyCode::Left => (position.x, 0, true),
                KeyCode::Right => (position.x, MAP_WIDTH, true),
                _ => continue,
            };

            let range = if start < end {
                (start..=end).collect::<Vec<_>>()
            } else {
                (end..=start).rev().collect::<Vec<_>>()
            };

            for x_or_y in range {
                let pos = if is_x {
                    (x_or_y, position.y)
                } else {
                    (position.x, x_or_y)
                };

                // find a movable
                // if it exists, we try to move it and continue
                // if it doesn't exist, we continue and try to find an immovable instead
                match mov.get(&pos) {
                    Some(id) => to_move.push((key, id.clone())),
                    None => {
                        // find an immovable
                        // if it exists, we need to stop and not move anything
                        // if it doesn't exist, we stop because we found a gap
                        match immov.get(&pos) {
                            Some(_id) => {
                                to_move.clear();
                                events.events.push(Event::PlayerHitObstacle {})
                            }
                            None => break,
                        }
                    }
                }
            }
        }

        // We've just moved, so let's increase the number of moves
        if to_move.len() > 0 {
            gameplay.moves_count += 1;
        }

        // Now actually move what needs to be moved
        for (key, id) in to_move {
            let position = positions.get_mut(entities.entity(id));
            if let Some(position) = position {
                match key {
                    KeyCode::Up => position.y -= 1,
                    KeyCode::Down => position.y += 1,
                    KeyCode::Left => position.x -= 1,
                    KeyCode::Right => position.x += 1,
                    _ => (),
                }
            }

            // Fire an event for the entity that just moved
            events.events.push(Event::EntityMoved(EntityMoved { id }));
        }
    }
}
