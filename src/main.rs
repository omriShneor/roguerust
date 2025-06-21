use rltk::{Rltk, GameState, RGB, VirtualKeyCode};
use specs::prelude::*;

mod components;
pub use components::*;
mod player;
pub use player::*;
mod map;
pub use map::*;
pub mod rect;
pub use rect::*;


struct State {
    ecs: World
}

impl State {
    fn run_systems(&mut self) {
        let mut player_movement = PlayerMovementSystem {};
        player_movement.run_now(&self.ecs); 

        let mut player_visability= PlayerVisiabilitySystem {};
        player_visability.run_now(&self.ecs);
        
        self.ecs.maintain();
    }

    fn player_input(&mut self, ctx: &mut Rltk) {
        let mut move_intents = self.ecs.write_storage::<PlayerMovementIntent>();
        let players = self.ecs.read_storage::<Player>();
        let entities = self.ecs.entities();

        // Find the player entity
        for (entity, _player) in (&entities, &players).join() {
            match ctx.key {
                None => {}
                Some(key) => match key {
                    VirtualKeyCode::Left => {
                        move_intents.insert(entity, PlayerMovementIntent { delta_x: -1, delta_y: 0 }).expect("Unable to insert");
                    },
                    VirtualKeyCode::Right => {
                        move_intents.insert(entity, PlayerMovementIntent { delta_x: 1, delta_y: 0 }).expect("Unable to insert");
                    },
                    VirtualKeyCode::Up => {
                        move_intents.insert(entity, PlayerMovementIntent { delta_x: 0, delta_y: -1 }).expect("Unable to insert");
                    },
                    VirtualKeyCode::Down => {
                        move_intents.insert(entity, PlayerMovementIntent { delta_x: 0, delta_y: 1 }).expect("Unable to insert");
                    },
                    _ => {}
                },
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        draw_map(&self.ecs, ctx);

        self.player_input(ctx);
        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}


fn draw_map(ecs: &World, ctx : &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;
    for (idx,tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[idx] {
            let glyph;
            let mut fg;
            match tile {
                TileType::Floor => {
                    glyph = rltk::to_cp437('.');
                    fg = RGB::from_f32(0.0, 0.5, 0.5);
                }
                TileType::Wall => {
                    glyph = rltk::to_cp437('#');
                    fg = RGB::from_f32(0., 1.0, 0.);
                }
            }
            if !map.visible_tiles[idx] { fg = fg.to_greyscale() }
            ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
        }
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}



fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State{ ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<PlayerMovementIntent>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let map = Map::new(80, 50);
    let center = map.rooms[0].center();
    gs.ecs.insert(map);

    gs.ecs 
        .create_entity() 
        .with(Position {x: center.x, y: center.y})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLUE)
        })
        .with(Player {})
        .with(Viewshed{ visible_tiles : Vec::new(), range :8, dirty: true})
        .build();

    rltk::main_loop(context, gs)
}