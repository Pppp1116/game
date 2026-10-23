use raylib::drawing::RaylibDrawHandle;

use crate::{
    grid::{Grid, Inhabitants},
    projectile::Projectile,
    soldiers::Soldier,
    traits::{Armed, Draw, TakeDamage, WorldObject},
};

#[derive(Debug)]
pub struct Collection<T: WorldObject> {
    pub items: Vec<Option<T>>,
    free_list: Vec<usize>,
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
    fn check_bullet_colision(&mut self) {
        for (index, slot) in &mut self.projectiles.items.iter_mut().enumerate() {
            let Some(projectile) = slot.as_mut() else {
                continue;
            };

            let Some(cell) = self.grid.get_cell(&projectile.hitbox.position) else {
                continue;
            };
            if cell.check_neighbours(&mut self.soldiers, projectile) {
                cell.remove_projectile(index);
                *slot = None;
                self.projectiles.free_list.push(index);
            }
        }
    }
    fn move_projectiles(&mut self, dt: f32) {
        for (index, slot) in self.projectiles.items.iter_mut().enumerate() {
            let Some(projectile) = slot.as_mut() else {
                continue;
            };

            if let Some(cell) = self.grid.get_cell(&projectile.hitbox.position) {
                cell.remove_projectile(index);
            }

            projectile.update(dt);

            if let Some(cell) = self.grid.get_cell(&projectile.hitbox.position) {
                cell.insert(Inhabitants::Projectiles(index));
            } else {
                *slot = None;
                self.projectiles.free_list.push(index);
            }
        }
    }
    pub fn update(&mut self, dt: f32, rl: &mut RaylibDrawHandle<'_>) {
        self.update_shooting();
        self.move_projectiles(dt);
        self.check_bullet_colision();
        self.draw(rl);
    }
    fn draw(&self, rl: &mut RaylibDrawHandle<'_>) {
        for soldiers in &self.soldiers {
            soldiers.draw(rl);
        }
        for projectile in &self.projectiles {
            projectile.draw(rl);
        }
    }
    pub fn update_shooting(&mut self) {
        let mut projectiles = Vec::new();

        for shooter_index in 0..self.soldiers.items.len() {
            let Some(shooter) = self.soldiers.items[shooter_index].as_ref() else {
                continue;
            };

            let position = shooter.position();
            let distance = shooter.fov().distance;

            'search: for cell in self.grid.overlapping(&position, distance) {
                for inhabitant in cell {
                    let Inhabitants::Soldiers(index) = inhabitant else {
                        continue;
                    };

                    let Ok([shooter_slot, target_slot]) = self
                        .soldiers
                        .items
                        .get_disjoint_mut([shooter_index, *index])
                    else {
                        continue;
                    };
                    let (Some(shooter), Some(target)) =
                        (shooter_slot.as_mut(), target_slot.as_ref())
                    else {
                        continue;
                    };

                    if let Some(projectile) = shooter.shoot_at(target) {
                        projectiles.push(projectile);
                        break 'search;
                    }
                }
            }
        }

        for projectile in projectiles {
            self.push(projectile);
        }
    }
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{hitbox::HitBox, soldiers::Soldier};

    use super::World;

    #[test]
    fn a_soldier_does_not_target_itself() {
        let mut world = World::new(200, 200);
        world.push(Soldier::new(HitBox::new(
            Vector2::new(50.0, 50.0),
            Vector2::new(20.0, 20.0),
        )));

        world.update_shooting();

        assert!(world.projectiles.items.is_empty());
    }
}
