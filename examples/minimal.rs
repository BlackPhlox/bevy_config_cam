//Base
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_config_cam::*;
use bevy_dolly::{dolly::glam, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConfigCam)
        .add_startup_system(setup)
        .add_system(update_yaw_driver)
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
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    commands.spawn().insert(
        Rig::builder()
            .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
            .with(Smooth::new_rotation(1.5))
            .with(Arm::new(glam::Vec3::Z * 4.0))
            .build(),
    );

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 0.01).looking_at(Vec3::ZERO, Vec3::Y),
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

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(5., 2.0, 0.).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
            is_active: false,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn update_yaw_driver(keys: Res<Input<KeyCode>>, mut query: Query<&mut Rig>) {
    for mut rig in &mut query {
        if let Some(camera_driver) = rig.try_driver_mut::<YawPitch>() {
            if keys.just_pressed(KeyCode::Z) {
                camera_driver.rotate_yaw_pitch(-90.0, 0.0);
            }
            if keys.just_pressed(KeyCode::X) {
                camera_driver.rotate_yaw_pitch(90.0, 0.0);
            }
        }
    }
}
