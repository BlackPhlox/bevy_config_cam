//Base
use bevy::prelude::*;
use bevy_config_cam::*;
use bevy_inspector_egui::widgets::ResourceInspector;
use bevy_inspector_egui::{
    Inspectable, InspectableRegistry, InspectorPlugin, WorldInspectorParams, WorldInspectorPlugin,
};

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ConfigCam)
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(WorldInspectorParams {
            despawnable_entities: true,
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

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}
