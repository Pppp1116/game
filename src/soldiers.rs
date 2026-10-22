use crate::hitbox::HitBox;
use crate::world::{Collection, World};
use crate::{
    health::Health,
    traits::{Armed, Draw, Movable, TakeDamage, WorldObject},
};
use raylib::prelude::*;
use std::time::{Duration, Instant};

enum SoldierState {
    Fighthing,
    TakingCover,
    Idle,
    Healing,
}

pub struct Soldier {
    hitbox: HitBox,
    health: Health,
    gun: Weapon,
    state: SoldierState,
}
impl WorldObject for Soldier {
    fn storage(world: &mut World) -> &mut Collection<Self> {
        &mut world.soldiers
    }
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
        last_shot: Instant,
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
                last_shot: last_send,
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

        let x = self.hitbox.position.x as i32;
        let y = self.hitbox.position.y as i32;

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
    fn mut_weapon(&mut self) -> &mut Weapon {
        &mut self.gun
    }
}

impl Soldier {
    pub fn new(hitbox: HitBox) -> Self {
        const SOLDIER_HEALTH: f32 = 300.0;
        Self {
            gun: Weapon::Gun {
                damage: 10.0,
                reatch: 100.0,
                last_shot: Instant::now(),
                fire_interval: Duration::from_millis(500),
            },
            hitbox,
            health: Health::new(SOLDIER_HEALTH),
            state: SoldierState::Idle,
        }
    }
}
impl Movable for Soldier {
    fn move_by(&mut self, ammount: Vector2) {
        self.hitbox.position.x -= ammount.x;
        self.hitbox.position.y -= ammount.y;
    }
    fn velocity(&self) -> &Vector2 {
        &self.hitbox.velocity
    }
}

impl TakeDamage for Soldier {
    fn health(&mut self) -> &mut Health {
        &mut self.health
    }
    fn hitbox(&self) -> &HitBox {
        &self.hitbox
    }
    fn take_damage(&mut self, damage: f32) {
        self.health.current -= damage;
    }
}
