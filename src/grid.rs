use raylib::ffi::Vector2;

use crate::{
    projectile::{self, Projectile},
    soldiers::{Soldier, Weapon},
    traits::{TakeDamage, WorldObject},
};

#[derive(Clone, Debug)]
enum Inhabitants {
    Soldiers(usize),
    Pojectiles(usize),
}

#[derive(Default, Clone)]
struct Cell {
    habitants: Vec<Inhabitants>,
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
    pub fn check_neighbours(&mut self, weapon: &mut Projectile) {
        for item in self {}
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
    pub fn get_cell(&self, position: &Vector2) -> Option<&Cell> {
        if !position.x.is_finite()
            || !position.y.is_finite()
            || position.x < 0.0
            || position.y < 0.0
        {
            return None;
        }

        let collum = (position.x / CELL_SIZE as f32).floor() as usize;
        let row = (position.y / CELL_SIZE as f32).floor() as usize;

        self.cells.get(row * self.collums + collum)
    }
}
