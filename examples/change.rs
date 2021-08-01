//Base
use bevy::prelude::*;
use bevy_config_cam::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title:
                "bevy_config_cam example: change.rs - How to change between specific camera-modes"
                    .to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1058, 0.1058, 0.1058)))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ConfigCam)
        .add_startup_system(setup.system())
        .add_system(update_camera_mode.system())
        .run();
}

struct TargetCube;

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
            .insert(TargetCube)
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

fn update_camera_mode(
    mut cams: ResMut<Cameras>,
    player_t: Query<(&PlayerMove, &Transform)>,
    mut target_t: Query<(&TargetCube, &Transform)>,
) {
    //Check to prevent panic on first loop
    if player_t.iter().count() == 0 {
        return;
    }
    let (_t1, t1) = player_t.single().unwrap();
    let (_, t) = target_t.single_mut().unwrap();

    let t1dist = t.translation.distance(t1.translation);

    //Moving outside of plane
    if t1dist < 3. {
        let _ = cams.set_camera("LookAt");
    } else {
        let _ = cams.set_camera("FollowBehind");
    }
}
