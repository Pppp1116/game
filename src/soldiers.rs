use std::time::{Duration, Instant};

use crate::{
    health::Health,
    traits::{Armed, Draw, Movable, TakeDamage},
};
use raylib::prelude::*;

enum SoldierState {
    Fighthing,
    TakingCover,
    Idle,
    Healing,
}

pub struct Soldier {
    position: Vector2,
    health: Health,
    gun: Weapon,
    state: SoldierState,
}

#[derive(Clone)]
pub enum Weapon {
    Gun {
        damage: f32,
        reatch: f32,
        last_shot: Instant,
        fire_interval: Duration,
    },
    Grenade {
        reatch: f32,
        radius: f32,
        damage: f32,
        last_send: Instant,
        fire_interval: Duration,
    },
}
impl Weapon {
    pub(crate) fn damage(&self) -> &f32 {
        match self {
            Weapon::Grenade { damage, .. } | Weapon::Gun { damage, .. } => damage,
        }
    }
    pub(crate) fn reatch(&self) -> &f32 {
        match self {
            Weapon::Grenade { reatch, .. } | Weapon::Gun { reatch, .. } => reatch,
        }
    }
    pub fn can_shoot(&self) -> bool {
        match self {
            Weapon::Grenade {
                fire_interval,
                last_send,
                ..
            } => last_send.elapsed() >= *fire_interval,
            Weapon::Gun {
                last_shot,
                fire_interval,
                ..
            } => last_shot.elapsed() >= *fire_interval,
        }
    }
}

impl Draw for Soldier {
    fn draw(&self, d: &mut RaylibDrawHandle<'_>) {
        const SIZE: i32 = 32;
        const CORNER: i32 = 7;

        let x = self.position.x as i32;
        let y = self.position.y as i32;

        d.draw_rectangle(x, y, SIZE as i32, SIZE as i32, Color::GREEN);

        let dark = Color::DARKGREEN;

        d.draw_rectangle(x, y, CORNER, CORNER, dark);
        d.draw_rectangle(x + SIZE - CORNER, y, CORNER, CORNER, dark);
        d.draw_rectangle(x, y + SIZE - CORNER, CORNER, CORNER, dark);
        d.draw_rectangle(x + SIZE - CORNER, y + SIZE - CORNER, CORNER, CORNER, dark);
    }
}

impl Armed for Soldier {
    fn weapon(&self) -> &Weapon {
        &self.gun
    }
}
impl Soldier {
    fn shoot<T: TakeDamage>(&mut self, target: &mut T) {
        let health = target.health();
        health.current = (health.current - *self.gun.damage()).max(0.0);
    }
}

impl Soldier {
    pub fn new(position: Vector2) -> Self {
        const SOLDIER_HEALTH: f32 = 300.0;
        Self {
            gun: Weapon::Gun {
                damage: 10.0,
                reatch: 100.0,
                last_shot: Instant::now(),
                fire_interval: Duration::from_millis(500),
            },
            position,
            health: Health::new(SOLDIER_HEALTH),
            state: SoldierState::Idle,
        }
    }
}
impl Movable for Soldier {
    fn move_by(&mut self, ammount: Vector2) {
        self.position.x -= ammount.x;
        self.position.y -= ammount.y;
    }
    fn move_to(&mut self, position: Vector2) {
        self.position.x = position.x;
        self.position.y = position.y;
    }
}

impl TakeDamage for Soldier {
    fn health(&mut self) -> &mut Health {
        &mut self.health
    }
    fn position(&self) -> Vector2 {
        self.position
    }
}
