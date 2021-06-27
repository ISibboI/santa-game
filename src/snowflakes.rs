use crate::assets::SantaAssets;
use crate::levels::LevelCameraBoundary;
use crate::physics::Position;
use crate::TIME_STEP;
use bevy::prelude::*;
use noise::{BasicMulti, MultiFractal, NoiseFn, Seedable};
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

static SNOWFLAKE_DENSITY: f32 = 0.002;

#[derive(Default, Clone, Debug)]
struct Snowflake(Vec2);

pub fn init_snowflakes(
    parent: &mut ChildBuilder,
    level_camera_boundary: &LevelCameraBoundary,
    santa_assets: &Res<SantaAssets>,
) {
    let target_amount = (((level_camera_boundary.0.top - level_camera_boundary.0.bottom).abs()
        + 20.0)
        * ((level_camera_boundary.0.right - level_camera_boundary.0.left).abs() + 20.0)
        * SNOWFLAKE_DENSITY)
        .abs()
        .floor() as usize;

    let mut rng = thread_rng();

    for _ in 0..target_amount {
        let mut sprite = TextureAtlasSprite::default();
        sprite.index = Uniform::new(0, 4).sample(&mut rng);

        let position = Position(Vec2::new(
            Uniform::new(
                level_camera_boundary.0.left - 10.0,
                level_camera_boundary.0.right + 10.0,
            )
            .sample(&mut rng),
            Uniform::new(
                level_camera_boundary.0.bottom - 10.0,
                level_camera_boundary.0.top + 10.0,
            )
            .sample(&mut rng),
        ));
        let translation = Vec3::new(position.0.x, position.0.y, 0.5);

        parent
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: santa_assets.snowflakes.clone(),
                sprite,
                transform: Transform::from_translation(translation),
                ..Default::default()
            })
            .insert(Snowflake(Vec2::new(position.0.x, position.0.y)))
            .insert(position);
    }
}

fn update_snowflakes_system(
    time: Res<Time>,
    mut snowflakes_query: Query<(&mut Snowflake, &mut Position)>,
    level_camera_boundary: Res<LevelCameraBoundary>,
) {
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

    for (mut snowflake, mut position) in snowflakes_query.iter_mut() {
        snowflake.0.y -= 10.0 * TIME_STEP;
        if snowflake.0.y < level_camera_boundary.0.bottom - 10.0 {
            snowflake.0.x = Uniform::new(
                level_camera_boundary.0.left - 10.0,
                level_camera_boundary.0.right + 10.0,
            )
            .sample(&mut rng);
            snowflake.0.y = level_camera_boundary.0.top + 10.0;
        }

        let displacement_x = noise_x.get([
            snowflake.0.x as f64,
            snowflake.0.y as f64,
            time.seconds_since_startup() * 2.0,
        ]) as f32
            * 40.0;
        let displacement_y = noise_y.get([
            snowflake.0.x as f64,
            snowflake.0.y as f64,
            time.seconds_since_startup() * 2.0,
        ]) as f32
            * 25.0;

        position.0.x = snowflake.0.x + displacement_x;
        position.0.y = snowflake.0.y + displacement_y;
    }
}

pub struct SnowflakesPlugin;

impl Plugin for SnowflakesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(
            update_snowflakes_system
                .system()
                .label("update_snowflakes")
                .before("position_sprites"),
        );
    }
}
