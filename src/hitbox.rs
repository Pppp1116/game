use raylib::ffi::Vector2;

#[derive(Clone, Debug)]
pub struct HitBox {
    pub position: Vector2,
    pub velocity: Vector2,
    pub size: Vector2,
}

impl HitBox {
    pub fn intersects(&self, previous_position: Vector2, other: &Self) -> bool {
        // Treat the projectile as a point by expanding the target.
        let min = other.position - self.size;
        let max = other.position + other.size;

        let delta = self.position - previous_position;

        let mut t_enter = 0.0_f32;
        let mut t_exit = 1.0_f32;

        for (start, direction, min, max) in [
            (previous_position.x, delta.x, min.x, max.x),
            (previous_position.y, delta.y, min.y, max.y),
        ] {
            if direction == 0.0 {
                // No movement on this axis.
                if start < min || start > max {
                    return false;
                }
                continue;
            }

            let t1 = (min - start) / direction;
            let t2 = (max - start) / direction;

            let axis_enter = t1.min(t2);
            let axis_exit = t1.max(t2);

            t_enter = t_enter.max(axis_enter);
            t_exit = t_exit.min(axis_exit);

            if t_enter > t_exit {
                return false;
            }
        }

        true
    }

    pub fn new(position: Vector2, size: Vector2) -> Self {
        HitBox {
            position: position,
            velocity: Vector2::new(0.0, 0.0),
            size: size,
        }
    }
}
