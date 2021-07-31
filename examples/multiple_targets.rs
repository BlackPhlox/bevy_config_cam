//Base
use bevy::prelude::*;
use bevy_config_cam::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "bevy_config_cam example: multiple_targets.rs - Automatically focus on entities when they are in range".to_string(),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ConfigCam)
        .add_startup_system(setup.system())
        .add_system(set_closest_target.system())
        .run();
}

struct T1;
struct T2;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut config: ResMut<Config>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 11.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    //Player
    config.target = Some(
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

    //Target 1
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(-5.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(T1);

    //Target 2
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(5.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(T2);

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

fn set_closest_target(
    mut config: ResMut<Config>,
    mut transforms: Query<(&PlayerMove, &Transform)>,
    query: QuerySet<(
        Query<(&T1, Entity, &Transform)>,
        Query<(&T2, Entity, &Transform)>,
    )>,
) {
    //Check to prevent panic on first loop
    if transforms.iter().count() == 0 {
        return;
    }
    let (_t1, e1, t1) = query.q0().single().unwrap();
    let (_t2, e2, t2) = query.q1().single().unwrap();
    let (_, t) = transforms.single_mut().unwrap();

    let t1dist = t.translation.distance(t1.translation);
    let t2dist = t.translation.distance(t2.translation);

    if t1dist < t2dist && t1dist < 5. {
        config.external_target = Some(e1);
    } else if t1dist > t2dist && t2dist < 5. {
        config.external_target = Some(e2);
    } else {
        config.external_target = None;
    }
}
