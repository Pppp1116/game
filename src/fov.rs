use raylib::math::Vector2;

#[derive(Clone, Debug)]
pub struct Fov {
    pub distance: f32,
    angle: f32,
    facing: Vector2,
}

impl Fov {
    pub(crate) fn new(distance: f32, angle: f32) -> Self {
        let facing_angle = rand::random::<f32>() * std::f32::consts::TAU;
        let facing = Vector2::new(facing_angle.cos(), facing_angle.sin());
        Self {
            distance,
            angle,
            facing,
        }
    }
    pub(crate) fn is_in_fov(&self, my_pos: &Vector2, target: &Vector2) -> bool {
        let delta = *target - *my_pos;
        let distance_sq = delta.dot(delta);

        if distance_sq > self.distance * self.distance {
            return false;
        }

        if distance_sq == 0.0 {
            return true;
        }
        let half_angle_cos = (self.angle / 2.0).to_radians().cos();

        self.facing.dot(delta) >= half_angle_cos * distance_sq.sqrt()
    }
}
