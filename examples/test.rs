use bevy::{
    prelude::*,
    render::camera::{ActiveCameras, Camera},
};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// Used in queries when you want filter between cameras
#[derive(Clone, Eq, PartialEq, Debug, Hash, EnumIter)]
enum Cameras {
    CubeCam,
    TopDownCam,
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_state(Cameras::CubeCam)
        .add_startup_system(setup.system())
        .add_system(change_selected_camera.system())
        .add_system(change_detection.system())
        //.add_system(switch_camera.system())
        .add_system(debug_stats_change.system())
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
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // cube camera
    let cube_cam = PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };

    commands.spawn_bundle(cube_cam).insert(Cameras::CubeCam);

    // topdown camera
    let topdown_cam = PerspectiveCameraBundle {
        camera: Camera {
            name: Some("Inactive".to_string()),
            ..Default::default()
        },
        transform: Transform::from_xyz(-2.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };

    commands
        .spawn_bundle(topdown_cam)
        .insert(Cameras::TopDownCam);
}

fn debug_stats_change(
    query: Query<
        // components
        (&Cameras, &Camera),
        // filters
        Changed<State<Cameras>>,
    >,
) {
    for (cams, cam) in query.iter() {
        println!("Cam Enum: {:?}, Cam Name: {:?}", cams, cam.name);
    }
}

fn change_detection(query: Query<(Entity, &Cameras), Changed<Cameras>>) {
    for (entity, component) in query.iter() {
        info!("{:?} changed: {:?}", entity, component,);
    }
}

fn switch_camera(
    mut act_cams: ResMut<ActiveCameras>,
    camera_state: Res<State<Cameras>>,
    mut query: Query<(&Cameras, &mut Camera), Changed<State<Cameras>>>,
) {
    act_cams.remove("Camera3d");
    for (c, mut b) in query.iter_mut() {
        println!("{:?}", camera_state.current());
        println!("{:?}", c);
        println!("{:?}", "Setting to test");
        println!("{:?}", b);

        b.name = Some("Inactive".to_string());
    }
    for (_, mut b) in query
        .iter_mut()
        .filter(|(c, _)| camera_state.current().eq(c))
    {
        println!("{:?}", "Setting to main");
        println!("{:?}", b);
        b.name = Some("Camera3d".to_string());
    }
    act_cams.add("Camera3d");
}

#[macro_export]
macro_rules! next_enum {
    ($l:ident, $k:expr) => {
        $l::iter()
            .enumerate()
            .nth(
                $l::iter()
                    .enumerate()
                    .find(|a| a.1 == *$k.current())
                    .map(|(i, _)| {
                        if i + 1 > $l::iter().count() - 1 {
                            0usize
                        } else {
                            i + 1
                        }
                    })
                    .unwrap(),
            )
            .unwrap()
            .1
    };
}

#[allow(unused_must_use)]
fn change_selected_camera(mut selected_cam: ResMut<State<Cameras>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::E) {
        let result = next_enum!(Cameras, selected_cam);
        println!("Camera: {:?}", result);
        selected_cam.set(result);
    }
}
