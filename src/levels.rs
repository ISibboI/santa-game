use bevy::prelude::*;
use crate::assets::SantaAssets;

#[derive(StageLabel, Clone, Hash, Debug, Eq, PartialEq)]
pub enum LevelState {
    Outside,
    Indoors,
}

pub struct OutsideLevel;

fn enter_outside_level_event(mut commands: Commands, assets: Res<SantaAssets>, mut materials: ResMut<Assets<ColorMaterial>>, texture_atlases: Res<Assets<TextureAtlas>>) {
    println!("Enter outside level event");

    commands.spawn().insert(OutsideLevel).with_children(|parent| {
        parent.spawn_bundle(SpriteBundle {
            material: materials.add(texture_atlases.get(assets.outside_background.clone()).unwrap().texture.clone().into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        });
    });
}

pub struct SantaLevelPlugin;

#[derive(StageLabel, Clone, Debug, Eq, PartialEq, Hash)]
pub struct LevelStage;

impl Plugin for SantaLevelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage_after(CoreStage::PostUpdate, LevelStage, SystemStage::parallel())
            .add_state_to_stage(LevelStage, LevelState::Outside)
            .add_system_set_to_stage(LevelStage, SystemSet::on_enter(LevelState::Outside).with_system(enter_outside_level_event.system()));
    }
}

