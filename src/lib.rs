//Base
use bevy::{
    core::FixedTimestep,
    ecs::schedule::SystemSet,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
    window::{CursorMoved, Windows},
    render::{camera::Camera, camera::CameraProjection, camera::PerspectiveProjection, render_graph::base::camera::CAMERA_3D},
};

use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin, KeyMap};
pub struct PlayerMove;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum PluginState {
    Enabled,
    Disabled,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum ScrollType {
    MovementSpeed,
    Zoom,
    Sensitivity,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum CameraState {
    LookAt,
    FollowStatic,
    FollowBehind,
    FPS,
    Free,
}

pub struct MultiCamSettings {
    pub sensitivity: f32,
    pub cam_speed: f32,
    pub player_speed: f32,
    pub map: KeyMap,
    pub disable_move: bool,
    pub disable_look: bool,
}

pub struct MultiCam;
impl Plugin for MultiCam {
    fn build(&self, app: &mut AppBuilder) {
        app//.add_plugins(DefaultPlugins)
        .init_resource::<CamLogic>()
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0, // default: 12.0
            ..Default::default()
        })
        .add_state(PluginState::Enabled)
        .add_state(CameraState::LookAt)
        .add_state(ScrollType::MovementSpeed)

        .add_system(switch_scroll_type.system())
        .add_system(scroll.system())

        .add_system(cycle_cam_state.system())

        .add_system_set(SystemSet::on_enter(PluginState::Enabled).with_system(setup.system()))
        .add_system_set(
            SystemSet::on_update(PluginState::Enabled)
                .with_system(move_player.system())
                .with_system(focus_camera.system()),
        );
    }
}

#[derive(Default)]
struct Player {
    entity: Option<Entity>,
    i: usize,
    j: usize,
}

#[derive(Default)]
struct CamLogic {
    player: Player,
    camera_should_focus: Vec3,
    camera_is_focus: Vec3,
    target: Option<Entity>,
}

const RESET_FOCUS: [f32; 3] = [
    0.,
    0.,
    0.,
];

#[allow(unused_must_use)]
fn cycle_cam_state(mut cam_state: ResMut<State<CameraState>>, keyboard_input: Res<Input<KeyCode>>){
    if keyboard_input.just_pressed(KeyCode::E){
        let result = match cam_state.current() {
            CameraState::LookAt => CameraState::FollowStatic,
            CameraState::FollowStatic => CameraState::FollowBehind,
            CameraState::FollowBehind => CameraState::FPS,
            CameraState::FPS => CameraState::Free,
            CameraState::Free => CameraState::LookAt,
        };

        println!("Camera: {:?}", result);
        cam_state.set(result);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut cl: ResMut<CamLogic>) {
    // reset the cam logic state

    let base_height = 0.2;

    // spawn the cam logic character
    cl.player.entity = Some(
        commands
            .spawn_bundle((
                Transform {
                    translation: Vec3::new(
                        cl.player.i as f32,
                        base_height,
                        cl.player.j as f32,
                    ),
                    rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
                    ..Default::default()
                },
                GlobalTransform::identity(),
            )).insert(PlayerMove)
            .with_children(|cell| {
                cell.spawn_scene(asset_server.load("models/AlienCake/craft_speederA.glb#Scene0"));
            })
            .id(),
    );

    // camera
    let camera = PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };

    // add plugin
    commands.spawn_bundle(camera).insert(FlyCam);
}

// control the cam logic character
fn move_player(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut cl: ResMut<CamLogic>,
    mut transforms: Query<(&PlayerMove, &mut Transform)>,
) {
    for (_player, mut transform) in transforms.iter_mut() {
        let (_ , mut rotation) = transform.rotation.to_axis_angle();
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);

        for key in keys.get_pressed() {
            match key {
                KeyCode::W => velocity += forward,
                KeyCode::S => velocity -= forward,
                KeyCode::A => velocity -= right,
                KeyCode::D => velocity += right,
                KeyCode::Space => velocity += Vec3::Y,
                KeyCode::LShift => velocity -= Vec3::Y,
                KeyCode::Left => {if rotation > std::f32::consts::FRAC_PI_2*4.0-0.05 {rotation = 0.0;} rotation += 0.1 },
                KeyCode::Right => {if rotation < 0.05 {rotation = std::f32::consts::FRAC_PI_2*4.0;} rotation -= 0.1},
                _ => (),
            }
        }

        velocity = velocity.normalize();

        transform.rotation = Quat::from_rotation_y(rotation);

        if !velocity.is_nan() {
            transform.translation += velocity * time.delta_seconds() * 4.0;
        }
    }
}

