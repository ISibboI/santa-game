use crate::assets::SantaAssets;
use crate::physics::Position;
use crate::player::Santa;
use crate::snowflakes::init_snowflakes;
use bevy::prelude::*;

#[derive(StageLabel, Clone, Hash, Debug, Eq, PartialEq)]
pub enum LevelState {
    Outside,
    Indoors,
}

pub struct LevelPlayerBoundary(pub Rect<f32>);

pub struct LevelCameraBoundary(pub Rect<f32>);

pub struct SpawnPoint(pub Vec2);

fn init_level_system(mut commands: Commands) {
    commands.insert_resource(SpawnPoint(Vec2::new(-190.0, 0.0)));
}

pub struct OutsideLevel;

fn enter_outside_level_event(
    mut commands: Commands,
    santa_assets: Res<SantaAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut player_query: Query<&mut Position, With<Santa>>,
    spawn_point: Res<SpawnPoint>,
) {
    let level_camera_boundary = LevelCameraBoundary(Rect {
        top: 105.0,
        bottom: -105.0,
        left: -270.0,
        right: 270.0,
    });

    let level_root = commands
        .spawn()
        .insert(OutsideLevel)
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                material: materials.add(
                    texture_atlases
                        .get(santa_assets.outside_background.clone())
                        .unwrap()
                        .texture
                        .clone()
                        .into(),
                ),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..Default::default()
            });

            init_snowflakes(parent, &level_camera_boundary, &santa_assets);
        })
        .id();
    commands.insert_resource(LevelPlayerBoundary(Rect {
        top: 105.0,
        bottom: -97.0,
        left: -270.0,
        right: 270.0,
    }));
    commands.insert_resource(level_camera_boundary);
    for mut position in player_query.iter_mut() {
        position.0 = spawn_point.0;
    }
}

fn update_outside_level_event(
    mut state: ResMut<State<LevelState>>,
    player_query: Query<&Position, With<Santa>>,
    mut spawn_point: ResMut<SpawnPoint>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for position in player_query.iter() {
        if position.0.x >= 200.0 && keyboard_input.just_released(KeyCode::F) {
            state.set(LevelState::Indoors).unwrap();
            spawn_point.0 = Vec2::new(-80.0, -85.0);
        }
    }
}

fn exit_outside_level_event(mut commands: Commands, query: Query<Entity, With<OutsideLevel>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct IndoorsLevel;

fn enter_indoors_level_event(
    mut commands: Commands,
    assets: Res<SantaAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut player_query: Query<&mut Position, With<Santa>>,
    spawn_point: Res<SpawnPoint>,
) {
    commands
        .spawn()
        .insert(IndoorsLevel)
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                material: materials.add(
                    texture_atlases
                        .get(assets.indoors_background.clone())
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
        left: -105.0,
        right: 105.0,
    }));
    commands.insert_resource(LevelCameraBoundary(Rect {
        top: 105.0,
        bottom: -105.0,
        left: -105.0,
        right: 105.0,
    }));
    for mut position in player_query.iter_mut() {
        position.0 = spawn_point.0;
    }
}

fn update_indoors_level_event(
    mut state: ResMut<State<LevelState>>,
    player_query: Query<&Position, With<Santa>>,
    mut spawn_point: ResMut<SpawnPoint>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for position in player_query.iter() {
        if position.0.x <= -85.0 && keyboard_input.just_released(KeyCode::F) {
            state.set(LevelState::Outside).unwrap();
            spawn_point.0 = Vec2::new(195.0, -85.0);
        }
    }
}

fn exit_indoors_level_event(mut commands: Commands, query: Query<Entity, With<IndoorsLevel>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct SantaLevelPlugin;

#[derive(StageLabel, Clone, Debug, Eq, PartialEq, Hash)]
pub struct LevelStage;

impl Plugin for SantaLevelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(init_level_system.system())
            .add_stage_before(CoreStage::PreUpdate, LevelStage, SystemStage::parallel())
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
            )
            .add_system_set_to_stage(
                LevelStage,
                SystemSet::on_enter(LevelState::Indoors)
                    .with_system(enter_indoors_level_event.system()),
            )
            .add_system_set_to_stage(
                LevelStage,
                SystemSet::on_update(LevelState::Indoors)
                    .with_system(update_indoors_level_event.system()),
            )
            .add_system_set_to_stage(
                LevelStage,
                SystemSet::on_exit(LevelState::Indoors)
                    .with_system(exit_indoors_level_event.system()),
            );
    }
}
