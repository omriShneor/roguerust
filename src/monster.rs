use specs::prelude::*;
use super::{Viewshed, Position, Monster};
use rltk::{console, Point};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = ( ReadStorage<'a, Viewshed>, 
                        ReadExpect<'a, Point>,
                        ReadStorage<'a, Monster>);

    fn run(&mut self, data: Self::SystemData) {
        let (viewshed, player_pos, monster) = data;

        for (viewshed,_monster) in (&viewshed,&monster).join() {
            if viewshed.visible_tiles.contains(&*&player_pos) {
                console::log("Monster considers their own existence");
            }
        }
    }
}