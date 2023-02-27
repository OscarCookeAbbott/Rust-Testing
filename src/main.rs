mod vector;

use std::{f32::consts::PI, ops::Range};

use bevy::{pbr::DirectionalLightShadowMap, prelude::*, window::PresentMode};

#[derive(Component)]
struct Object {
    origin: Vec3,
}

#[derive(Deref, Component, Debug)]
struct Name(String);

#[derive(Resource, Default)]
struct Profiling {
    total_frames: u64,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                present_mode: PresentMode::AutoNoVsync,
                ..Default::default()
            },
            ..Default::default()
        }))
        .insert_resource(DirectionalLightShadowMap { size: 16384 })
        .insert_resource(Profiling::default())
        .add_startup_system(spawn_entities)
        .add_system(move_input)
        .add_system(move_entities)
        .add_system(log_performance)
        .run();
}

fn spawn_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = StandardMaterial { ..default() };

    commands.spawn((
        Name(String::from("Testicles")),
        PbrBundle {
            mesh: meshes.add(shape::Icosphere::default().into()),
            material: materials.add(debug_material.clone().into()),
            ..default()
        },
        Object {
            origin: Vec3::new(0.0, 1.0, -2.0),
        },
    ));
    commands.spawn((
        Name(String::from("Giblets")),
        PbrBundle {
            mesh: meshes.add(shape::Box::default().into()),
            material: materials.add(debug_material.clone().into()),
            transform: Transform::from_translation(Vec3::new(3.0, 1.0, 0.0))
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
            ..default()
        },
    ));
    commands.spawn((
        Name(String::from("Rockets")),
        PbrBundle {
            mesh: meshes.add(shape::Torus::default().into()),
            material: materials.add(debug_material.clone().into()),
            transform: Transform::from_translation(Vec3::new(-3.0, 1.0, 0.0))
                .with_rotation(Quat::from_rotation_x(PI / 4.)),
            ..default()
        },
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 8.0, 16.0, 8.0)),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane { size: 50. }.into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., -12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });
}

fn move_entities(time: Res<Time>, mut query: Query<(&Object, &mut Transform)>) {
    for (object, mut transform) in &mut query {
        transform.translation = object.origin + Vec3::Y * 4.0 * time.elapsed_seconds().sin().abs();
    }
}

fn move_input(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Object>,
) {
    for mut object in &mut query {
        let mut hor_input: Vec3 = default();

        if keys.pressed(KeyCode::D) {
            hor_input.x -= 1.0;
        }

        if keys.pressed(KeyCode::A) {
            hor_input.x += 1.0;
        }

        if keys.pressed(KeyCode::W) {
            hor_input.z += 1.0;
        }

        if keys.pressed(KeyCode::S) {
            hor_input.z -= 1.0;
        }

        object.origin += 3.0 * hor_input.normalize_or_zero() * time.delta_seconds();
    }
}

fn log_performance(time: Res<Time>, mut profiling: ResMut<Profiling>) {
    profiling.total_frames += 1;

    let average_framerate = (profiling.total_frames as f64 / time.elapsed_seconds_f64()) as u64;

    println!("{average_framerate}");
}