// change the focus of the camera
fn focus_camera(
    time: Res<Time>,
    mut state: ResMut<State<CameraState>>,
    mut cl: ResMut<CamLogic>,
    mut settings: ResMut<MovementSettings>,
    mut transforms: QuerySet<(Query<(&mut Transform, &Camera)>, Query<&Transform>)>,
) {
    let mut delta_trans = Transform::identity();
    settings.disable_look = true;
    settings.disable_move = false;
    if *state.current() == CameraState::Free {
        settings.disable_look = false;
        return;
    } else if *state.current() == CameraState::FollowStatic || *state.current() == CameraState::FollowBehind || *state.current() == CameraState::FPS  {
        if let Some(player_entity) = cl.player.entity {
            if let Ok(player_transform) = transforms.q1().get(player_entity) {
                if *state.current() == CameraState::FollowBehind || *state.current() == CameraState::FPS {
                    settings.disable_move = true;
                    if *state.current() == CameraState::FPS {settings.disable_look = false;}
                    delta_trans.translation = player_transform.translation;
                    delta_trans.rotation = player_transform.rotation;
                    delta_trans.translation += Vec3::new(/*-4.*/0.,1.,0.);
                }
                cl.camera_should_focus = player_transform.translation;
            }
        // otherwise, target the middle
        } else {
            cl.camera_should_focus = Vec3::from(RESET_FOCUS);
        }
    } else {   
        // if there is both a player and a bonus, target the mid-point of them
        if let (Some(player_entity), Some(bonus_entity)) = (cl.player.entity, cl.target) {
            if let (Ok(player_transform), Ok(bonus_transform)) = (
                transforms.q1().get(player_entity),
                transforms.q1().get(bonus_entity),
            ) {
                cl.camera_should_focus = player_transform
                    .translation
                    .lerp(bonus_transform.translation, 0.5);
            }
        // otherwise, if there is only a player, target the player
        } else if let Some(player_entity) = cl.player.entity {
            if let Ok(player_transform) = transforms.q1().get(player_entity) {
                cl.camera_should_focus = player_transform.translation;
            }
        // otherwise, target the middle
        } else {
            cl.camera_should_focus = Vec3::from(RESET_FOCUS);
        }
    }

    const SPEED: f32 = 2.0;

    // calculate the camera motion based on the difference between where the camera is looking
    // and where it should be looking; the greater the distance, the faster the motion;
    // smooth out the camera movement using the frame time
    let mut camera_motion = cl.camera_should_focus - cl.camera_is_focus;
    if camera_motion.length() > 0.2 {
        camera_motion *= SPEED * time.delta_seconds();
        // set the new camera's actual focus
        cl.camera_is_focus += camera_motion;
    }
    // look at that new camera's actual focus
    for (mut transform, camera) in transforms.q0_mut().iter_mut() {
        if camera.name == Some(CAMERA_3D.to_string()) {
            if delta_trans.translation != Vec3::ZERO { *transform = delta_trans }
            else { *transform = transform.looking_at(cl.camera_is_focus, Vec3::Y) }
        }
    }
}

// Listens for Z key being pressed and toggles between the two scroll-type states
#[allow(unused_must_use)]
fn switch_scroll_type(
    mut scroll_type: ResMut<State<ScrollType>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Z) {
        let result = match scroll_type.current() {
            ScrollType::Sensitivity => ScrollType::Zoom,
            ScrollType::Zoom => ScrollType::MovementSpeed,
            ScrollType::MovementSpeed => ScrollType::Sensitivity,
        };

        println!("{:?}", result);
        scroll_type.set(result);
    }
}

// Depending on the state, the mouse-scroll changes either the movement speed or the field-of-view of the camera
fn scroll(
    mut settings: ResMut<MovementSettings>,
    scroll_type: Res<State<ScrollType>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    windows: Res<Windows>,
    mut query: Query<(&FlyCam, &mut Camera, &mut PerspectiveProjection)>,
) {
    for event in mouse_wheel_events.iter() {
        match *scroll_type.current() {
            ScrollType::MovementSpeed => {
                settings.speed = (settings.speed + event.y * 0.1).abs();
                println!("Speed: {:?}", settings.speed);
            }
            ScrollType::Sensitivity => {
                settings.sensitivity = (settings.sensitivity + event.y * 0.000001).abs();
                println!("Sensitivity: {:?}", settings.sensitivity);
            }
            _ => {
                for (_camera, mut camera, mut project) in query.iter_mut() {
                    project.fov = (project.fov - event.y * 0.01).abs();
                    let prim = windows.get_primary().unwrap();
    
                    //Calculate projection with new fov
                    project.update(prim.width(), prim.height());
    
                    //Update camera with the new fov
                    camera.projection_matrix = project.get_projection_matrix();
                    camera.depth_calculation = project.depth_calculation();
    
                    println!("FOV: {:?}", project.fov);
                }
            }
        }
    }
}
