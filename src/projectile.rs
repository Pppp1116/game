use raylib::ffi::Vector2;

use crate::{soldiers::Weapon, traits::WorldObject};

pub(crate) struct Projectile {
    position: Vector2,
    velocity: Vector2,
    wepond: Weapon,
}
const PROJECTILE_SPEED: f32 = 32.0;

impl Projectile {
    pub fn new(from: Vector2, to: Vector2, weapond: &Weapon) -> Self {
        let direction = (to - from).normalize();

        //TODO diff speed for diff weaponds
        Self {
            position: from,
            velocity: direction * PROJECTILE_SPEED,
            wepond: weapond.clone(),
        }
    }
    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }
}
impl WorldObject for Projectile {
    fn storage(world: &mut crate::world::World) -> &mut Vec<Self> {
        &mut world.projectiles
    }
}
