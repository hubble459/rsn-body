use std::ops::Neg;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::Vec3,
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use rand::Rng;
use rayon::prelude::*;

use crate::orbital_entity::OrbitalEntity;

mod orbital_entity;

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const TIME_STEP: f32 = 0.002;
const NUM_BODIES: usize = 1000;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const SCALE: f32 = 10.0;

fn add_components(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    world: Res<World>,
) {
    let mut rng = rand::thread_rng();
    // Camera
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: SCALE,
            ..Default::default()
        },
        ..Default::default()
    });

    for (index, entity) in world.0.iter().enumerate() {
        commands
            .spawn(MaterialMesh2dBundle {
                material: materials.add(ColorMaterial::from(Color::Rgba {
                    red: rng.gen_range(0.8..1.),
                    green: rng.gen_range(0.0..0.6),
                    blue: rng.gen_range(0.0..0.05),
                    alpha: 1.0,
                })),
                transform: Transform {
                    translation: Vec3::new(entity.x, entity.y, index as f32),
                    ..Default::default()
                },
                mesh: meshes
                    .add(shape::Circle::new(entity.mass / 30.).into())
                    .into(),
                visibility: Visibility::Visible,
                ..Default::default()
            })
            .insert(entity.clone());
    }
}

fn update_bodies(
    mut world: ResMut<World>,
    mut body_query: Query<&mut Transform, With<OrbitalEntity>>,
) {
    let bodies = &mut world.0;
    let bodies_readonly = bodies.clone();

    bodies.par_iter_mut().enumerate().for_each(|(i, e1)| {
        let avg = bodies_readonly
            .par_iter()
            .enumerate()
            .map(|(j, e2)| {
                // If not self
                if i != j {
                    // Calculate suCtiOn power
                    let r_vector = e1.r_vector(e2);
                    let r_mag = e1.pos().distance(e2.pos());
                    r_vector / r_mag * e2.mass
                } else {
                    Vec2::ZERO
                }
            })
            .sum::<Vec2>()
            .neg()
            * TIME_STEP;

        e1.vx += avg.x;
        e1.vy += avg.y;
        e1.x += e1.vx * TIME_STEP;
        e1.y += e1.vy * TIME_STEP;
    });

    for (i, mut transform) in body_query.iter_mut().enumerate() {
        let entity = &bodies[i];

        transform.translation.x = entity.x;
        transform.translation.y = entity.y;
    }
}

#[derive(Resource)]
struct World(Vec<OrbitalEntity>);

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy-nbody".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(World(initialize_bodies()))
        .add_startup_system(add_components)
        .add_system(update_bodies)
        .run();
}

fn initialize_bodies() -> Vec<OrbitalEntity> {
    let mut rng = rand::thread_rng();
    let mut bodies = vec![];
    for i in 0..NUM_BODIES {
        let mass = if i == 0 {
            rng.gen_range(500.0..1500.0)
        } else {
            rng.gen_range(50.0..500.0)
        };
        bodies.push(OrbitalEntity {
            x: rng.gen_range(
                (-(WIDTH as f32 / 2.) * SCALE as f32)..((WIDTH as f32 / 2.) * SCALE as f32),
            ),
            y: rng.gen_range(
                (-(HEIGHT as f32 / 2.) * SCALE as f32)..((HEIGHT as f32 / 2.) * SCALE as f32),
            ),
            vx: rng.gen_range(-50.0..50.0),
            vy: rng.gen_range(-50.0..50.0),
            mass,
        });
    }
    bodies
}
