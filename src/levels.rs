use crate::assets::SantaAssets;
use bevy::prelude::*;
use crate::physics::Position;
use crate::player::Santa;

#[derive(StageLabel, Clone, Hash, Debug, Eq, PartialEq)]
pub enum LevelState {
    Outside,
    Indoors,
}

pub struct OutsideLevel;

pub struct LevelPlayerBoundary(pub Rect<f32>);

pub struct LevelCameraBoundary(pub Rect<f32>);

fn enter_outside_level_event(
    mut commands: Commands,
    assets: Res<SantaAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut player_query: Query<&mut Position, With<Santa>>,
) {
    commands
        .spawn()
        .insert(OutsideLevel)
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                material: materials.add(
                    texture_atlases
                        .get(assets.outside_background.clone())
                        .unwrap()
                        .texture
                        .clone()
                        .into(),
                ),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..Default::default()
            });
        });
    commands.insert_resource(LevelPlayerBoundary(Rect {
        top: 105.0,
        bottom: -97.0,
        left: -270.0,
        right: 270.0,
    }));
    commands.insert_resource(LevelCameraBoundary(Rect {
        top: 105.0,
        bottom: -105.0,
        left: -270.0,
        right: 270.0,
    }));
    for mut position in player_query.iter_mut() {
        position.0 = Vec2::new(-190.0, 0.0);
    }
}

fn update_outside_level_event(mut state: ResMut<State<LevelState>>,
                              player_query: Query<&Position, With<Santa>>,) {
    for position in player_query.iter() {
        if position.0.x >= 200.0 {
            state.set(LevelState::Indoors);
        }
    }
}

fn exit_outside_level_event(
    mut commands: Commands,
    query: Query<Entity, With<OutsideLevel>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct SantaLevelPlugin;

#[derive(StageLabel, Clone, Debug, Eq, PartialEq, Hash)]
pub struct LevelStage;

impl Plugin for SantaLevelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage_before(CoreStage::PreUpdate, LevelStage, SystemStage::parallel())
            .add_state_to_stage(LevelStage, LevelState::Outside)
            .add_system_set_to_stage(
                LevelStage,
                SystemSet::on_enter(LevelState::Outside)
                    .with_system(enter_outside_level_event.system()),
            )
            .add_system_set_to_stage(
                LevelStage,
                SystemSet::on_update(LevelState::Outside)
                    .with_system(update_outside_level_event.system()),
            )
            .add_system_set_to_stage(
                LevelStage,
                SystemSet::on_exit(LevelState::Outside)
                    .with_system(exit_outside_level_event.system()),
            );
    }
}
