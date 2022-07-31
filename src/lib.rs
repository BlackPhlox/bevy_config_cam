use bevy::{
    math::Vec3,
    prelude::{App, Camera, Changed, Entity, Plugin, Query, Res, Time, Transform},
};
use bevy_dolly::prelude::*;

pub struct ConfigCam;
impl Plugin for ConfigCam {
    fn build(&self, app: &mut App) {
        app.add_system(config_cam_change_detection);
    }
}

fn config_cam_change_detection(
    mut cameras: Query<(&mut Transform, &Camera)>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Rig), Changed<Rig>>,
) {
    for (_entity, mut rig) in &mut query {
        //let d = rig.drivers.iter().map(|f| format!("{:?}", f)).collect::<Vec<String>>().join(", ");
        //info!("{:?} changed: {:?}", entity, d);

        let transform = rig.update(time.delta_seconds());

        cameras.for_each_mut(|(mut t, camera)| {
            if camera.is_active {
                t.transform_2_bevy(transform);
            }
        });
    }
}
