use bevy::prelude::*;

use crate::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    pub timer: Timer,
}


#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Bullet {
    pub direction: Vec3,
    pub speed: f32,
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Lifetime>()
        .register_type::<Bullet>()
        .add_systems(Update, (bullet_collision, bullet_despawn, move_bullets));
    }
}

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
for (entity, mut lifetime) in &mut bullets {
    lifetime.timer.tick(time.delta());
    if lifetime.timer.just_finished() {
        commands.entity(entity).despawn_recursive();
    }
}
}

fn move_bullets(
mut bullets: Query<(&Bullet, &mut Transform)>, 
time: Res<Time>
) {
for (bullet, mut transform) in &mut bullets {
    transform.translation += bullet.direction.normalize() * bullet.speed * time.delta_seconds();
}
}

fn bullet_collision(
    mut commands: Commands,
    bullets: Query<(Entity, &GlobalTransform), With<Bullet>>,
    mut targets: Query<(&mut Health, &Transform), With<Target>>,
) {
    for(bullet, bullet_transform) in &bullets {
        for( mut health, target_transform) in &mut targets {
            if Vec3::distance(bullet_transform.translation(), target_transform.translation) < 0.2 {
                commands.entity(bullet).despawn_recursive();
                health.value  -= 1;
                break;
            }
        }
    }
}