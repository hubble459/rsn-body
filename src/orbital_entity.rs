use bevy::prelude::*;

#[derive(Clone, Component, PartialEq)]
pub struct OrbitalEntity {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub mass: f32,
}

impl OrbitalEntity {
    pub fn r_vector(&self, other: &Self) -> Vec2 {
        Vec2 { x: self.x - other.x, y: self.y - other.y }
    }

    pub fn pos(&self) -> Vec2 {
        Vec2 { x: self.x, y: self.y }
    }
}
