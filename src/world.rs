use crate::{
    projectile::Projectile,
    soldiers::Soldier,
    traits::{TakeDamage, WorldObject},
};

pub struct World {
    pub soldiers: Vec<Soldier>,
    pub projectiles: Vec<Projectile>,
}

impl World {
    fn remove<T: WorldObject>(&mut self, index: usize) {
        T::storage(self).swap_remove(index);
    }
    //TODO add update fn verify if projetile touched guu if yes deal damage if guy life is <= 0 kill
    //him
}
