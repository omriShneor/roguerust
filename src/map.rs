use crate::Rect;
use rltk::{Algorithm2D, Point, RandomNumberGenerator, BaseMap};

#[derive(PartialEq, Clone, Copy)]
pub enum TileType {
    Wall, Floor
}

pub struct Map {
    pub visible_tiles: Vec<bool>,
    pub revealed_tiles: Vec<bool>,
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: usize,
    pub height: usize
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width) + x as usize
    }

    pub fn new(width: usize, height: usize) -> Map {
        let mut tiles = vec![TileType::Wall; width*height];

        let mut rooms : Vec<Rect> = Vec::new();
        const MAX_ROOMS : i32 = 30;
        const MIN_SIZE : i32 = 6;
        const MAX_SIZE : i32 = 10;


        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, 80 - w - 1) - 1;
            let y = rng.roll_dice(1, 50 - h - 1) - 1;
            let new_room = Rect::new(Point::new(x, y),w, h);
            let mut ok = true;
            for other_room in rooms.iter() {
                if new_room.intersect(other_room) { ok = false }
            }
            if ok {
                for x in new_room.p1.x .. new_room.p2.x {
                    for y in new_room.p1.y .. new_room.p2.y {
                        tiles[(y as usize * width) + x as usize] = TileType::Floor;
                    }
                }
                rooms.push(new_room);            
            }
        }

        let mut map = Map {visible_tiles: vec![false; width*height], 
                                revealed_tiles: vec![false; width*height],
                                tiles, 
                                rooms: rooms.clone(), 
                                width,
                                height};

        map.connect_rooms_with_mst(&rooms);

        map
    }

    fn apply_tunnel_between_two_points(&mut self, p1: &Point, p2: &Point) {
        use std::cmp::{min,max};

        for x in min(p1.x, p2.x) ..=max(p1.x, p2.x) {
            let idx = self.xy_idx(x, p1.y);
            self.tiles[idx] = TileType::Floor;
        }

        for y in min(p1.y, p2.y) ..= max(p1.y,p2.y) {
            let idx = self.xy_idx(p2.x, y);
            self.tiles[idx] = TileType::Floor;
        }
    }

    fn connect_rooms_with_mst(&mut self, rooms: &Vec<Rect>) {
        if rooms.is_empty() {
            return;
        }
        let mut connected = vec![false; rooms.len()];
        connected[0] = true; 
        let mut connected_count = 1;
        while connected_count < rooms.len() {
            let mut best_distance = f64::MAX;
            let mut best_pair = (0, 0);
            for (i, _) in rooms.iter().enumerate() {
                if !connected[i] {
                    continue;
                }
                for (j, _) in rooms.iter().enumerate() {
                    if connected[j] { continue; }
                    let d = distance(&rooms[j].center(), &rooms[i].center());
                    if d < best_distance {
                        best_distance = d;
                        best_pair = (i, j);
                    }
                }
            }

            self.apply_tunnel_between_two_points(&rooms[best_pair.0].center(), &rooms[best_pair.1].center());
            connected[best_pair.1] = true;
            connected_count += 1;
        }
    }

    fn is_exit_valid(&self, x:i32, y:i32) -> bool {
        if x < 1 || x > self.width as i32 - 1 || y < 1 || y > self.height as i32 - 1 { return false; }
        let idx = self.xy_idx(x, y);
        self.tiles[idx as usize] != TileType::Wall
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}


impl BaseMap for Map {
    fn is_opaque(&self, idx:usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }
    fn get_available_exits(&self, idx:usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let x = idx as i32 % self.width as i32;
        let y = idx as i32 / self.width as i32;
        let w = self.width as usize;

        // Cardinal directions
        if self.is_exit_valid(x-1, y) { exits.push((idx-1, 1.0)) };
        if self.is_exit_valid(x+1, y) { exits.push((idx+1, 1.0)) };
        if self.is_exit_valid(x, y-1) { exits.push((idx-w, 1.0)) };
        if self.is_exit_valid(x, y+1) { exits.push((idx+w, 1.0)) };

        exits
    }
}

pub fn distance(p1: &Point, p2: &Point) -> f64 {
    ((p1.x - p2.x).pow(2) as f64 + (p1.y - p2.y).pow(2) as f64).sqrt()
}