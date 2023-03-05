pub mod driver;
pub mod drivers;

use bevy::{
    ecs::component::TableStorage,
    input::Input,
    prelude::{
        default, App, Bundle, Camera, Camera2dBundle, Camera3d, Camera3dBundle, Commands,
        Component, IntoSystemDescriptor, KeyCode, OrthographicProjection, PerspectiveProjection,
        Plugin, Query, Res, ResMut, Resource, SystemLabel, Transform, Vec3, With,
    },
    reflect::Reflect,
    render::camera::{CameraProjection, CameraProjectionPlugin, ScalingMode},
};
use bevy_dolly::prelude::*;
use driver::{
    driver_core::{DriverRigs, Drivers},
    driver_resources::{change_driver_system, update_driver_system},
};
use drivers::{fpv::CCFpv, orbit::CCOrbit};
pub use std::any::TypeId;

// TODO documentation

pub struct ConfigCam;
impl Plugin for ConfigCam {
    fn build(&self, app: &mut App) {
        app.init_resource::<Drivers>()
            .init_resource::<CCConfig>()
            .add_plugin(DollyPosCtrl)
            .add_plugin(DollyCursorGrab)
            .insert_resource(DollyPosCtrlConfig {
                default_player: false,
                ..Default::default()
            })
            .add_startup_system(camera_setup.label(CCSetupLabel))
            .add_plugin(CCFpv)
            .add_plugin(CCOrbit)
            .add_system(change_driver_system)
            .add_system(update_driver_system)
            .add_system(update_look_at);
    }
}

#[derive(Resource)]
pub struct CCConfig {
    pub init_cameras: bool,
    pub features: Vec<String>,
}

impl Default for CCConfig {
    fn default() -> Self {
        Self {
            init_cameras: true,
            features: vec![],
        }
    }
}

pub fn bind_perspective<T: Bundle>(bundle: T) {}

fn camera_setup(mut commands: Commands, config: Res<CCConfig>) {
    if !config.init_cameras {
        return;
    }
    commands.spawn((
        MainCamera,
        PerspectiveCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(2.0, 2.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
    ));
    commands.spawn((
        OrthographicCamera,
        Camera3dBundle {
            projection: OrthographicProjection {
                scale: 3.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..default()
            }
            .into(),
            camera: Camera {
                is_active: false,
                ..Default::default()
            },
            ..Default::default()
        },
    ));
    commands.spawn((
        TwoDimensionalCamera,
        Camera2dBundle {
            camera: Camera {
                is_active: false,
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub struct CCSetupLabel;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct PerspectiveCamera;

#[derive(Component)]
pub struct OrthographicCamera;

#[derive(Component)]
pub struct TwoDimensionalCamera;

// Target at is just a valid option for Follow, Orbit and FPV
// Have the camera point at one or the summed vector direction
// of all entities with the Target Component
#[derive(Component)]
pub struct Target;

pub(crate) fn update_look_at(targets: Query<(&Transform, With<Target>)>, mut rigs: DriverRigs) {
    rigs.try_for_each_driver_mut::<LookAt>(|la| {
        let it = targets.iter();
        if it.len().eq(&0) {
        } else if it.len().eq(&1) {
            la.target = it.last().unwrap().0.translation;
        } else {
            la.target = get_center_point(it.map(|f| f.0.translation).collect());
        }
    });
}

fn get_center_point(targets: Vec<Vec3>) -> Vec3 {
    if targets.len() == 0 {
        Vec3::ONE
    } else if targets.len() == 1 {
        *targets.first().unwrap()
    } else {
        let mut a = Vec3::ONE;
        for t in targets {
            a += t
        }
        a * 0.5
    }
}
