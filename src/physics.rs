use crate::levels::LevelPlayerBoundary;
use crate::{TIME_STEP, TIME_STEP_F64};
use bevy::core::FixedTimestep;
use bevy::prelude::*;

#[derive(Default)]
pub struct Position(pub Vec2);

#[derive(Default)]
pub struct Speed(pub Vec2);

#[derive(Default)]
pub struct GroundState {
    pub on_ground: bool,
    pub just_landed: bool,
}

pub const GRAVITY: f32 = 450.0;
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
    level_boundary: Res<LevelPlayerBoundary>,
    mut query: Query<(
        &mut Position,
        &mut Speed,
        &SpriteBoundary,
        Option<&mut GroundState>,
    )>,
) {
    for (mut position, mut speed, sprite_boundary, mut ground_state) in query.iter_mut() {
        let min_x = level_boundary.0.left - sprite_boundary.0.left;
        let max_x = level_boundary.0.right - sprite_boundary.0.right;
        let min_y = level_boundary.0.bottom - sprite_boundary.0.bottom;
        let max_y = level_boundary.0.top - sprite_boundary.0.top;

        if position.0.x < min_x {
            position.0.x = min_x;
            speed.0.x = speed.0.x.max(0.0);
        }
        if position.0.x > max_x {
            position.0.x = max_x;
            speed.0.x = speed.0.x.min(0.0);
        }
        let detected_on_ground = if position.0.y < min_y {
            position.0.y = min_y;
            speed.0.y = speed.0.y.max(0.0);
            true
        } else {
            position.0.y == min_y
        };
        if position.0.y > max_y {
            position.0.y = max_y;
            speed.0.y = speed.0.y.min(0.0);
        }

        if let Some(mut ground_state) = ground_state {
            ground_state.just_landed = !ground_state.on_ground && detected_on_ground;
            ground_state.on_ground = detected_on_ground;
        }
    }
}

pub struct SantaPhysicsPlugin;

impl Plugin for SantaPhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage_after(
            CoreStage::Update,
            "santa_physics",
            SystemStage::parallel()
                //.with_run_criteria(FixedTimestep::step(TIME_STEP_F64))
                .with_system(gravity_system.system().before("move"))
                .with_system(move_system.system().label("move"))
                .with_system(level_boundary_system.system().after("move")),
        )
            .insert_resource(GroundState::default());
    }
}
