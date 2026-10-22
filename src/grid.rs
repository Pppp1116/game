use raylib::ffi::Vector2;

use crate::{
    projectile::{self, Projectile},
    soldiers::{Soldier, Weapon},
    traits::{TakeDamage, WorldObject},
    world::{Collection, World},
};

#[derive(Clone, Debug)]
pub(crate) enum Inhabitants {
    Soldiers(usize),
    Pojectiles(usize),
}

#[derive(Default, Clone)]
pub(crate) struct Cell {
    pub(crate) habitants: Vec<Inhabitants>,
}
impl Cell {
    pub fn insert(&mut self, inhabitant: Inhabitants) {
        self.habitants.push(inhabitant);
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

        self.cells.get_mut(row * self.collums + collum)
    }
}
