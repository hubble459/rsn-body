use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::Vec3,
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use rand::Rng;

use crate::orbital_entity::OrbitalEntity;

mod orbital_entity;

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const TIME_STEP: f32 = 0.02;
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
    mut body_query: Query<(&OrbitalEntity, &mut Transform)>,
) {
    let bodies = &mut world.0;

    let mut averages = vec![];
    for i in 0..bodies.len() {
        let mut average_acc = Vec2::ZERO;

        for j in 0..bodies.len() {
            // If not self
            if i != j {
                // Calculate suCtiOn power
                let e1 = &bodies[i];
                let e2 = &bodies[j];
                let r_vector = e1.r_vector(e2);
                let r_mag = e1.pos().distance(e2.pos());
                average_acc -= r_vector / r_mag * (e2.mass / 30.0);
            }
        }

        averages.push(average_acc * TIME_STEP as f32);
    }

    let mut index = 0;
    for (_, mut transform) in body_query.iter_mut() {
        let updated_body = &mut bodies[index];
        let avg = averages[index];
        updated_body.vx += avg.x * TIME_STEP;
        updated_body.vy += avg.y * TIME_STEP;

        updated_body.x += updated_body.vx * TIME_STEP;
        updated_body.y += updated_body.vy * TIME_STEP;

        transform.translation.x = updated_body.x;
        transform.translation.y = updated_body.y;
        index += 1;
    }
}

// fn next_frame(mut query: Query<(&mut Transform, &mut OrbitalEntity)>) {
//     let mut averages = vec![];

//     for (i, (_, entity1)) in query.iter().enumerate() {
//         let mut average_gravity = Vec3::ZERO;
//         for (j, (_, entity2)) in query.iter().enumerate() {
//             if i != j {
//                 let r_vector = entity1.position - entity2.position;
//                 let r_mag = r_vector
//                     .to_array()
//                     .into_iter()
//                     .fold(0.0, |sum, a| sum + a * a)
//                     .sqrt();
//                 let acc = -1.0 * BIG_G * entity2.mass / r_mag.powf(2.0);
//                 average_gravity += r_vector / r_mag * acc;
//             }
//         }

//         averages.push(average_gravity * TIME_STEP as f32);
//     }

//     for (avg, (mut transform, mut entity)) in averages.into_iter().zip(query.iter_mut()) {
//         entity.velocity += avg * TIME_STEP as f32;

//         let bonus_velocity: Vec3 = entity.velocity * TIME_STEP as f32;
//         entity.position = (entity.position + bonus_velocity).clamp(
//             glam::Vec3::splat(f32::MIN + 1.0),
//             glam::Vec3::splat(f32::MAX - 1.0),
//         );

//         transform.translation = entity.position / 1e20;

//         // println!("pos: {:#?}", entity.position);
//     }
// }

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
