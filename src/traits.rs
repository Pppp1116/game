use crate::grid::Inhabitants;
use crate::health::Health;
use crate::hitbox::HitBox;
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
    fn velocity(&self) -> &Vector2;
}

pub trait WorldObject: Sized {
    fn storage(world: &mut World) -> &mut Collection<Self>;
    fn position(&self) -> Vector2;
    fn inhabitant(index: usize) -> Inhabitants;
}

pub trait TakeDamage {
    fn hitbox(&self) -> &HitBox;
    fn health(&mut self) -> &mut Health;
    fn take_damage(&mut self, damage: f32);
}

pub trait Armed: TakeDamage + Movable {
    fn shoot_at<T: TakeDamage>(&mut self, target: &T) -> Option<Projectile> {
        if self.in_range(&target.hitbox()) && self.weapon().can_shoot() {
            match self.mut_weapon() {
                Weapon::Grenade { last_shot, .. } | Weapon::Gun { last_shot, .. } => {
                    *last_shot = Instant::now();
                }
            }
            return Some(Projectile::new(
                self.hitbox().clone(),
                target.hitbox().clone(),
                self.weapon(),
            ));
        }
        None
    }
    fn weapon(&self) -> &Weapon;
    fn mut_weapon(&mut self) -> &mut Weapon;
    fn in_range(&self, target: &HitBox) -> bool {
        self.hitbox().position.distance_sqr(target.position) <= self.weapon().reatch().powi(2)
    }
}
