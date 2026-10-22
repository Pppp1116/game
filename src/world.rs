use std::{marker::PhantomData, ops::Index};

use raylib::drawing::RaylibDrawHandle;

use crate::{
    grid::Grid,
    projectile::Projectile,
    soldiers::Soldier,
    traits::{Draw, TakeDamage, WorldObject},
};

#[derive(Debug)]
pub struct Collection<T: WorldObject> {
    pub items: Vec<Option<T>>,
    pub free_list: Vec<usize>,
}
impl<T: WorldObject> Default for Collection<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            free_list: Vec::new(),
        }
    }
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
    pub fn push<T: WorldObject + Clone>(&mut self, object: T) {
        let position = object.position();

        let Some(index) = self.add(object) else {
            return;
        };
        if let Some(cell) = self.grid.get_cell(&position) {
            cell.insert(T::inhabitant(index));
        }
    }
}

impl World {
    pub fn new(window_x: i32, widow_y: i32) -> World {
        let grid = Grid::init(window_x as usize, widow_y as usize);

        World {
            soldiers: Collection::default(),
            projectiles: Collection::default(),
            grid,
        }
    }
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
    pub fn check_bullet_colision(&mut self) {
        for (index, slot) in &mut self.projectiles.items.iter_mut().enumerate() {
            let Some(projectile) = slot.as_mut() else {
                continue;
            };

            let Some(cell) = self.grid.get_cell(&projectile.hitbox.position) else {
                continue;
            };
            if cell.check_neighbours(&mut self.soldiers, projectile) {
                if let Some(pos) = cell.habitants.iter().position(|inhabitants| matches!(inhabitants, crate::grid::Inhabitants::Pojectiles(i) if *i == index) ){
                    cell.habitants.swap_remove(pos);
                };

                //TODO maybe this should be a method
                *slot = None;
                self.projectiles.free_list.push(index);
            }
        }
    }
    pub fn draw(&self, rl: &mut RaylibDrawHandle<'_>) {
        for soldiers in &self.soldiers {
            soldiers.draw(rl);
        }
        for projectile in &self.projectiles {
            projectile.draw(rl);
        }
    }
}
