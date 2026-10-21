use crate::health::Health;
use crate::projectile::Projectile;
use crate::soldiers::Weapon;
use crate::world::Collection;
use crate::world::World;
use raylib::prelude::*;
use std::time::Instant;
pub trait Draw {
    fn draw(&self, d: &mut RaylibDrawHandle<'_>);
}

pub trait Movable {
    fn move_by(&mut self, ammount: Vector2);
    fn velocity(&self) -> Vector2;
}

pub trait WorldObject: Sized {
    fn storage(world: &mut World) -> &mut Collection<Self>;
}

pub trait TakeDamage {
    fn position(&self) -> Vector2;
    fn health(&mut self) -> &mut Health;
}

pub trait Armed: TakeDamage + Movable {
    fn shoot_at<T: TakeDamage>(&mut self, target: &T) -> Option<Projectile> {
        if self.in_range(target.position()) && self.weapon().can_shoot() {
            match self.mut_weapon() {
                Weapon::Grenade { last_shot, .. } | Weapon::Gun { last_shot, .. } => {
                    *last_shot = Instant::now();
                }
            }
            return Some(Projectile::new(
                self.position(),
                target.position(),
                self.weapon(),
            ));
        }
        None
    }
    fn weapon(&self) -> &Weapon;
    fn mut_weapon(&mut self) -> &mut Weapon;
    fn in_range(&self, target: Vector2) -> bool {
        self.position().distance_sqr(target) <= self.weapon().reatch().powi(2)
    }
}
