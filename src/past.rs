use std::time::Instant;

use raylib::ffi::Vector2;

pub struct Enemy {
    timeout: Instant,
    position: Vector2,
}

pub struct Memory {
    seen: Vec<Enemy>,
}
