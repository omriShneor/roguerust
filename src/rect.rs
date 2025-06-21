use rltk::Point;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub p1: Point,
    pub p2: Point
}

impl Rect {
    pub fn new(p: Point, w: i32, h: i32) -> Rect {
        Rect {p1: p, p2:Point::new(p.x + w, p.y+h)}
    }

    pub fn intersect(&self, other: &Rect) -> bool {
        self.p1.x <= other.p2.x && 
        self.p2.x >= other.p1.x && 
        self.p1.y <= other.p2.y && 
        self.p2.y >= other.p1.y
    } 
    
    pub fn center(&self) -> Point {
       Point::new((self.p1.x +self.p2.x)/2 as i32,(self.p1.y +self.p2.y)/2 as i32) 
    }
}

