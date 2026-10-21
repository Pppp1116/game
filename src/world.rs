use std::{marker::PhantomData, ops::Index};

use crate::{
    grid::Grid,
    projectile::Projectile,
    soldiers::Soldier,
    traits::{TakeDamage, WorldObject},
};

pub struct Collection<T: WorldObject> {
    pub items: Vec<Option<T>>,
    pub free_list: Vec<usize>,
}

impl<'a, T: WorldObject> IntoIterator for &'a Collection<T> {
    type Item = &'a T;
    type IntoIter =
        std::iter::FilterMap<std::slice::Iter<'a, Option<T>>, fn(&'a Option<T>) -> Option<&'a T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter().filter_map(Option::as_ref)
    }
}
impl<'a, T: WorldObject> IntoIterator for &'a mut Collection<T> {
    type Item = &'a mut T;
    type IntoIter = std::iter::FilterMap<
        std::slice::IterMut<'a, Option<T>>,
        fn(&mut Option<T>) -> Option<&mut T>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.items
            .iter_mut()
            .filter_map(Option::as_mut as fn(&mut Option<T>) -> Option<&mut T>)
    }
}

pub struct World {
    pub soldiers: Collection<Soldier>,
    pub projectiles: Collection<Projectile>,
    pub grid: Grid,
}

impl World {
    fn remove<T: WorldObject>(&mut self, index: usize) -> Option<()> {
        let collection = T::storage(self);

        if collection.items.get_mut(index)?.take().is_some() {
            collection.free_list.push(index);
            Some(())
        } else {
            None
        }
    }
    fn add<T: WorldObject>(&mut self, object: T) -> Option<usize> {
        let collection = T::storage(self);

        if let Some(index) = collection.free_list.pop() {
            *collection.items.get_mut(index)? = Some(object);
            Some(index)
        } else {
            let index = collection.items.len();
            collection.items.push(Some(object));
            Some(index)
        }
    }
    fn check_bullet_colision(&mut self) {
        for projectile in &mut self.projectiles {
            let Some(grid) = self.grid.get_grid(&projectile.position) else {
                continue;
            };
        }
    }
}
