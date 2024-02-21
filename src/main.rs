use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::highlight::Highlight;
use bevy_obj::ObjPlugin;
use bevy_mod_picking::*;
use bevy_mod_picking::prelude::On;
use bevy_mod_picking::events::{Drag, Pointer};



mod bullet;
mod target;
mod tower;

pub use bullet::*;
pub use target::*;
pub use tower::*;

pub const WIDTH: f32 = 720.0;
pub const HEIGHT: f32 = 1280.0;






fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
    .add_systems(PreStartup, asset_loading)
    .add_systems(Startup, (spawn_camera, spawn_basic_scene))
    .add_plugins((DefaultPlugins, DefaultPickingPlugins))
    //.add_systems(Update, (tower_shooting, bullet_despawn, move_bullets, move_targets, target_death))
    .add_systems(Update, camera_controls)
    .add_plugins((BulletPlugin, TargetPlugin, TowerPlugin))
    //.add_plugins(WorldInspectorPlugin::new())
    .run();
}

//Startup Systems
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-4.0, 9.0, 7.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        target_scene: assets.load("Tomato.glb#Scene0"),
        tower_base_scene: assets.load("TowerBase.glb#Scene0"),
        tomato_tower_scene: assets.load("TomatoTower.glb#Scene0"),
    });
    
    //let target_mesh = assets.load("target/10162_target_v01_l3.obj");
}


#[derive(Resource)]
pub struct GameAssets {
    target_scene: Handle<Scene>,
    tower_base_scene: Handle<Scene>,
    tomato_tower_scene: Handle<Scene>,
}

fn spawn_basic_scene(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<GameAssets>
) {
    //Spawn Ground
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {size: 25.0, ..default() })),
        material: materials.add(Color::rgb(0.7, 0.5, 0.7).into()),
        ..default()
    });

    for i in 0..25 {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {size: 0.1})),
            material: materials.add(Color::rgb(0.47, 0.84, 0.99).into()),
            transform: Transform::from_xyz(-12.5+(i as f32), 0.0, 2.0),
            ..default()
        });
    }

    //Spawn Tower (Base)

    let default_collider_color = materials.add(Color::rgba(0.3, 0.5, 0.3, 0.3).into());
    let selected_collider_color = materials.add(Color::rgba(0.3, 0.9, 0.3, 0.9).into());

    commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(
            0.0, 0.8, 0.0
        )), PickableBundle::default()))
        .insert(Name::new("Tower_Base"))
        .insert(meshes.add(shape::Capsule::default().into()))
        .insert(Highlight {
            hovered: Some(bevy_mod_picking::prelude::HighlightKind::Fixed(selected_collider_color.clone())),
            pressed: Some(bevy_mod_picking::prelude::HighlightKind::Fixed(selected_collider_color.clone())),
            selected: Some(bevy_mod_picking::prelude::HighlightKind::Fixed(selected_collider_color.clone()))
        })
        .insert(default_collider_color.clone())
        .insert(NotShadowCaster)
        .with_children(|commands| {
            commands.spawn(
                SceneBundle {
                    scene: game_assets.tower_base_scene.clone(),
                    transform: Transform::from_xyz(0.0,-0.8, 0.0),
                    ..default()
                }
            );
        }
    );

    commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(
            7.0, 0.8, 4.0
        )), PickableBundle::default()))
        .insert(Name::new("Tower_Base"))
        .insert(meshes.add(shape::Capsule::default().into()))
        .insert(Highlight {
            hovered: Some(bevy_mod_picking::prelude::HighlightKind::Fixed(selected_collider_color.clone())),
            pressed: Some(bevy_mod_picking::prelude::HighlightKind::Fixed(selected_collider_color.clone())),
            selected: Some(bevy_mod_picking::prelude::HighlightKind::Fixed(selected_collider_color.clone()))
        })
        .insert(default_collider_color.clone())
        .insert(NotShadowCaster)
        .with_children(|commands| {
            commands.spawn(
                SceneBundle {
                    scene: game_assets.tower_base_scene.clone(),
                    transform: Transform::from_xyz(0.0,-0.8, 0.0),
                    ..default()
                }
            );
        }
    );

    commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(
            -7.0, 0.8, 4.0
        )), PickableBundle::default()))
        .insert(Name::new("Tower_Base"))
        .insert(meshes.add(shape::Capsule::default().into()))
        .insert(Highlight {
            hovered: Some(bevy_mod_picking::prelude::HighlightKind::Fixed(selected_collider_color.clone())),
            pressed: Some(bevy_mod_picking::prelude::HighlightKind::Fixed(selected_collider_color.clone())),
            selected: Some(bevy_mod_picking::prelude::HighlightKind::Fixed(selected_collider_color.clone()))
        })
        .insert(default_collider_color.clone())
        .insert(NotShadowCaster)
        .with_children(|commands| {
            commands.spawn(
                SceneBundle {
                    scene: game_assets.tower_base_scene.clone(),
                    transform: Transform::from_xyz(0.0,-0.8, 0.0),
                    ..default()
                }
            );
        }
    );
    
    //Spawn Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    //Enemies
    for _i in 0..5 {

        commands.spawn(SceneBundle {
            scene: game_assets.target_scene.clone(),
            transform: Transform::from_xyz(-12.5 + (_i as f32), 1.0, 2.0).with_scale(Vec3::new(4.0,4.0,4.0)),
            ..default()
        })
        .insert( Target {
            speed: 0.3
        })
        .insert( Health {
            value: 3
        });
    }
}

//Update systems


fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();
    let mut forward = camera.forward();
    let mut left = camera.left();
    forward.y = 0.0;
    left.y = 0.0;
    forward = forward.normalize();
    left = left.normalize();

    let speed = 3.0;
    let rotate_speed: f32 = 0.3;

    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds());
    }
    if keyboard.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds());
    }

}
