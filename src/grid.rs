use raylib::ffi::Vector2;

use crate::{
    projectile::{self, Projectile},
    soldiers::{Soldier, Weapon},
    traits::{TakeDamage, WorldObject},
    world::{Collection, World},
};

#[derive(Clone, Debug)]
pub enum Inhabitants {
    Soldiers(usize),
    Projectiles(usize),
}

#[derive(Default, Clone)]
pub struct Cell {
    pub(crate) habitants: Vec<Inhabitants>,
}
impl Cell {
    pub fn insert(&mut self, inhabitant: Inhabitants) {
        self.habitants.push(inhabitant);
    }

    pub(crate) fn remove_projectile(&mut self, index: usize) {
        if let Some(position) = self
            .habitants
            .iter()
            .position(|inhabitant| matches!(inhabitant, Inhabitants::Projectiles(i) if *i == index))
        {
            self.habitants.swap_remove(position);
        }
    }
}
impl<'a> IntoIterator for &'a Cell {
    type Item = &'a Inhabitants;
    type IntoIter = std::slice::Iter<'a, Inhabitants>;

    fn into_iter(self) -> Self::IntoIter {
        self.habitants.iter()
    }
}
impl<'a> IntoIterator for &'a mut Cell {
    type Item = &'a mut Inhabitants;
    type IntoIter = std::slice::IterMut<'a, Inhabitants>;

    fn into_iter(self) -> Self::IntoIter {
        self.habitants.iter_mut()
    }
}

impl Cell {
    pub fn check_neighbours(
        &mut self,
        soldiers: &mut Collection<Soldier>,
        weapon: &mut Projectile,
    ) -> bool {
        let mut hit = false;
        for inhabitant in &self.habitants {
            let Inhabitants::Soldiers(index) = inhabitant else {
                continue;
            };
            let Some(Some(soldier)) = soldiers.items.get_mut(*index) else {
                continue;
            };
            if weapon.hitbox.intersects(soldier.hitbox()) {
                soldier.take_damage(weapon.damage());
                hit = true;
            }
        }
        hit
    }
}

pub struct Grid {
    cells: Box<[Cell]>,
    collums: usize,
    rows: usize,
}

const CELL_SIZE: usize = 100;
impl Grid {
    pub fn init(width: usize, height: usize) -> Self {
        let collums = width.div_ceil(CELL_SIZE);
        let rows = height.div_ceil(CELL_SIZE);

        let cells = vec![Cell::default(); collums * rows].into_boxed_slice();

        Grid {
            collums,
            rows,
            cells,
        }
    }
    pub fn get_cell(&mut self, position: &Vector2) -> Option<&mut Cell> {
        if !position.x.is_finite()
            || !position.y.is_finite()
            || position.x < 0.0
            || position.y < 0.0
        {
            return None;
        }

        let collum = (position.x / CELL_SIZE as f32).floor() as usize;
        let row = (position.y / CELL_SIZE as f32).floor() as usize;

        if collum >= self.collums || row >= self.rows {
            return None;
        }

        self.cells.get_mut(row * self.collums + collum)
    }
    pub fn overlapping(
        &self,
        position: &Vector2,
        distance: f32,
    ) -> impl Iterator<Item = &Cell> + '_ {
        let size = CELL_SIZE as f32;

        let min_x = ((position.x - distance).max(0.0) / size) as usize;
        let max_x = (((position.x + distance) / size) as usize).min(self.collums - 1);

        let min_y = ((position.y - distance).max(0.0) / size) as usize;
        let max_y = (((position.y + distance) / size) as usize).min(self.rows - 1);

        (min_y..=max_y)
            .flat_map(move |y| (min_x..=max_x).map(move |x| &self.cells[y * self.collums + x]))
    }
}
