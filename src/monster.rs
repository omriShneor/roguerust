use specs::prelude::*;
use crate::{Map, Position};

use super::{Viewshed, Monster, Name};
use rltk::{console, Point, a_star_search};


pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = ( WriteStorage<'a, Viewshed>, 
                        ReadExpect<'a, Point>,
                        WriteExpect<'a, Map>,
                        ReadStorage<'a, Monster>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut viewshed, player_pos, mut map, monster, name,mut position) = data;

        for (viewshed,_monster, name, pos) in (&mut viewshed,&monster,&name, &mut position).join() {
            let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
            if distance < 1.5 {
                console::log(format!("{} shouts insults!",name.name));
                return;
            }

            if viewshed.visible_tiles.contains(&*player_pos) {
                let path = a_star_search(map.xy_idx(pos.x, pos.y) as i32,
                                         map.xy_idx(player_pos.x, player_pos.y) as i32,
                                         &mut *map);
                if path.success && path.steps.len() > 1 {
                    pos.x = (path.steps[1] % map.width) as i32;
                    pos.y = (path.steps[1] / map.width) as i32;
                    viewshed.dirty = true;
                }
            }
        }
    }
}