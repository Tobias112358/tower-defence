use bevy::prelude::*;
use bevy::app::AppExit;



#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Target {
    pub speed: f32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health {
    pub value: i32,
}

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app:&mut App) {
        app
        .register_type::<Target>()
        .register_type::<Health>()
        .add_systems(Update, (move_targets, target_death));
    }
}

fn move_targets(
    mut targets: Query<(&Target, &mut Transform)>, 
    time: Res<Time>
) {
    for (target, mut transform) in &mut targets {
        transform.translation.x += target.speed * time.delta_seconds();
    }
}

fn target_death(
    mut commands: Commands,
    targets: Query<(Entity, &Health)>,
    mut exit: EventWriter<AppExit>
) {
    for (ent, health) in &targets {
        if health.value <= 0 {
            commands.entity(ent).despawn_recursive();
        }
    }

    if targets.is_empty() {
        exit.send(AppExit);
    }
}
