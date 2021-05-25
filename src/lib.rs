//Base
use bevy::{
    core::FixedTimestep,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    ecs::schedule::SystemSet,
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
    render::{
        camera::Camera,
        camera::CameraProjection,
        camera::{ActiveCameras, PerspectiveProjection},
        render_graph::base::camera::CAMERA_3D,
    },
    window::{CursorMoved, Windows},
};

use bevy_flycam::{CamKeyMap, FlyCam, MovementSettings, NoCameraPlayerPlugin, PlayerCam};
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
    Lerp,
    CamFwd,
}

//Plan:
// Merge LookAt and FollowStatic into FollowFree
// Change rotation of TopDown cam, either a rotation or aligned with player rotation
// Make FollowBehind actually work, make parenting work
// Merge FPS and Free into one called FPS and have a no-clip and different control-scheme setting (rotation with arrows or mouse etc.)
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum CameraState {
    //Look at player and other targets if set
    LookAt,
    //Follow the player only
    FollowStatic,
    //Camera is moved above and pointed down, rotation bound to one axis
    TopDown,
    TopDownDirection,
    //Follows behind the player a certain distance
    FollowBehind,
    //Camera at same position as player, enables to use the mouse to look
    FPS,
    //Use the mouse to look and move the camera freely
    Free,
    //Same as Free, but going forward is base on where your looking
    //FreeFPS
}

pub struct PlayerKeyMap {
    pub forward: &'static [KeyCode],
    pub backward: &'static [KeyCode],
    pub left: &'static [KeyCode],
    pub right: &'static [KeyCode],
    pub up: &'static [KeyCode],
    pub down: &'static [KeyCode],
    pub rot_left: &'static [KeyCode],
    pub rot_right: &'static [KeyCode],
}

pub struct PlayerSettings {
    pub speed: f32,
    pub map: PlayerKeyMap,
    pub pos: Vec3,
    pub cam_fwd: bool,
}

impl Default for PlayerKeyMap {
    fn default() -> Self {
        Self {
            forward: &[KeyCode::Up],
            backward: &[KeyCode::Down],
            left: &[KeyCode::Comma],
            right: &[KeyCode::Period],
            up: &[KeyCode::RShift],
            down: &[KeyCode::Minus],
            rot_left: &[KeyCode::Left],
            rot_right: &[KeyCode::Right],
        }
    }
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            speed: 12.0,
            map: PlayerKeyMap::default(),
            pos: Default::default(),
            cam_fwd: false,
        }
    }
}

