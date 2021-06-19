use crate::assets::SantaAssets;
use crate::physics::{Gravity, Position, Speed, SpriteBoundary, GroundState, GRAVITY};
use bevy::prelude::*;
use crate::TIME_STEP;

const MAX_WALK_SPEED: f32 = 50.0;
const WALK_ACCELERATION: f32 = 100.0;
const WALK_DECELERATION: f32 = 500.0;
const MAX_JUMP_HEIGHT: f32 = 30.0;
lazy_static! {
    static ref JUMP_TIME: f32 = (MAX_JUMP_HEIGHT / GRAVITY).sqrt();
    static ref JUMP_POWER: f32 = GRAVITY * *JUMP_TIME + MAX_JUMP_HEIGHT / *JUMP_TIME;
}

pub struct Santa;

pub struct AnimationTimer(pub Timer);

fn init_santa_system(
    mut commands: Commands,
    assets: Res<SantaAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    commands
        .spawn()
        .insert(Santa)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: assets.santa.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.3, true)))
        .insert(Position::default())
        .insert(Speed::default())
        .insert(SpriteBoundary(Rect {
            left: -15.0,
            right: 15.0,
            top: 25.0,
            bottom: -25.0,
        }))
        .insert(Gravity)
        .insert(GroundState::default());
}

fn control_santa_system(keyboard_input: Res<Input<KeyCode>>, mut santa_query: Query<(&mut Speed, &GroundState), With<Santa>>) {
    for (mut speed, ground_state) in santa_query.iter_mut() {
        if ground_state.on_ground {
            let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
            let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
            let jump = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Space);

            let mut accelerating = false;
            if left && !right {
                speed.0.x = (speed.0.x - (if speed.0.x <= 0.0 {WALK_ACCELERATION} else {WALK_DECELERATION}) * TIME_STEP).max(-MAX_WALK_SPEED);
                accelerating = true;
            } else if right && !left {
                speed.0.x = (speed.0.x + (if speed.0.x >= 0.0 {WALK_ACCELERATION} else {WALK_DECELERATION}) * TIME_STEP).min(MAX_WALK_SPEED);
                accelerating = true;
            }

            if !accelerating {
                if speed.0.x > 0.0 {
                    speed.0.x = (speed.0.x - (WALK_DECELERATION * TIME_STEP)).max(0.0);
                } else if speed.0.x < 0.0 {
                    speed.0.x = ((WALK_DECELERATION * TIME_STEP) + speed.0.x).min(0.0);
                }
            }

            if jump {
                speed.0.y = *JUMP_POWER;
            }
        }
    }
}

fn animate_santa_system(time: Res<Time>, mut query: Query<(&mut Transform, &Speed, &GroundState, &mut AnimationTimer, &mut TextureAtlasSprite), With<Santa>>) {
    for (mut transform, speed, ground_state, mut animation_timer, mut sprite) in query.iter_mut() {
        let mut moving = false;

        if speed.0.x > 0.0 {
            transform.scale.x = 1.0;
            moving = true;
        } else if speed.0.x < 0.0 {
            transform.scale.x = -1.0;
            moving = true;
        }

        animation_timer.0.tick(time.delta());
        if animation_timer.0.just_finished() {
            if moving {
                sprite.index = 1 - sprite.index;
            } else {
                sprite.index = 0;
            }
        }

        if !ground_state.on_ground {
            sprite.index = 1;
        } else if ground_state.just_landed {
            if speed.0.y < 1.0 {
                animation_timer.0.reset();
                sprite.index = 0;
            } else {
                sprite.index = 1;
            }
        }
    }
}

pub struct SantaPlayerPlugin;

impl Plugin for SantaPlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(init_santa_system.system())
            .add_system_to_stage(CoreStage::PreUpdate, control_santa_system.system().label("control_santa"))
            .add_system_to_stage(CoreStage::PreUpdate, animate_santa_system.system().after("control_santa"));
    }
}
