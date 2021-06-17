use bevy::prelude::*;
use crate::assets::SantaAssets;

struct Santa;

fn init_santa_system(mut commands: Commands, assets: Res<SantaAssets>, mut materials: ResMut<Assets<ColorMaterial>>, texture_atlases: Res<Assets<TextureAtlas>>) {
    commands.spawn().insert(Santa).insert_bundle(SpriteSheetBundle {
        texture_atlas: assets.santa.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..Default::default()
    }).insert(Timer::from_seconds(0.3, true));
}

pub struct SantaPlayerPlugin;

impl Plugin for SantaPlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(init_santa_system.system());
    }
}