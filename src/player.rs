use crate::assets::SantaAssets;
use crate::physics::{Gravity, Position, Speed, SpriteBoundary};
use bevy::prelude::*;

pub struct Santa;

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
        .insert(Timer::from_seconds(0.3, true))
        .insert(Position(Vec2::new(-200.0, -50.0)))
        .insert(Speed(Vec2::new(1.0, 0.0)))
        .insert(SpriteBoundary(Rect {
            left: -15.0,
            right: 15.0,
            top: 25.0,
            bottom: -25.0,
        }))
        .insert(Gravity);
}

pub struct SantaPlayerPlugin;

impl Plugin for SantaPlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(init_santa_system.system());
    }
}
