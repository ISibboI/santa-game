use crate::assets::SantaAssets;
use crate::levels::LevelCameraBoundary;
use crate::TIME_STEP;
use bevy::prelude::*;
use noise::{BasicMulti, MultiFractal, NoiseFn, Seedable};
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use crate::physics::Position;
use std::time::Instant;

static SNOWFLAKE_DENSITY: f32 = 0.002;

struct Snowflakes;

#[derive(Default, Clone, Debug)]
struct Snowflake;

pub fn init_snowflakes(
    parent: &mut ChildBuilder,
    level_camera_boundary: &LevelCameraBoundary,
    santa_assets: &Res<SantaAssets>,
) {
    let target_amount = (((level_camera_boundary.0.top - level_camera_boundary.0.bottom).abs() + 20.0)
        * ((level_camera_boundary.0.right - level_camera_boundary.0.left).abs() + 20.0)
        * SNOWFLAKE_DENSITY)
        .abs()
        .floor() as usize;
    println!("Spawning {} snowflakes", target_amount);

    let mut rng = thread_rng();

    for _ in 0..target_amount {
        let mut sprite = TextureAtlasSprite::default();
        sprite.index = Uniform::new(0, 4).sample(&mut rng);

        let position = Position(Vec2::new(Uniform::new(level_camera_boundary.0.left - 10.0, level_camera_boundary.0.right + 10.0)
                                              .sample(&mut rng),
                                          Uniform::new(level_camera_boundary.0.bottom - 10.0, level_camera_boundary.0.top + 10.0)
                                              .sample(&mut rng),));
        let translation = Vec3::new(
            position.0.x,
            position.0.y,
            0.5,
        );
        println!("Spawning snowflake at {:?}", translation);

        parent
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: santa_assets.snowflakes.clone(),
                sprite,
                global_transform: GlobalTransform::from_translation(translation),
                ..Default::default()
            })
            .insert(Snowflake)
            .insert(position);
    }
}

fn update_snowflakes_system(
    time: Res<Time>,
    mut snowflakes_query: Query<(&mut Position, &mut GlobalTransform), With<Snowflake>>,
    level_camera_boundary: Res<LevelCameraBoundary>,
) {
    let start = Instant::now();

    let mut rng = thread_rng();
    let noise_x = BasicMulti::new()
        .set_seed(0)
        .set_octaves(6)
        .set_frequency(0.02)
        .set_lacunarity(1.5)
        .set_persistence(0.7);
    let noise_y = BasicMulti::new()
        .set_seed(432627)
        .set_octaves(6)
        .set_frequency(0.02)
        .set_lacunarity(1.5)
        .set_persistence(0.7);

    let mut snowflake_count = 0;
    for (mut position,  mut global_transform) in snowflakes_query.iter_mut() {
        snowflake_count += 1;
        position.0.y -= 10.0 * TIME_STEP;
        if position.0.y < level_camera_boundary.0.bottom - 10.0 {
            position.0.x = Uniform::new(level_camera_boundary.0.left - 10.0, level_camera_boundary.0.right + 10.0)
                .sample(&mut rng);
            position.0.y = level_camera_boundary.0.top + 10.0;
        }

        let displacement_x = noise_x.get([
            position.0.x as f64,
            position.0.y as f64,
            time.seconds_since_startup() * 2.0,
        ]) as f32
            * 40.0;
        let displacement_y = noise_y.get([
            position.0.x as f64,
            position.0.y as f64,
            time.seconds_since_startup() * 2.0,
        ]) as f32
            * 25.0;

        global_transform.translation.x = position.0.x + displacement_x;
        global_transform.translation.y = position.0.y + displacement_y;
    }

    let duration = Instant::now() - start;
    //println!("Took {:.2}ms to update {} snowflakes", duration.as_secs_f64() * 1e3, snowflake_count);
}

pub struct SnowflakesPlugin;

impl Plugin for SnowflakesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update_snowflakes_system.system());
    }
}
