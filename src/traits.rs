use crate::health::Health;
use crate::projectile::Projectile;
use crate::soldiers::Weapon;
use crate::world::World;
use raylib::prelude::*;
pub trait Draw {
    fn draw(&self, d: &mut RaylibDrawHandle<'_>);
}

pub trait Movable {
    fn move_by(&mut self, ammount: Vector2);
    fn move_to(&mut self, position: Vector2);
}

pub trait WorldObject: Sized {
    fn storage(world: &mut World) -> &mut Vec<Self>;
}

pub trait TakeDamage {
    fn health(&mut self) -> &mut Health;
    fn position(&self) -> Vector2;
}
//TODO when projectile hits player take damage
pub trait Armed: TakeDamage {
    fn shoot_at<T: TakeDamage>(&mut self, target: &T) -> Option<Projectile> {
        if self.in_range(target.position()) && self.weapon().can_shoot() {
            return Some(Projectile::new(
                self.position(),
                target.position(),
                self.weapon(),
            ));
        }
        None
    }
    fn weapon(&self) -> &Weapon;

    fn in_range(&self, target: Vector2) -> bool {
        self.position().distance_sqr(target) <= self.weapon().reatch().powi(2)
    }
}
