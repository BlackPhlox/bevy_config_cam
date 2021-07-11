//Base
use bevy::prelude::*;
use bevy_config_cam::*;
use bevy_config_cam::{cam::MovementSettings, player::PlayerSettings};

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.1058, 0.1058, 0.1058)))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ConfigCam)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0,          // default: 12.0
            ..Default::default()
        })
        .insert_resource(PlayerSettings {
            pos: Vec3::new(2., 0., 0.),
            player_asset: "models/craft_speederA.glb#Scene0",
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut cl: ResMut<Config>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // cube, set as target
    cl.external_target = Some(
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..Default::default()
            })
            .id(),
    );

    cl.target = Some(
        commands
            .spawn_bundle((
                Transform {
                    translation: Vec3::new(0., 0., 0.),
                    rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
                    ..Default::default()
                },
                GlobalTransform::identity(),
            ))
            .with_children(|cell| {
                cell.spawn_scene(asset_server.load("models/craft_speederA.glb#Scene0"));
            })
            .id(),
    );

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}
