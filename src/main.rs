use std::f32::consts::PI;

use bevy::{prelude::*, render::camera::ScalingMode, pbr::CascadeShadowConfigBuilder};

const MOVEMENT_SPEED: f32 = 0.05;
const MOVEMENT_PER_KEY_MODIFIER: f32= 0.5;

#[derive(Component)]
struct Cubewoman;

fn add_people(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // camera
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(1.0),
            ..default()
        }
            .into(),
        transform: Transform::from_xyz(5.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // plane
    commands.spawn(
        PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0, subdivisions: 0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
        }
    );

    // cube?
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube{ size: 0.5 })),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz( 0.0, 0.5, 0.0),
            ..default()
        },
        Cubewoman
    ));

    // Lighting
    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 3.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 30.0,
            ..default()
        }
            .into(),
        ..default()
    });
}

fn move_cube(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Cubewoman>>,
) {
    let mut cube_transform = query.single_mut();
    let mut numb_keys: f32 = 0.0;
    let mut transform: Transform = Transform::from_xyz(0.0, 0.0, 0.0);

    if keyboard_input.pressed(KeyCode::Left) {
        transform.translation.x -= MOVEMENT_SPEED;
        numb_keys += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        transform.translation.x += MOVEMENT_SPEED;
        numb_keys += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        transform.translation.z -= MOVEMENT_SPEED;
        numb_keys += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        transform.translation.z += MOVEMENT_SPEED;
        numb_keys += 1.0;
    }

    if numb_keys == 0.0 {  return; }

    if numb_keys > 1.0 {
        println!("Reducing Movement Speed");
        transform.translation.x *= (MOVEMENT_PER_KEY_MODIFIER);
        transform.translation.z *= (MOVEMENT_PER_KEY_MODIFIER);
    }
    println!("Transforming Cube {:?}, with translation: {:?}", cube_transform.translation, transform.translation);
    cube_transform.translation += transform.translation;
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(add_people)
            .add_system(move_cube);
    }
}

fn main() {
    println!("Starting App...");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
