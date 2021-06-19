use crate::levels::LevelBoundary;
use crate::{TIME_STEP, TIME_STEP_F64};
use bevy::core::FixedTimestep;
use bevy::prelude::*;

pub struct Position(pub Vec2);

pub struct Speed(pub Vec2);

const GRAVITY: f32 = 10.0;
pub struct Gravity;

pub struct SpriteBoundary(pub Rect<f32>);

fn move_system(mut query: Query<(&mut Position, &Speed)>) {
    for (mut position, speed) in query.iter_mut() {
        position.0 += speed.0 * TIME_STEP;
    }
}

fn gravity_system(mut query: Query<&mut Speed, With<Gravity>>) {
    for mut speed in query.iter_mut() {
        speed.0.y -= GRAVITY * TIME_STEP;
    }
}

fn level_boundary_system(
    level_boundary: Res<LevelBoundary>,
    mut query: Query<(&mut Position, &SpriteBoundary)>,
) {
    for (mut position, sprite_boundary) in query.iter_mut() {
        position.0.x = position
            .0
            .x
            .max(level_boundary.0.left - sprite_boundary.0.left);
        position.0.x = position
            .0
            .x
            .min(level_boundary.0.right - sprite_boundary.0.right);
        position.0.y = position
            .0
            .y
            .max(level_boundary.0.bottom - sprite_boundary.0.bottom);
        position.0.y = position
            .0
            .y
            .min(level_boundary.0.top - sprite_boundary.0.top);
    }
}

pub struct SantaPhysicsPlugin;

impl Plugin for SantaPhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage_after(
            CoreStage::Update,
            "santa_physics",
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(TIME_STEP_F64))
                .with_system(gravity_system.system().before("move"))
                .with_system(move_system.system().label("move"))
                .with_system(level_boundary_system.system().after("move")),
        );
    }
}
