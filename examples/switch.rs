use bevy::{
    prelude::*,
    render::camera::{ActiveCameras, Camera, CameraPlugin},
};

use bevy_config_cam::next_enum;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// Used in queries when you want filter between cameras
#[derive(Clone, Eq, PartialEq, Debug, Hash, EnumIter, Component)]
enum Cameras {
    CubeCam,
    TopDownCam,
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_state(Cameras::CubeCam)
        .add_startup_system(setup)
        .add_system(change_selected_camera)
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

    // cube camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(Cameras::CubeCam);

    // topdown camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            camera: Camera {
                name: Some("Inactive".to_string()),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 10.0, 0.1).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(Cameras::TopDownCam);
}

fn switch_camera(
    mut act_cams: ResMut<ActiveCameras>,
    camera_state: ResMut<State<Cameras>>,
    mut query: Query<(&Cameras, &mut Camera)>,
) {
    act_cams.remove(CameraPlugin::CAMERA_3D);
    for (_, mut b) in query.iter_mut() {
        b.name = Some("Inactive".to_string());
    }
    for (_, mut b) in query
        .iter_mut()
        .filter(|(c, _)| camera_state.current().eq(c))
    {
        b.name = Some(CameraPlugin::CAMERA_3D.to_string());
    }
    act_cams.add(CameraPlugin::CAMERA_3D);
}

#[allow(unused_must_use)]
fn change_selected_camera(
    mut selected_cam: ResMut<State<Cameras>>,
    keys: Res<Input<KeyCode>>,
    act_cams: ResMut<ActiveCameras>,
    query: Query<(&Cameras, &mut Camera)>,
) {
    if keys.just_pressed(KeyCode::E) {
        println!("Camera: {:?}", selected_cam);
        let result = next_enum!(Cameras, selected_cam);
        selected_cam.set(result);

        switch_camera(act_cams, selected_cam, query);
    }
}
