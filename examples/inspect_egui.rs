//Base
use bevy::prelude::*;
use bevy_config_cam::*;
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "bevy_config_cam example: inspect_egui.rs - Using bevy_inspector_egui to show the structure of the scene and config cam using world inspector".to_string(),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ConfigCam)
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(WorldInspectorParams {
            despawnable_entities: true,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_system(update_camera_mode.system())
        .run();
}

struct TargetCube;

struct PlayerCube2;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
            .insert(TargetCube)
            .id(),
    );

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.4, 0.4, 0.7).into()),
            transform: Transform::from_xyz(2.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(PlayerCube2)
        .id();
}

fn update_camera_mode(
    commands: Commands,
    mut config: ResMut<Config>,
    player_t: Query<(&PlayerMove, &Transform, Entity)>,
    mut target_t: Query<(&TargetCube, &Transform)>,
    mut player_cube2: Query<(&PlayerCube2, Entity)>,
) {
    //Check to prevent panic on first loop
    if player_t.iter().count() == 0 {
        return;
    }
    let (_t1, t1, e) = player_t.single().unwrap();
    let (_, t) = target_t.single_mut().unwrap();

    let (_, pc2e) = player_cube2.single_mut().unwrap();

    let t1dist = t.translation.distance(t1.translation);

    //Moving outside of plane
    if t1dist < 3. {
        //let _ = cams.set_camera("LookAt");
        let _ = config.set_player_target(e, commands);
    } else {
        //let _ = cams.set_camera("FollowBehind");
        let _ = config.set_player_target(pc2e, commands);
    }
}
