use std::fmt::Debug;

use bevy::{
    app::{Events, ManualEventReader},
    ecs::schedule::{ShouldRun, SystemSet},
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::{
        camera::Camera,
        camera::CameraProjection,
        camera::{ActiveCameras, PerspectiveProjection},
        render_graph::base::camera::CAMERA_3D,
    },
    window::Windows,
};

pub mod default_cam_modes;
use crate::default_cam_modes::{
    FollowBehind, FollowStatic, Fps, Free, LookAt, TopDown, TopDownDirection,
};

pub mod cam;
use cam::{player_move, MovementSettings};

pub mod player;
use player::{move_player, PlayerSettings};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct PlayerMove;

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

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum PluginState {
    Enabled,
    //Disabled,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, EnumIter)]
enum ScrollType {
    MovementSpeed,
    Zoom,
    Sensitivity,
    Lerp,
    CamFwd,
}

//Free moving camera
pub struct StaticCam;
//Attached to target
pub struct AttachedCam;

#[derive(Clone, PartialEq)]
pub struct Config {
    pub target: Option<Entity>,
    pub external_target: Option<Entity>,
    pub camera_settings: CameraSettings,
    pub controller_settings: Option<Controller>,
    pub debug: bool,
}

#[derive(Debug)]
pub struct Cameras {
    pub current_camera_mode: usize,
    pub camera_modes: Vec<Box<dyn CameraMode + Sync + Send + 'static>>,
    pub debug: bool,
}

impl Debug for std::boxed::Box<dyn CameraMode + Send + Sync> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl PartialEq for std::boxed::Box<dyn CameraMode + Send + Sync> {
    fn eq(&self, other: &Self) -> bool {
        self.name().eq(other.name())
    }
}

impl Default for Cameras {
    fn default() -> Self {
        Self {
            current_camera_mode: 0usize,
            camera_modes: vec![
                Box::new(LookAt),
                Box::new(FollowStatic),
                Box::new(TopDown),
                Box::new(TopDownDirection),
                Box::new(FollowBehind),
                Box::new(Fps),
                Box::new(Free),
            ],
            debug: true,
        }
    }
}

#[allow(clippy::type_complexity)]
pub trait CameraMode {
    fn update(
        &self,
        config: ResMut<Config>,
        move_config: ResMut<MovementSettings>,
        transforms: &QuerySet<(Query<(&mut Transform, &Camera)>, Query<&Transform>)>,
    ) -> (Config, Transform);

    fn name(&self) -> &str;
}

pub trait ChangeTarget {
    //TODO Check the entity has a transform
    fn set_player_target(&mut self, entity: Entity) -> Result<(), &str>;
    //TODO Convert to using vec instead
    fn add_ext_target(&mut self, entity: Entity);
    fn remove_ext_target(&mut self);
    fn clear_ext_targets(&mut self);
}

impl ChangeTarget for Config {
    fn set_player_target(&mut self, entity: Entity) -> Result<(), &str> {
        self.target = Some(entity);
        Ok(())
    }

    fn add_ext_target(&mut self, entity: Entity) {
        self.external_target = Some(entity);
    }

    fn remove_ext_target(&mut self) {
        self.external_target = None;
    }

    fn clear_ext_targets(&mut self) {
        self.external_target = None;
    }
}

pub trait ChangeCamera {
    fn set_camera(
        &mut self,
        cam_mode: &std::boxed::Box<dyn CameraMode + Send + Sync>,
    ) -> Result<(), &str>;
    fn next_camera(&mut self);
}

impl ChangeCamera for Cameras {
    fn set_camera(
        self: &mut Cameras,
        cam_mode: &std::boxed::Box<dyn CameraMode + Send + Sync>,
    ) -> Result<(), &str> {
        let is_valid = self
            .camera_modes
            .iter()
            .any(|b| b.name() == cam_mode.name());
        if !is_valid {
            Err("This camera mode is not allowed")
        } else {
            let index = self
                .camera_modes
                .iter()
                .position(|cms| cms.name().eq(cam_mode.name()))
                .unwrap();
            self.current_camera_mode = index;
            if self.debug {
                println!(
                    "Camera: {:?}",
                    self.camera_modes.get(self.current_camera_mode).unwrap()
                );
            }
            Ok(())
        }
    }

    fn next_camera(self: &mut Cameras) {
        let current = &self.current_camera_mode;
        let available = &self.camera_modes;
        let next = if available.len() - 1 > *current {
            current + 1
        } else {
            0
        };

        self.current_camera_mode = next;
        if self.debug {
            println!(
                "Camera: {:?}",
                self.camera_modes.get(self.current_camera_mode).unwrap()
            );
        }
    }
}

/*
    The config component.
    Currently handles everything except input which is handled by cam.rs and player.rs respectively
    TODO:
        - Break up the component into a Controller component similar to smooth-bevy-cameras for easy integration
*/
impl Default for Config {
    fn default() -> Self {
        Self {
            /*
            current_camera_mode: 0,
            allowed_camera_modes: &[
                CameraMode::LookAt,
                CameraMode::FollowStatic,
                CameraMode::TopDown,
                CameraMode::TopDownDirection,
                CameraMode::FollowBehind,
                CameraMode::Fps,
                CameraMode::Free,
            ],*/
            target: None,
            external_target: None,
            camera_settings: CameraSettings {
                mouse_sensitivity: 0.00012,
                speed: 12.,
                pos: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
                map: 0.,
                camera: None,
                camera_should_focus: Vec3::default(),
                camera_is_focus: Vec3::default(),
                attached_camera: None,
            },
            controller_settings: Some(Controller {
                speed: 1.,
                rot_speed: 0.1,
                map: 0.,
            }),
            debug: true,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct CameraSettings {
    pub mouse_sensitivity: f32,
    pub speed: f32,
    pub pos: Transform,
    pub map: f32,
    pub camera: Option<Entity>,
    camera_should_focus: Vec3,
    camera_is_focus: Vec3,
    pub attached_camera: Option<Entity>,
}

#[derive(Clone, PartialEq)]
pub struct Controller {
    pub speed: f32,
    pub rot_speed: f32,
    pub map: f32,
}

/**
    Setting up the camera.
    If no camera is provided, a new camera is created
    else the StaticCam component is inserted onto the camera entity.
    TODO: Lookup for a new ConfigCamera tag-component
*/
fn setup_camera(mut commands: Commands, mut config: ResMut<Config>) {
    if config.camera_settings.camera.is_none() {
        config.camera_settings.camera = Some(
            commands
                .spawn_bundle(PerspectiveCameraBundle {
                    camera: Camera {
                        name: Some("Camera3d".to_string()),
                        ..Default::default()
                    },
                    transform: config.camera_settings.pos,
                    ..Default::default()
                })
                .insert(StaticCam)
                .id(),
        );
    } else {
        let mut e = commands.entity(config.camera_settings.camera.unwrap());
        config.camera_settings.camera = Some(e.insert(StaticCam).id());
    }
}

/*
    Setting up the controller.
    If no target(player) is provided, create a new default player (red cube).
    Then attach a unique follow camera to the player entity with component AttachedCam
    TODO:
        - Input a initial position from setup will move camera to the correct relative position
        - Allow the user to provided its own AttachedCam
*/
fn setup_controller(
    mut commands: Commands,
    mut config: ResMut<Config>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //Create or Update Target
    let trans = Vec3::new(0., 0.5, 0.);

    let player = if config.target.is_none() {
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.7, 0.3, 0.3).into()),
                transform: Transform::from_translation(trans),
                ..Default::default()
            })
            .id()
    } else {
        config.target.unwrap()
    };

    let a = commands
        .entity(player)
        .insert(PlayerMove)
        .with_children(|parent| {
            parent
                .spawn_bundle(PerspectiveCameraBundle {
                    camera: Camera {
                        name: Some("Target".to_string()),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(trans).looking_at(Vec3::ZERO, Vec3::Y),
                    ..Default::default()
                })
                .insert(AttachedCam);
        })
        .id();

    config.target = Some(a);
}

#[allow(unused_must_use)]
fn cycle_cam_state(
    //mut cam_state: ResMut<State<CameraMode>>,
    settings: Res<MovementSettings>,
    keyboard_input: Res<Input<KeyCode>>,
    mut cams: ResMut<Cameras>,
) {
    if keyboard_input
        .get_just_pressed()
        .any(|m| settings.map.next_cam.iter().any(|nc| m == nc))
    {
        cams.next_camera();
    }
}

// change the focus of the camera
#[allow(clippy::type_complexity)]
#[allow(unused_mut)]
fn move_camera(
    time: Res<Time>,
    mut config: ResMut<Config>,
    mut settings: ResMut<MovementSettings>,
    cameras: Res<Cameras>,
    mut transforms: QuerySet<(Query<(&mut Transform, &Camera)>, Query<&Transform>)>,
) {
    settings.disable_look = true;
    settings.disable_move = false;
    settings.locked_to_player = false;

    let (mut config2, delta_trans) = cameras.camera_modes[cameras.current_camera_mode].update(
        config,
        settings,
        &transforms,
    );

    /*let disable = false;
    if disable {
        config.camera_settings.camera_should_focus = Vec3::from(RESET_FOCUS);
    }*/

    const SPEED: f32 = 2.0;

    // calculate the camera motion based on the difference between where the camera is looking
    // and where it should be looking; the greater the distance, the faster the motion;
    // smooth out the camera movement using the frame time
    let mut camera_motion =
        config2.camera_settings.camera_should_focus - config2.camera_settings.camera_is_focus;
    if camera_motion.length() > 0.2 {
        camera_motion *= SPEED * time.delta_seconds();
        // set the new camera's actual focus
        config2.camera_settings.camera_is_focus += camera_motion;
    }
    // look at that new camera's actual focus
    for (mut transform, camera) in transforms.q0_mut().iter_mut() {
        if camera.name == Some(CAMERA_3D.to_string()) {
            if delta_trans.translation != Vec3::ZERO {
                *transform = delta_trans
            } else {
                *transform = transform.looking_at(config2.camera_settings.camera_is_focus, Vec3::Y)
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn toggle_camera_parent(
    mut act_cams: ResMut<ActiveCameras>,
    mut settings: ResMut<MovementSettings>,
    mut query: QuerySet<(
        Query<(&StaticCam, &mut Camera)>,
        Query<(&AttachedCam, &mut Camera)>,
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
        let result = next_enum!(ScrollType, scroll_type);

        println!("{:?}", result);
        scroll_type.set(result);
    }
}

fn show_cams(
    mut query: Query<(&mut Camera, &mut PerspectiveProjection)>,
    keyboard_input: Res<Input<KeyCode>>,
    config: Res<Config>,
) {
    if !config.debug {
        return;
    }
    if keyboard_input.just_pressed(KeyCode::Y) {
        for (a, _) in query.iter_mut() {
            println!("{:?}", a);
        }
    }
}

// Depending on the state, the mouse-scroll changes either the movement speed or the field-of-view of the camera
fn scroll(
    mut settings: ResMut<MovementSettings>,
    mut p_settings: ResMut<PlayerSettings>,
    scroll_type: Res<State<ScrollType>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    windows: Res<Windows>,
    mut query: Query<(&StaticCam, &mut Camera, &mut PerspectiveProjection)>,
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

//bevy_flycam by sburris0 - https://github.com/sburris0/bevy_flycam

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(window: &mut Window) {
    window.set_cursor_lock_mode(!window.cursor_locked());
    window.set_cursor_visibility(!window.cursor_visible());
}

/// Grabs the cursor when game first starts
fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    toggle_grab_cursor(windows.get_primary_mut().unwrap());
}

pub fn validate_key<T>(codes: &'static [T], key: &T) -> bool
where
    T: PartialEq<T>,
{
    codes.iter().any(|m| m == key)
}

fn should_mouse_look(settings: Local<MovementSettings>) -> ShouldRun {
    if settings.disable_look {
        ShouldRun::No
    } else {
        ShouldRun::Yes
    }
}

/// Handles looking around if cursor is locked
fn mouse_look(
    settings: Res<MovementSettings>,
    windows: Res<Windows>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<(&StaticCam, &mut Transform)>,
) {
    let window = windows.get_primary().unwrap();
    for (_camera, mut transform) in query.iter_mut() {
        for ev in state.reader_motion.iter(&motion) {
            if window.cursor_locked() {
                state.pitch -= (settings.sensitivity * ev.delta.y * window.height()).to_radians();
                state.yaw -= (settings.sensitivity * ev.delta.x * window.width()).to_radians();
            }

            state.pitch = state.pitch.clamp(-1.54, 1.54);

            // Order is important to prevent unintended roll
            transform.rotation = Quat::from_axis_angle(Vec3::Y, state.yaw)
                * Quat::from_axis_angle(Vec3::X, state.pitch);
        }
    }
}

fn cursor_grab(keys: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    if keys.just_pressed(KeyCode::Escape) {
        toggle_grab_cursor(window);
    }
}

pub struct ConfigCam;
impl Plugin for ConfigCam {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Config>()
            .init_resource::<Cameras>()
            .add_plugin(MovementPlugin)
            .init_resource::<PlayerSettings>()
            .add_state(PluginState::Enabled)
            .add_state(ScrollType::MovementSpeed)
            .add_system(toggle_camera_parent.system())
            .add_system(switch_scroll_type.system())
            .add_system(scroll.system())
            .add_system(show_cams.system())
            .add_system(cycle_cam_state.system())
            .add_system_set(
                SystemSet::on_enter(PluginState::Enabled)
                    .with_system(setup_camera.system())
                    .with_system(setup_controller.system()),
            )
            .add_system_set(
                SystemSet::on_update(PluginState::Enabled)
                    .with_system(move_player.system())
                    .with_system(move_camera.system()),
            );
    }
}
/// Same as `PlayerPlugin` but does not spawn a camera
pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .add_startup_system(initial_grab_cursor.system())
            .add_system(player_move.system())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(should_mouse_look.system())
                    .with_system(mouse_look.system()),
            )
            .add_system(cursor_grab.system());
    }
}