pub struct ConfigCam;
impl Plugin for ConfigCam {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<CamLogic>()
            .add_plugin(NoCameraPlayerPlugin)
            .init_resource::<PlayerSettings>()
            .add_state(PluginState::Enabled)
            .add_state(CameraState::LookAt)
            .add_state(ScrollType::MovementSpeed)
            .add_system(toggle_camera_parent.system())
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
pub struct Player {
    entity: Option<Entity>,
}

#[derive(Default)]
pub struct CamLogic {
    player: Player,
    camera_should_focus: Vec3,
    camera_is_focus: Vec3,
    pub target: Option<Entity>,
}

const RESET_FOCUS: [f32; 3] = [0., 0., 0.];

#[allow(unused_must_use)]
fn cycle_cam_state(
    mut cam_state: ResMut<State<CameraState>>,
    settings: Res<MovementSettings>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input
        .get_just_pressed()
        .any(|m| settings.map.next_cam.iter().any(|nc| m == nc))
    {
        let result = match cam_state.current() {
            CameraState::LookAt => CameraState::FollowStatic,
            CameraState::FollowStatic => CameraState::TopDown,
            CameraState::TopDown => CameraState::TopDownDirection,
            CameraState::TopDownDirection => CameraState::FollowBehind,
            CameraState::FollowBehind => CameraState::FPS,
            CameraState::FPS => CameraState::Free,
            CameraState::Free => CameraState::LookAt,
        };

        println!("Camera: {:?}", result);
        cam_state.set(result);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut cl: ResMut<CamLogic>,
    settings: Res<PlayerSettings>,
) {
    let mut c2: Camera = Camera::default();
    c2.name = Some("player".to_string());

    // spawn the cam logic character
    cl.player.entity = Some(
        commands
            .spawn_bundle((
                Transform {
                    translation: settings.pos,
                    rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
                    ..Default::default()
                },
                GlobalTransform::identity(),
            ))
            .insert(PlayerMove)
            .with_children(|cell| {
                cell.spawn_scene(asset_server.load("models/AlienCake/craft_speederA.glb#Scene0"));
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(PerspectiveCameraBundle {
                        camera: c2,
                        transform: Transform::from_xyz(-2.0, 5.0, 5.0)
                            .looking_at(Vec3::ZERO, Vec3::Y),
                        ..Default::default()
                    })
                    .insert(PlayerCam);
            })
            .id(),
    );

    let mut c: Camera = Camera::default();
    c.name = Some("Camera3d".to_string());

    // camera
    let camera = PerspectiveCameraBundle {
        camera: c,
        transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };

    // add plugin
    commands.spawn_bundle(camera).insert(FlyCam);
}

// control the cam logic character
fn move_player(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    settings: Res<PlayerSettings>,
    mut transforms: Query<(&PlayerMove, &mut Transform)>,
) {
    for (_player, mut transform) in transforms.iter_mut() {
        let (_, mut rotation) = transform.rotation.to_axis_angle();
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        //Forward should be togglable either xyz or cam direction xyz
        let forward = if settings.cam_fwd {
            Vec3::new(local_z.x, 0., local_z.z)
        } else {
            -Vec3::new(local_z.x, 0., local_z.z)
        };

        let right = Vec3::new(local_z.z, 0., -local_z.x);

        for key in keys.get_pressed() {
            if bevy_flycam::validate_key(settings.map.forward, key) {
                velocity += forward
            }
            if bevy_flycam::validate_key(settings.map.backward, key) {
                velocity -= forward
            }
            if bevy_flycam::validate_key(settings.map.left, key) {
                velocity -= right
            }
            if bevy_flycam::validate_key(settings.map.right, key) {
                velocity += right
            }
            if bevy_flycam::validate_key(settings.map.up, key) {
                velocity += Vec3::Y
            }
            if bevy_flycam::validate_key(settings.map.down, key) {
                velocity -= Vec3::Y
            }
            if bevy_flycam::validate_key(settings.map.rot_left, key) {
                //Wrapping around
                if rotation > std::f32::consts::FRAC_PI_2 * 4.0 - 0.05 {
                    rotation = 0.0;
                }
                rotation += 0.1
            }
            if bevy_flycam::validate_key(settings.map.rot_right, key) {
                //Wrapping around
                if rotation < 0.05 {
                    rotation = std::f32::consts::FRAC_PI_2 * 4.0;
                }
                rotation -= 0.1
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
    state: Res<State<CameraState>>,
    mut cl: ResMut<CamLogic>,
    mut settings: ResMut<MovementSettings>,
    mut transforms: QuerySet<(Query<(&mut Transform, &Camera)>, Query<&Transform>)>,
) {
    let mut delta_trans = Transform::identity();
    settings.disable_look = true;
    settings.disable_move = false;
    settings.locked_to_player = false;

    match *state.current() {
        CameraState::Free => {
            settings.disable_look = false;
            return;
        }
        CameraState::LookAt => {
            // if there is both a player and a bonus, target the mid-point of them
            if let (Some(player_entity), Some(bonus_entity)) = (cl.player.entity, cl.target) {
                if let (Ok(player_transform), Ok(bonus_transform)) = (
                    transforms.q1().get(player_entity),
                    transforms.q1().get(bonus_entity),
                ) {
                    cl.camera_should_focus = player_transform
                        .translation
                        .lerp(bonus_transform.translation, settings.lerp);
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
        _ => {
            if let Some(player_entity) = cl.player.entity {
                if let Ok(player_transform) = transforms.q1().get(player_entity) {
                    match *state.current() {
                        CameraState::FPS => {
                            delta_trans.translation = player_transform.translation;
                            settings.disable_move = true;

                            settings.disable_look = false;
                            delta_trans.rotation = player_transform.rotation;
                            delta_trans.translation += Vec3::new(/*-4.*/ 0., 1., 0.);
                        }
                        CameraState::TopDown => {
                            delta_trans.translation = player_transform.translation;
                            settings.disable_move = true;

                            delta_trans.translation +=
                                Vec3::new(/*-4.*/ 0., settings.dist, 0.);
                            delta_trans.rotation =
                                Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)
                        }
                        CameraState::TopDownDirection => {
                            settings.disable_move = true;
                            settings.locked_to_player = true;

                            delta_trans.translation +=
                                Vec3::new(/*-4.*/ 0., settings.dist, 0.);
                            delta_trans.rotation =
                                Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2);
                        }
                        CameraState::FollowBehind => {
                            settings.disable_move = true;

                            settings.locked_to_player = true;

                            delta_trans.translation += Vec3::new(/*-4.*/ 0., 1., 4.);
                        }
                        _ => {}
                    }
                    cl.camera_should_focus = player_transform.translation;
                }
            // otherwise, target the middle
            } else {
                cl.camera_should_focus = Vec3::from(RESET_FOCUS);
            }
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
            if delta_trans.translation != Vec3::ZERO {
                *transform = delta_trans
            } else {
                *transform = transform.looking_at(cl.camera_is_focus, Vec3::Y)
            }
        }
    }
}

fn toggle_camera_parent(
    mut act_cams: ResMut<ActiveCameras>,
    mut settings: ResMut<MovementSettings>,
    mut query: QuerySet<(
        Query<(&FlyCam, &mut Camera)>,
        Query<(&PlayerCam, &mut Camera)>,
    )>,
) {
    if settings.locked_to_player && !settings.ltp {
        act_cams.remove("Camera3d");

        let (_, mut b) = query.q1_mut().single_mut().unwrap();
        b.name = Some("Camera3d".to_string());

        act_cams.add("Camera3d");

        let (_, mut b) = query.q0_mut().single_mut().unwrap();
        b.name = Some("Test".to_string());

        settings.ltp = true;
    } else if !settings.locked_to_player && settings.ltp {
        act_cams.remove("Camera3d");

        let (_, mut b) = query.q0_mut().single_mut().unwrap();
        b.name = Some("Camera3d".to_string());

        act_cams.add("Camera3d");

        let (_, mut b) = query.q1_mut().single_mut().unwrap();
        b.name = Some("Test".to_string());

        settings.ltp = false;
    }
}

// Listens for Z key being pressed and toggles between the scroll-type states
#[allow(unused_must_use)]
fn switch_scroll_type(
    mut scroll_type: ResMut<State<ScrollType>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::E) {
        let result = match scroll_type.current() {
            ScrollType::Sensitivity => ScrollType::Zoom,
            ScrollType::Zoom => ScrollType::MovementSpeed,
            ScrollType::MovementSpeed => ScrollType::Lerp,
            ScrollType::Lerp => ScrollType::CamFwd,
            ScrollType::CamFwd => ScrollType::Sensitivity,
        };

        println!("{:?}", result);
        scroll_type.set(result);
    }
}

// Depending on the state, the mouse-scroll changes either the movement speed or the field-of-view of the camera
fn scroll(
    mut settings: ResMut<MovementSettings>,
    mut p_settings: ResMut<PlayerSettings>,
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
            ScrollType::Zoom => {
                for (_camera, mut camera, mut project) in query.iter_mut() {
                    println!("{:?}", camera.name);
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
            ScrollType::Lerp => {
                settings.lerp = (settings.lerp + event.y * 0.01).abs();
                println!("Lerp: {:?}", settings.lerp);
            }
            ScrollType::CamFwd => {
                if event.y > 0.01 {
                    p_settings.cam_fwd = !p_settings.cam_fwd;
                }
                println!("CamFwd: {:?}", p_settings.cam_fwd);
            }
        }
    }
}
