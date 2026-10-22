#[derive(Clone, Debug)]
pub struct Health {
    pub current: f32,
    max: f32,
}

impl Health {
    pub(crate) fn new(max: f32) -> Self {
        Self { max, current: max }
    }
}
