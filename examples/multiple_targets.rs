use bevy::prelude::*;
use bevy_config_cam::{driver::driver_core::DriverMarker, *};
use bevy_dolly::prelude::*;
use config_cam_derive::DriverMarker;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ConfigCam)
        .add_rig_component(T1)
        .add_dolly_component(MainCamera)
        /*.insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0,          // default: 12.0
            ..Default::default()
        })*/
        /*.insert_resource(PlayerSettings {
            pos: Vec3::new(2., 0., 0.),
            player_asset: "models/craft_speederA.glb#Scene0",
            ..Default::default()
        })*/
        .add_startup_system(setup)
        //.add_system(set_closest_target)
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(DriverMarker, Component, Clone, Copy, Debug)]
struct T1;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 11.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    //Target 1
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(-5.0, 0.5, 0.0),
            ..Default::default()
        },
        Target,
    ));

    //Target 2
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(5.0, 0.5, 0.0),
            ..Default::default()
        },
        Target,
    ));

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

/*
fn set_closest_target(
    mut cl: ResMut<CamLogic>,
    mut transforms: Query<(&PlayerMove, &Transform)>,
    mut query: ParamSet<(
        Query<(&T1, Entity, &Transform)>,
        Query<(&T2, Entity, &Transform)>,
    )>,
) {
    //Check to prevent panic on first loop
    if transforms.iter().count() == 0 {
        return;
    }

    let mut p0 = query.p0();
    let (_t1, e1, t1) = p0.single_mut();
    let (_, t) = transforms.single_mut();
    let t1dist = t.translation.distance(t1.translation);

    let mut p1 = query.p1();
    let (_t2, e2, t2) = p1.single_mut();
    let t2dist = t.translation.distance(t2.translation);

    if t1dist < t2dist && t1dist < 5. {
        cl.target = Some(e1);
    } else if t1dist > t2dist && t2dist < 5. {
        cl.target = Some(e2);
    } else {
        cl.target = None;
    }
}
*/
