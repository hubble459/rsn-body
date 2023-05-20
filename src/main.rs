use glam::Vec3;

use crate::orbital_entity::OrbitalEntity;

mod orbital_entity;

fn main() {
    let mut entities = [
        // Sun
        OrbitalEntity::new(Vec3::ZERO, Vec3::ZERO, 1.989e30),
        // Mercury
        OrbitalEntity::new_from_params(57.909e9, 0.0, 0.0, 0.0, 47.36e3, 0.0, 0.33011e24),
        // Venus
        OrbitalEntity::new_from_params(108.209e9, 0.0, 0.0, 0.0, 35.02e3, 0.0, 4.8675e24),
        // Earth
        // Mars
        // Jupiter
        // Saturn
        // Uranus
        // Neptune
    ];

    let dt: usize = 86400;
    let t_end: usize = dt * 365 * 10; // approximately a decade in seconds
    const BIG_G: f32 = 6.67e-11;

    for _t in (0..t_end).step_by(dt) {
        for i in 0..entities.len() {
            let mut average_gravity = Vec3::ZERO;
            let entity1 = &entities[i];
            for (j, entity2) in entities.iter().enumerate() {
                if i != j {
                    let pos_delta = entity1.position - entity2.position;
                    let r_magnitude = pos_delta.exp().to_array().into_iter().sum::<f32>().sqrt();
                    let acc = -1.0 * BIG_G * entity2.mass / r_magnitude.exp2();
                    let r_unit_vector = pos_delta / r_magnitude;
                    average_gravity += r_unit_vector * acc;
                }
            }

            entities[i].velocity += average_gravity * dt as f32;
        }

        for entity in entities.iter_mut() {
            entity.position += entity.velocity * dt as f32;
        }
    }
}
