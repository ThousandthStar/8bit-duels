use crate::utils;
use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimMoveSpeed(5.0))
            .add_system(movement_anims)
            .add_system(attack_anims);
    }
}

#[derive(Resource)]
pub struct AnimMoveSpeed(f32);

#[derive(Component)]
pub struct MovementAnimation {
    pub target: Vec2,
}
#[derive(Component)]
pub struct AttackAnimation {
    pub target: Vec2,
    pub initial: Vec2,
    pub moving_back: bool,
}

fn movement_anims(
    mut query: Query<(
        Entity,
        &mut Transform,
        &MovementAnimation,
        Option<&AttackAnimation>,
    )>,
    speed: Res<AnimMoveSpeed>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut transform, movement_anim, att_anim_opt) in query.iter_mut() {
        if att_anim_opt.is_some() {
            continue;
        }
        let current_v2 = Vec2::new(transform.translation.x, transform.translation.y);
        let movement = utils::move_towards(
            current_v2,
            movement_anim.target,
            speed.0,
            time.delta_seconds(),
        );
        transform.translation.x = movement.x;
        transform.translation.y = movement.y;
        if movement == movement_anim.target {
            commands.entity(entity).remove::<MovementAnimation>();
        }
    }
}

fn attack_anims(
    mut query: Query<(Entity, &mut Transform, &mut AttackAnimation)>,
    speed: Res<AnimMoveSpeed>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut transform, mut atck_anim) in query.iter_mut() {
        let current_v2 = Vec2::new(transform.translation.x, transform.translation.y);
        let movement = utils::move_towards(
            current_v2,
            if !atck_anim.moving_back {
                atck_anim.target
            } else {
                atck_anim.initial
            },
            speed.0 * 3.0,
            time.delta_seconds(),
        );
        transform.translation.x = movement.x;
        transform.translation.y = movement.y;
        if movement == atck_anim.target || movement == atck_anim.initial {
            if atck_anim.moving_back {
                commands.entity(entity).remove::<AttackAnimation>();
            } else {
                atck_anim.moving_back = true;
            }
        }
    }
}
