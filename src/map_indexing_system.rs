use specs::prelude::*;
use super::{Map, Position, BlocksTile};

pub struct MapIndexingSystem{}

impl<'a> System<'a> for MapIndexingSystem{
    type SystemData = (WriteExpect<'a, Map>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, BlocksTile>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, positions, blockers) = data;

        for(pos, _blocker) in (&positions, &blockers).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            map.blocked[idx] = true;
        }
    }
}
