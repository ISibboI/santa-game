use crate::assets::{AssetsReady, SantaAssets};
use bevy::prelude::*;
use std::collections::VecDeque;
use crate::physics::{GroundState, Position};
use crate::player::Santa;
use crate::levels::IndoorsLevel;

#[derive(Default)]
pub struct DialogueQueue {
    backlog: VecDeque<String>,
}

pub struct ActiveDialogue;

#[derive(Default)]
pub struct DialogueTimer(pub Timer);

pub enum DialogueState {
    Hello,
    Tutorial,
    Arrive,
    EnterHouse,
    Finished,
}

fn dialogue_setup_system(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn dialogue_trigger_system(
    time: Res<Time>,
    mut commands: Commands,
    mut dialogue_state: ResMut<DialogueState>,
    mut dialogue_queue: ResMut<DialogueQueue>,
    mut dialogue_timer: ResMut<DialogueTimer>,
    assets_ready: Res<AssetsReady>,
    santa_query: Query<(&Position, &GroundState), With<Santa>>,
    active_dialogue_query: Query<Entity, With<ActiveDialogue>>,
    indoors_level_query: Query<(), With<IndoorsLevel>>,
) {
    dialogue_timer.0.tick(time.delta());
    let has_active_dialogue = active_dialogue_query.iter().next().is_some();
    let on_ground = santa_query.iter().any(|(position, ground_state)| ground_state.on_ground);
    let position = santa_query.iter().map(|(position, ground_state)| position).next().unwrap();
    let indoors = indoors_level_query.iter().next().is_some();

    match *dialogue_state {
        DialogueState::Hello => {
            if assets_ready.0 && on_ground {
                dialogue_queue.backlog.push_back("hello_1".to_owned());
                dialogue_queue.backlog.push_back("hello_2".to_owned());
                dialogue_queue.backlog.push_back("hello_3".to_owned());
                *dialogue_state = DialogueState::Tutorial;
            }
        }

        DialogueState::Tutorial => {
            if !has_active_dialogue && dialogue_timer.0.elapsed_secs() > 5.0 {
                dialogue_queue.backlog.push_back("tutorial_1".to_owned());
                dialogue_queue.backlog.push_back("tutorial_2".to_owned());
                dialogue_queue.backlog.push_back("tutorial_3".to_owned());
                *dialogue_state = DialogueState::Arrive;
            }
        }

        DialogueState::Arrive => {
            if !has_active_dialogue && position.0.x >= 100.0 && dialogue_timer.0.elapsed_secs() > 1.0 {
                dialogue_queue.backlog.push_back("arrive_1".to_owned());
                *dialogue_state = DialogueState::EnterHouse;
            }
        }

        DialogueState::EnterHouse => {
            if indoors {
                dialogue_queue.backlog.push_back("enter_house_1".to_owned());
                *dialogue_state = DialogueState::Finished;
            }
        }

        DialogueState::Finished => {
            // Done
        }
    }
}

fn dialogue_execution_system(
    mut commands: Commands,
    audio: Res<Audio>,
    keyboard_input: Res<Input<KeyCode>>,
    mut dialogue_queue: ResMut<DialogueQueue>,
    active_dialogue_query: Query<Entity, With<ActiveDialogue>>,
    santa_assets: Res<SantaAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut dialogue_timer: ResMut<DialogueTimer>,
) {
    let next = keyboard_input.just_released(KeyCode::P);
    let mut has_active_dialogue = active_dialogue_query.iter().next().is_some();

    if next && has_active_dialogue {
        for active_dialogue in active_dialogue_query.iter() {
            commands.entity(active_dialogue).despawn_recursive();
        }
        has_active_dialogue = false;
        dialogue_timer.0.reset();
    }

    if !has_active_dialogue {
        if let Some(next_dialogue_key) = dialogue_queue.backlog.pop_front() {
            let speech = santa_assets.speech.get(&next_dialogue_key).unwrap();
            audio.play(speech.audio.clone());
            dialogue_timer.0.reset();

            commands
                .spawn_bundle( NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .insert(ActiveDialogue)
                .with_children(|parent| {
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(90.0), Val::Px(160.0)),
                            justify_content: JustifyContent::Center,
                            align_content: AlignContent::Center,
                            align_self: AlignSelf::Center,
                            ..Default::default()
                        },
                        material: materials.add(Color::ANTIQUE_WHITE.into()),
                        ..Default::default()
                    }).with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            style: Style {
                                align_self: AlignSelf::Center,
                                position_type: PositionType::Absolute,
                                ..Default::default()
                            },
                            // Use the `Text::with_section` constructor
                            text: Text::with_section(
                                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                                format!("{}\nPress <P>", speech.text),
                                TextStyle {
                                    font: santa_assets.font.clone(),
                                    font_size: 64.0,
                                    color: Color::BLACK,
                                },
                                // Note: You can use `Default::default()` in place of the `TextAlignment`
                                TextAlignment {
                                    horizontal: HorizontalAlign::Center,
                                    ..Default::default()
                                },
                            ),
                            ..Default::default()
                        });
                    });
                });
        }
    }
}

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(DialogueState::Hello)
            .insert_resource(DialogueQueue::default())
            .insert_resource(DialogueTimer(Timer::from_seconds(99999999.0, true)))
            .add_system(dialogue_setup_system.system())
            .add_system(dialogue_trigger_system.system())
            .add_system(dialogue_execution_system.system());
    }
}
