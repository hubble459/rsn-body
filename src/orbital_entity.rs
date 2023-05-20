use glam::Vec3;

#[derive(Clone)]
pub struct OrbitalEntity {
    pub position: Vec3,
    pub velocity: Vec3,
    pub mass: f32,
}

impl OrbitalEntity {
    pub fn new(position: Vec3, velocity: Vec3, mass: f32) -> Self {
        Self {
            position,
            velocity,
            mass,
        }
    }

    pub fn new_from_params(
        p1: f32,
        p2: f32,
        p3: f32,
        v1: f32,
        v2: f32,
        v3: f32,
        mass: f32,
    ) -> Self {
        Self {
            position: Vec3::new(p1, p2, p3),
            velocity: Vec3::new(v1, v2, v3),
            mass,
        }
    }
}
