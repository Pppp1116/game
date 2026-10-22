use raylib::ffi::Vector2;

#[derive(Clone, Debug)]
pub struct HitBox {
    pub position: Vector2,
    pub velocity: Vector2,
    pub size: Vector2,
}

impl HitBox {
    pub fn intersects(&self, other: &Self) -> bool {
        self.position.x < other.position.x + other.size.x
            && self.position.x + self.size.x > other.position.x
            && self.position.y < other.position.y + other.size.y
            && self.position.y + self.size.y > other.position.y
    }

    pub fn new(position: Vector2) -> Self {
        HitBox {
            position: position,
            velocity: Vector2::new(0.0, 0.0),
            size: Vector2::new(4.0, 4.0),
        }
    }
}
