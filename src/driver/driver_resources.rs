use bevy::prelude::{ResMut, KeyCode, Input, Res, Camera, Query, Entity, Commands};

use super::driver_core::{DriverIndex, Drivers};


// TODO documentation


//Use collection with and keeping an index component or the like
pub fn change_driver_system(
    mut index: ResMut<DriverIndex>,
    drivers: Res<Drivers>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::T) {
        index.next(drivers.marker.len());
    }
}

pub fn update_driver_system(
    q: Query<(Entity, &Camera)>,
    mut commands: Commands,
    index: Res<DriverIndex>,
    drivers: Res<Drivers>,
) {
    if index.is_changed() {
        for box_component in &drivers.marker {
            let component = box_component.as_ref();
            let component_id = component.get_id();

            if let Some(h) = drivers.marker.get(index.index) {
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
