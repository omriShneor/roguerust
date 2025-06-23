use rltk::{GameState, Point, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;

mod components;
pub use components::*;

mod player;
pub use player::*;

mod map;
pub use map::*;

pub mod rect;
pub use rect::*;

pub mod monster;
pub use monster::*;

#[derive(PartialEq, Clone, Copy)]
pub enum RunState { Paused, Running }

struct State {
    ecs: World,
    runstate: RunState
}

impl State {
    fn run_systems(&mut self) {
        let mut player_movement = PlayerMovementSystem {};
        player_movement.run_now(&self.ecs); 

        let mut visability= VisiabilitySystem {};
        visability.run_now(&self.ecs);

        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);
        
        self.ecs.maintain();
    }

    fn player_input(&mut self, ctx: &mut Rltk) -> RunState {
        // Fetch the player entity
        let player_entity = self.ecs.fetch::<PlayerEntity>().0;
        let mut move_intents = self.ecs.write_storage::<PlayerMovementIntent>();
        
        match ctx.key {
            None => return RunState::Paused,
            Some(key) => match key {
                VirtualKeyCode::Left => {
                    move_intents.insert(player_entity, PlayerMovementIntent { delta_x: -1, delta_y: 0 })
                        .expect("Unable to insert");
                },
                VirtualKeyCode::Right => {
                    move_intents.insert(player_entity, PlayerMovementIntent { delta_x: 1, delta_y: 0 })
                        .expect("Unable to insert");
                },
                VirtualKeyCode::Up => {
                    move_intents.insert(player_entity, PlayerMovementIntent { delta_x: 0, delta_y: -1 })
                        .expect("Unable to insert");
                },
                VirtualKeyCode::Down => {
                    move_intents.insert(player_entity, PlayerMovementIntent { delta_x: 0, delta_y: 1 })
                        .expect("Unable to insert");
                },
                _ => return RunState::Paused
            },
        }
        RunState::Running
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        draw_map(&self.ecs, ctx);

        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = self.player_input(ctx);
        }

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
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
    let mut gs = State{ ecs: World::new(), runstate: RunState::Running};

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<PlayerMovementIntent>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();

    let map = Map::new(80, 50);
    let player_init_pos = map.rooms[0].center();
    let mut rng = rltk::RandomNumberGenerator::new();

    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let p = room.center();
        let glyph : rltk::FontCharType;
        let name: String;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => { glyph = rltk::to_cp437('g'); name = "Goblin".to_string()}
            _ => { glyph = rltk::to_cp437('o'); name = "Orc".to_string()}
        }
        gs.ecs.create_entity()
        .with(Position{ x:p.x,y:p.y })
        .with(Renderable{
            glyph,
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Viewshed{ visible_tiles : Vec::new(), range: 8, dirty: true })
        .with(Monster{})
        .with(Name{ name: format!("{} #{}",&name ,i )})
        .build();
    }

    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_init_pos.x, player_init_pos.y));

    let player_entity = gs.ecs 
        .create_entity() 
        .with(Position {x: player_init_pos.x, y: player_init_pos.y})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLUE)
        })
        .with(Player {})
        .with(Viewshed{ visible_tiles : Vec::new(), range :8, dirty: true})
        .with(Name {name: "Player".to_string()})
        .build();
    
    gs.ecs.insert(PlayerEntity(player_entity));

    rltk::main_loop(context, gs)
}