//Base
use bevy::prelude::*;
use bevy_config_cam::player::PlayerSettings;
use bevy_config_cam::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "bevy_config_cam example: custom_move.rs - Showing how to implement and use your own controller system".to_string(),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ConfigCam)
        .add_system(move_player.system())
        .add_startup_system(setup.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cl: ResMut<Config>,
    mut settings: ResMut<PlayerSettings>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // cube, set as player
    let p = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(PlayerMove)
        .id();

    settings.disable_move = true;
    cl.target = Some(p);

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

// control the cam logic character
fn move_player(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut transforms: Query<(&PlayerMove, &mut Transform)>,
) {
    for (_player, mut transform) in transforms.iter_mut() {
        let (_, mut rotation) = transform.rotation.to_axis_angle();
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = Vec3::new(local_z.x, 0., local_z.z);

        for key in keys.get_pressed() {
            if validate_key(&[KeyCode::Up], key) {
                velocity += forward
            }
            if validate_key(&[KeyCode::Down], key) {
                velocity -= forward
            }
            if validate_key(&[KeyCode::RShift], key) {
                velocity += Vec3::Y
            }
            if validate_key(&[KeyCode::Minus], key) {
                velocity -= Vec3::Y
            }
            if validate_key(&[KeyCode::Left], key) {
                //Wrapping around
                if rotation > std::f32::consts::FRAC_PI_2 * 4.0 - 0.05 {
                    rotation = 0.0;
                }
                rotation += 0.1
            }
            if validate_key(&[KeyCode::Right], key) {
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
