use bevy::{
    ecs::{component::TableStorage, system::SystemParam},
    input::Input,
    prelude::{
        App, Camera, Commands, Component, Entity, KeyCode, Plugin, Query, Res, ResMut, Transform,
        Vec3, With,
    },
};
use bevy_dolly::{dolly::glam, prelude::*};
use driver_marker_derive::DriverMarker;

pub use std::any::TypeId;

pub struct ConfigCam;
impl Plugin for ConfigCam {
    fn build(&self, app: &mut App) {
        app.init_resource::<DriverIndex>()
            .init_resource::<Drivers>()
            .add_rig_component(Pinned)
            .add_rig_component(FPV)
            .add_startup_system(default_setup)
            .add_system(change_driver_system)
            .add_system(update_driver_system)
            .add_system(update_look_at)
            .add_system(update_yaw_driver);
    }
}

pub(crate) fn update_look_at(
    mut targets: Query<(&mut Transform, With<Target>)>,
    mut rigs: DriverRigs,
) {
    let mut avg = Vec3::ZERO;

    for (t, _b) in &mut targets {
        avg += t.translation;
    }

    //https://math.stackexchange.com/questions/80923/average-of-multiple-vectors
    let total_targets = targets.iter().count();
    //avg /= total_targets as f32;

    rigs.try_for_each_driver_mut::<bevy_dolly::prelude::LookAt>(|la| {
        la.target = avg;
    });
}

pub(crate) fn update_yaw_driver(keys: Res<Input<KeyCode>>, mut rigs: DriverRigs) {
    // Waiting for 1.63 for stable, use nightly until August 11 2022
    // https://forge.rust-lang.org/#current-release-versions
    // https://github.com/rust-lang/rust/issues/83701
    rigs.try_for_each_driver_mut::<YawPitch>(|yp| {
        if keys.just_pressed(KeyCode::Z) {
            yp.rotate_yaw_pitch(-90.0, 0.0);
        }
        if keys.just_pressed(KeyCode::X) {
            yp.rotate_yaw_pitch(90.0, 0.0);
        }
    });
}

// Target at is just a valid option for Follow, Orbit and FPV
// Have the camera point at one or the summed vector direction
// of all entities with the Target Component
#[derive(Component)]
pub struct Target;

#[derive(DriverMarker, Component, Clone, Copy, Debug)]
pub struct Pinned;
//Substates:
//Locked Rotation
//Free

#[derive(DriverMarker, Component, Clone, Copy, Debug)]
pub struct FPV;

fn default_setup(mut commands: Commands) {
    //Should be default player entity
    //Default player entity : Cone
    //commands.spawn().insert(Target);

    commands
        .spawn()
        .insert(
            Rig::builder()
                .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-45.0))
                .with(Smooth::new_rotation(1.5))
                .with(Arm::new(glam::Vec3::Z * 4.0))
                .with(bevy_dolly::prelude::LookAt::new(glam::Vec3::new(
                    0., 0., 0.,
                )))
                .build(),
        )
        .insert(Pinned);

    //Missing FPV
    commands.spawn().insert(FPV);
}

#[derive(SystemParam)]
struct DriverRigs<'w, 's> {
    rigs: Query<'w, 's, &'static mut Rig>,
}

impl<'w, 's> DriverRigs<'w, 's> {
    fn try_for_each_driver_mut<T>(&mut self, f: impl FnOnce(&mut T) + std::marker::Copy)
    where
        T: bevy_dolly::prelude::RigDriver<bevy_dolly::prelude::RightHanded>,
    {
        for mut rig in &mut self.rigs {
            if let Some(camera_driver) = rig.try_driver_mut::<T>() {
                f(camera_driver);
            }
        }
    }
}

pub struct Drivers(Vec<Box<dyn DriverMarker>>);

impl Default for Drivers {
    fn default() -> Self {
        Self(vec![Box::new(Pinned), Box::new(FPV)])
    }
}

impl Drivers {
    pub fn new(driver_markers: Vec<Box<dyn DriverMarker>>) -> Self {
        Self(driver_markers)
    }
}

pub trait DriverMarker: Component<Storage = TableStorage> + Sync + Send + 'static {
    fn get_id(&self) -> TypeId;
    fn get_name(&self) -> &str;
    fn add_to(&self, commands: &mut Commands, entity: Entity);
    fn remove_from(&self, commands: &mut Commands, entity: Entity);
}

#[derive(Default)]
pub struct DriverIndex(usize);

impl DriverIndex {
    pub fn next(&mut self, len: usize) {
        if self.0 >= len - 1 {
            self.0 = 0;
        } else {
            self.0 += 1;
        }
    }
}

//Use collection with and keeping an index component or the like
fn change_driver_system(
    mut index: ResMut<DriverIndex>,
    drivers: Res<Drivers>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::T) {
        index.next(drivers.0.len());
    }
}

fn update_driver_system(
    q: Query<(Entity, &Camera)>,
    mut commands: Commands,
    index: Res<DriverIndex>,
    drivers: Res<Drivers>,
) {
    if index.is_changed() {
        for box_component in &drivers.0 {
            let component = box_component.as_ref();
            let component_id = component.get_id();

            if let Some(h) = drivers.0.get(index.0) {
                if component_id.eq(&h.get_id()) {
                    //Add new driver component
                    //Remove old driver component
                    for (entity, camera) in q.iter() {
                        if camera.is_active {
                            component.add_to(&mut commands, entity);
                            println!("Adding {:?} to Camera {:?}", component.get_name(), entity);
                        }
                    }
                } else {
                    for (entity, camera) in q.iter() {
                        if camera.is_active {
                            component.remove_from(&mut commands, entity);
                            println!("Remove {:?} from Camera {:?}", component.get_name(), entity);
                        }
                    }
                }
            }
        }
    }
}
