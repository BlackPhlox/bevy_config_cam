//Base
use bevy::prelude::*;
use bevy_config_cam::*;
use bevy_mod_picking::{
    DebugCursorPickingPlugin, DebugEventsPickingPlugin, HighlightablePickingPlugin,
    InteractablePickingPlugin, PickableBundle, PickingCameraBundle, PickingEvent, PickingPlugin,
};

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.1058, 0.1058, 0.1058)))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(HighlightablePickingPlugin)
        .add_plugin(DebugCursorPickingPlugin)
        .add_plugin(DebugEventsPickingPlugin)
        .add_system_to_stage(CoreStage::PostUpdate, print_events.system())
        .add_plugin(ConfigCam)
        .add_startup_system(setup.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut config: ResMut<Config>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // cube, set as selectable external target when clicked
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default());

    config.camera_settings.camera = Some(
        commands
            .spawn_bundle(PerspectiveCameraBundle {
                transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            })
            .insert_bundle(PickingCameraBundle::default())
            .id(),
    );

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

pub fn print_events(mut events: EventReader<PickingEvent>, mut config: ResMut<Config>) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(se) => match se {
                bevy_mod_picking::SelectionEvent::JustSelected(e) => {
                    config.external_target = Some(*e);
                    println!("Clicked cube, focusing! {:?}", se);
                }
                bevy_mod_picking::SelectionEvent::JustDeselected(_) => {
                    config.external_target = None;
                    println!("Unclicked cube, defocusing! {:?}", se);
                }
            },
            PickingEvent::Hover(_) => (),
        }
    }
}
