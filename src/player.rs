use std::cmp::{max,min};
use specs::prelude::*;
use crate::{Viewshed};

use rltk::{field_of_view,Point};
use super::{Player, PlayerMovementIntent, Position,TileType, Map};


pub struct PlayerEntity(pub Entity);

pub struct PlayerMovementSystem {} // System struct the players movement.

impl <'a> System<'a> for PlayerMovementSystem {
    type SystemData = (ReadStorage<'a, Player>,
                        WriteStorage<'a, PlayerMovementIntent>,
                        WriteStorage<'a, Position>,
                        ReadExpect<'a, Map>,
                        WriteStorage<'a, Viewshed>,
                        WriteExpect<'a, Point>);  
    fn run(&mut self, data : Self::SystemData) {
        let (players, mut intents, mut positions, map,mut viewshed, mut p) = data;
        for (_player, intention, pos, viewshed) in (&players, &intents, &mut positions, &mut viewshed).join() {

            let destination_x = pos.x + intention.delta_x;
            let destination_y = pos.y + intention.delta_y;

            if destination_x < 0 || destination_x >= map.width as i32 ||
                destination_y < 0 || destination_y >= map.height as i32 {
                    continue;
                }

            let dest_idx = map.xy_idx(destination_x, destination_y);
            if !map.blocked[dest_idx] {
                pos.x = destination_x;
                pos.y = destination_y;

                viewshed.dirty = true;

                p.x = destination_x;
                p.y = destination_y;
            }

        }
        intents.clear();
    }
}

pub struct  VisiabilitySystem {}

impl <'a> System<'a> for VisiabilitySystem {
    type SystemData = (WriteExpect<'a, Map>,
                        Entities<'a>,
                        WriteStorage<'a, Viewshed>, 
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Player>);
    fn run(&mut self, data : Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;
        for (ent, viewshed,pos) in (&entities, &mut viewshed, &pos).join() {
            // Only calculate this if the player moved somewhere.
            if viewshed.dirty {
                
                // reset the dirty
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < map.width as i32 && p.y >= 0 && p.y < map.height as i32);

                let p: Option<&Player> = player.get(ent);
                if let Some(_) = p {

                    // This is done to grayscale the visited tiles that are no longer visible.
                    map.visible_tiles.fill(false);

                    for vis in viewshed.visible_tiles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.revealed_tiles[idx] = true;
                        map.visible_tiles[idx] = true;
                    }
                }
            }
        }
    }
    
}
