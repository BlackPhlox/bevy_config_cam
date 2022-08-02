use std::f32::consts::PI;

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_config_cam::*;
use bevy_dolly::prelude::Rig;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConfigCam)
        .add_startup_system(setup)
        .add_system(rotator_system)
        .add_system(add_target_system)
        .add_system(remove_target_system)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // cube

    commands
        .spawn_bundle(SpatialBundle::from_transform(Transform {
            rotation: Quat::IDENTITY,
            translation: Vec3::new(2.,0.,0.),
            ..default()
        }))
        .with_children(|cell| {
            cell.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..Default::default()
            });
        })
        .insert(Rotates).insert(Target);

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    commands.spawn_bundle(Camera3dBundle {
        camera: Camera {
            is_active: true,
            ..Default::default()
        },
        projection: OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(1.0),
            ..default()
        }
        .into(),
        ..Default::default()
    });
}


#[derive(Component)]
struct Rotates;

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform = Transform::from_rotation(Quat::from_rotation_y(
            (4.0 * PI / 20.0) * time.delta_seconds(),
        )) * *transform;
    }
}

fn remove_target_system(
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    q: Query<Entity, (With<Rotates>, With<Target>)>
) {
    if keys.just_pressed(KeyCode::G) {
        for e in &q {
            commands
                .entity(e)
                .remove::<Target>();
        }
    }
}

fn add_target_system(
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    q: Query<Entity, (With<Rotates>, Without<Target>)>
) {
    if keys.just_pressed(KeyCode::G) {
        for e in &q {
            commands
                .entity(e)
                .insert(Target);
        }
    }
}
