extern crate bevy_config_cam;

//Base
use bevy::prelude::*;
use bevy_config_cam::*;

fn main() {
    App::build()
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
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 11.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

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
    mut cl: ResMut<CamLogic>,
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
        cl.target = Some(e1);
    } else if t1dist > t2dist && t2dist < 5. {
        cl.target = Some(e2);
    } else {
        cl.target = None;
    }
}
