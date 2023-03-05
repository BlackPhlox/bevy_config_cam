use std::f32::consts::PI;

use bevy::{
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
};
use bevy_config_cam::*;
use bevy_config_cam::{driver::driver_core::DriverMarker, drivers::fpv::CCFpv};
use bevy_dolly::prelude::{Fpv, Rig};
use config_cam_derive::DriverMarker;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_plugin(ConfigCam)
        .add_startup_system(setup)
        .add_system(rotator_system)
        .add_system(add_target_system)
        .add_system(remove_target_system)
        .add_system(switch_camera)
        .run();
}

#[derive(DriverMarker, Component, Clone, Copy, Debug)]
pub struct Pinned2;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // cube

    commands
        .spawn(SpatialBundle::from_transform(Transform {
            rotation: Quat::IDENTITY,
            translation: Vec3::new(2., 0., 0.),
            ..default()
        }))
        .with_children(|cell| {
            cell.spawn((
                Rotates,
                Target,
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    transform: Transform::from_xyz(0.0, 0.5, 0.0),
                    ..Default::default()
                },
            ));
        });

    commands
        .spawn(SpatialBundle::from_transform(Transform {
            rotation: Quat::IDENTITY,
            translation: Vec3::new(-2., 0., 0.),
            ..default()
        }))
        .with_children(|cell| {
            cell.spawn((
                Target,
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    transform: Transform::from_xyz(0.0, 0.5, 0.0),
                    ..Default::default()
                },
            ));
        });

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    let cam: Handle<Mesh> = asset_server.load("models/cam.gltf#Mesh0/Primitive0");

    /*commands.spawn_bundle(SceneBundle {
        scene: asset_server.load("models/cam.gltf#Scene0"),
        ..default()
    }).insert(Wireframe);*/

    commands
        .spawn(PbrBundle {
            mesh: cam.clone(),
            material: materials
                .add(
                    Color::Rgba {
                        red: 0.,
                        green: 0.,
                        blue: 0.,
                        alpha: 0.,
                    }
                    .into(),
                )
                .clone(),
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.5,
                    z: 0.0,
                },
                scale: Vec3::new(0.5, 0.5, 0.5),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Wireframe);

    /*
        commands.spawn((
            Camera3dBundle {
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
        },
        Rig::builder()
        .with(Fpv::from_position_target(Transform::default()))
        .build(),
        CCFpv,
    ));

    commands.spawn(Camera3dBundle {
        camera: Camera {
            is_active: false,
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
    */

    commands.spawn(CameraCount { total: 2, index: 0 });
}

#[derive(Component)]
struct CameraCount {
    total: u16,
    index: usize,
}

#[derive(Component)]
struct Rotates;

#[derive(Component)]
struct Selected;

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
    q: Query<Entity, (With<Selected>, Without<Target>)>,
) {
    if keys.just_pressed(KeyCode::G) {
        for e in &q {
            commands.entity(e).remove::<Target>();
            commands.entity(e).remove::<Rotates>();
            println!("Removed Target");
        }
    }
}

fn add_target_system(
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    q: Query<Entity, (Without<Selected>, With<Target>)>,
) {
    if keys.just_pressed(KeyCode::H) {
        for e in &q {
            commands.entity(e).insert(Target);
            commands.entity(e).insert(Rotates);
            println!("Added Target");
        }
    }
}

fn switch_camera(
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut q: Query<&Camera>,
    mut q2: Query<&CameraCount>,
) {
    let mut cc = q2.single_mut();
    if keys.just_pressed(KeyCode::V) {
        for (i, c) in &mut q.iter_mut().enumerate() {
            if i + 1 > (cc.total - 1).into() {
                //cc.index = 0;
                //c.is_active = true;
            }
            if i.eq(&(cc.index + 1 as usize)) {}
        }
    }
}
