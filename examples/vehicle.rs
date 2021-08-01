//Base
use bevy::prelude::*;
use bevy_config_cam::default_cam_modes::FollowBehind;
use bevy_config_cam::{
    Cameras, ChangeTarget, Config, ConfigCam, PlayerMove,
};

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "bevy_config_cam example: vehicle.rs - Changing camera-mode based on a custom player state".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1058, 0.1058, 0.1058)))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ConfigCam)
        .add_state(PlayerState::OnFoot)
        .add_startup_system(setup.system())
        .add_system(set_closest_target.system())
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum PlayerState {
    OnFoot,
    InVehicle,
}

struct SpaceCraft;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut cl: ResMut<Config>,
    mut cams: ResMut<Cameras>,
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

    //Spawn player
    cl.target = Some(
        commands
            .spawn_bundle((
                Transform {
                    translation: Vec3::new(-1.5, 0., 0.),
                    rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
                    ..Default::default()
                },
                GlobalTransform::identity(),
            ))
            .with_children(|cell| {
                cell.spawn_scene(asset_server.load("models/alien.glb#Scene0"));
            })
            .id(),
    );

    // Spawn spacecraft
    commands
        .spawn_bundle((
            Transform {
                translation: Vec3::new(1.5, 0., 1.5),
                rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
                ..Default::default()
            },
            GlobalTransform::identity(),
        ))
        .with_children(|cell| {
            cell.spawn_scene(asset_server.load("models/craft_speederA.glb#Scene0"));
        })
        .insert(SpaceCraft);

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    //Only allow FollowBehind camera-mode
    cams.camera_modes = vec![Box::new(FollowBehind)];
}

//https://github.com/bevyengine/bevy/issues/838
fn set_visible_recursive(
    is_visible: bool,
    entity: Entity,
    visible_query: &mut Query<&mut Visible>,
    children_query: &Query<&Children>,
) {
    if let Ok(mut visible) = visible_query.get_mut(entity) {
        visible.is_visible = is_visible;
    }

    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            set_visible_recursive(is_visible, *child, visible_query, children_query);
        }
    }
}

fn set_closest_target(
    commands: Commands,
    mut config: ResMut<Config>,
    mut transforms: Query<(&PlayerMove, Entity, &Transform)>,
    keys: Res<Input<KeyCode>>,
    query: Query<(&SpaceCraft, Entity, &Transform)>,
    mut visible_query: Query<&mut Visible>,
    children_query: Query<&Children>,
    mut p_state: ResMut<State<PlayerState>>,
) {
    //Check to prevent panic on first loop
    if transforms.iter().count() == 0 {
        return;
    }
    let (_t1, e1, t1) = query.single().unwrap();
    let (_, e, t) = transforms.single_mut().unwrap();

    let t1dist = t.translation.distance(t1.translation);

    if t1dist < 2. {
        //Look at player and spacecraft
        config.target = Some(e);
        config.external_target = Some(e1);
        if keys.just_pressed(KeyCode::F) {
            if p_state.current() == &PlayerState::OnFoot {
                set_visible_recursive(false, e, &mut visible_query, &children_query);
                println!("Make Invisible");
                //commands.entity(e).remove::<PlayerMove>();
                let _ = config.set_player_target(e1, commands);
                //let _ = config.set_player_controller(e1, commands);
                let _ = p_state.replace(PlayerState::InVehicle);
            } else {
                set_visible_recursive(true, e, &mut visible_query, &children_query);
                println!("Make visible");
                let _ = config.set_player_target(e, commands);
                //let _ = config.set_player_controller(e1, commands);
                let _ = p_state.replace(PlayerState::OnFoot);
            }
        }
    } else {
        //Look only at player. TODO: Work with external_targets
        config.target = Some(e);
        config.external_target = None;
    }
}
