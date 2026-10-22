use crate::hitbox::HitBox;
use crate::traits::Draw;
use crate::{
    soldiers::Weapon,
    traits::{Movable, WorldObject},
    world::Collection,
};
use raylib::ffi::Vector2;
use raylib::prelude::*;

pub(crate) struct Projectile {
    pub hitbox: HitBox,
    pub velocity: Vector2,
    wepond: Weapon,
}
const PROJECTILE_SPEED: f32 = 32.0;

impl Projectile {
    pub fn damage(&self) -> f32 {
        match self.wepond {
            Weapon::Grenade {
                reatch,
                radius,
                damage,
                last_shot,
                fire_interval,
            } => damage,
            Weapon::Gun {
                damage,
                reatch,
                last_shot,
                fire_interval,
            } => damage,
        }
    }
    pub fn new(from: HitBox, to: HitBox, weapond: &Weapon) -> Self {
        let direction = (to.position - from.position).normalize();

        //TODO diff speed for diff weaponds
        Self {
            hitbox: from,
            velocity: direction * PROJECTILE_SPEED,
            wepond: weapond.clone(),
        }
    }
    pub fn update(&mut self, dt: f32) {
        self.hitbox.position += self.velocity * dt;
    }
}
impl WorldObject for Projectile {
    fn storage(world: &mut crate::world::World) -> &mut Collection<Self> {
        &mut world.projectiles
    }
    fn position(&self) -> Vector2 {
        self.hitbox.position
    }
    fn inhabitant(index: usize) -> crate::grid::Inhabitants {
        crate::grid::Inhabitants::Pojectiles(index)
    }
}

impl Movable for Projectile {
    fn move_by(&mut self, ammount: Vector2) {
        self.hitbox.position += ammount;
    }
    fn velocity(&self) -> &Vector2 {
        &self.velocity
    }
}

impl Draw for Projectile {
    fn draw(&self, d: &mut raylib::prelude::RaylibDrawHandle<'_>) {
        d.draw_rectangle(
            self.hitbox.position.x as i32,
            self.hitbox.position.y as i32,
            2,
            2,
            Color::ORANGE,
        );
    }
}
