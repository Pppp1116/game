use crate::hitbox::HitBox;
use crate::traits::Draw;
use crate::{
    soldiers::Weapon,
    traits::{Movable, WorldObject},
    world::Collection,
};
use raylib::ffi::Vector2;
use raylib::prelude::*;

#[derive(Clone)]
pub struct Projectile {
    pub hitbox: HitBox,
    pub previus_position: Vector2,
    pub velocity: Vector2,
    wepond: Weapon,
}
const PROJECTILE_SPEED: f32 = 320.0;

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
        let size = Vector2::new(4.0, 4.0);
        let from_center = from.position + from.size * 0.5;
        let to_center = to.position + to.size * 0.5;
        let toward_target = to_center - from_center;
        let direction = if toward_target.length_sqr() > 0.0 {
            toward_target.normalize()
        } else {
            Vector2::new(1.0, 0.0)
        };
        let muzzle_distance = (from.size.x.hypot(from.size.y) + size.x.hypot(size.y)) * 0.5 + 1.0;
        let position = from_center + direction * muzzle_distance - size * 0.5;
        let hitbox = HitBox::new(position, size);

        //TODO diff speed for diff weaponds
        Self {
            previus_position: position,
            hitbox: hitbox,
            velocity: direction * PROJECTILE_SPEED,
            wepond: weapond.clone(),
        }
    }
    pub fn update(&mut self, dt: f32) {
        self.previus_position = self.hitbox.position;
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
        crate::grid::Inhabitants::Projectiles(index)
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
            self.hitbox.size.x as i32,
            self.hitbox.size.y as i32,
            Color::ORANGE,
        );
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;

    #[test]
    fn projectile_starts_clear_of_shooter_and_moves() {
        let shooter = HitBox::new(Vector2::new(50.0, 50.0), Vector2::new(20.0, 20.0));
        let target = HitBox::new(Vector2::new(100.0, 50.0), Vector2::new(20.0, 20.0));
        let weapon = Weapon::Gun {
            damage: 10.0,
            reatch: 100.0,
            last_shot: Instant::now(),
            fire_interval: Duration::from_millis(500),
        };

        let mut projectile = Projectile::new(shooter.clone(), target, &weapon);
        assert!(!projectile.hitbox.intersects(&shooter));

        let start_x = projectile.hitbox.position.x;
        projectile.update(0.5);
        assert!(projectile.hitbox.position.x > start_x);
    }
}
