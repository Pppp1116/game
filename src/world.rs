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
    //generations is a custom index to identify the soldiers
    generations: Vec<usize>,
    free_list: Vec<usize>,
}
impl<T: WorldObject> Default for Collection<T> {
    fn default() -> Self {
        Self {
            generations: Vec::new(),
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
    pub fn remove<T: WorldObject>(&mut self, index: usize) -> Option<()> {
        let position = {
            let collection = T::storage(self);
            let object = collection.items.get_mut(index)?.take()?;
            collection.free_list.push(index);
            object.position()
        };

        if let Some(cell) = self.grid.get_cell(&position) {
            cell.remove_inhabitant(&T::inhabitant(index));
        }

        Some(())
    }
    fn add<T: WorldObject>(&mut self, object: T) -> Option<usize> {
        let collection = T::storage(self);

        if let Some(index) = collection.free_list.pop() {
            *collection.items.get_mut(index)? = Some(object);
            if let Some(value) = collection.generations.get_mut(index) {
                *value += 1;
            };
            Some(index)
        } else {
            let index = collection.items.len();
            collection.items.push(Some(object));
            collection.generations.insert(index, 1);
            Some(index)
        }
    }
    fn check_bullet_colision(&mut self) {
        for index in 0..self.projectiles.items.len() {
            let result = {
                let Some(projectile) = self.projectiles.items[index].as_mut() else {
                    continue;
                };
                let position = projectile.hitbox.position;

                let Some(cell) = self.grid.get_cell(&position) else {
                    continue;
                };
                cell.check_neighbours(&mut self.soldiers, projectile)
            };
            let (hit, dead_soldier) = result;

            if let Some(soldier_index) = dead_soldier {
                self.remove::<Soldier>(soldier_index);
            }
            if hit {
                self.remove::<Projectile>(index);
            }
        }
    }

    fn move_projectiles(&mut self, dt: f32) {
        for (index, slot) in self.projectiles.items.iter_mut().enumerate() {
            let Some(projectile) = slot.as_mut() else {
                continue;
            };

            if let Some(cell) = self.grid.get_cell(&projectile.hitbox.position) {
                cell.remove_inhabitant(&Inhabitants::Projectiles(index));
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
    use std::time::{Duration, Instant};

    use crate::{
        grid::Inhabitants,
        hitbox::HitBox,
        projectile::Projectile,
        soldiers::{Soldier, Weapon},
    };

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

    #[test]
    fn moving_projectile_keeps_soldier_in_cell() {
        let mut world = World::new(200, 200);
        let soldier_hitbox = HitBox::new(Vector2::new(10.0, 10.0), Vector2::new(20.0, 20.0));
        let target_hitbox = HitBox::new(Vector2::new(90.0, 10.0), Vector2::new(20.0, 20.0));
        let weapon = Weapon::Gun {
            damage: 10.0,
            reatch: 100.0,
            last_shot: Instant::now(),
            fire_interval: Duration::from_millis(500),
        };
        let projectile = Projectile::new(soldier_hitbox.clone(), target_hitbox, &weapon);

        world.push(Soldier::new(soldier_hitbox));
        world.push(projectile);
        world.move_projectiles(0.01);

        let cell = world.grid.get_cell(&Vector2::new(10.0, 10.0)).unwrap();
        assert_eq!(cell.habitants.len(), 2);
        assert!(cell.habitants.contains(&Inhabitants::Soldiers(0)));
        assert!(cell.habitants.contains(&Inhabitants::Projectiles(0)));
    }

    #[test]
    fn soldier_index() {
        let mut word = World::new(200, 200);
    }
}
