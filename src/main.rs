use crate::assets::SantaAssetPlugin;
use crate::camera::SantaCameraPlugin;
use crate::levels::SantaLevelPlugin;
use crate::physics::SantaPhysicsPlugin;
use crate::player::SantaPlayerPlugin;
use crate::render::SantaRenderPlugin;
use bevy::prelude::*;
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

mod assets;
mod camera;
mod levels;
mod physics;
mod player;
mod render;

const TIME_STEP_F64: f64 = 1.0 / 60.0;
const TIME_STEP: f32 = 1.0 / 60.0;

fn santa_init_system(mut commands: Commands) {}

pub struct SantaInitPlugin;

impl Plugin for SantaInitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(santa_init_system.system());
    }
}

fn main() {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Always,
    )
    .unwrap();

    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(SantaInitPlugin)
        .add_plugin(SantaAssetPlugin)
        .add_plugin(SantaCameraPlugin)
        .add_plugin(SantaLevelPlugin)
        .add_plugin(SantaPlayerPlugin)
        .add_plugin(SantaPhysicsPlugin)
        .add_plugin(SantaRenderPlugin)
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}
