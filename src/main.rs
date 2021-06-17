use bevy::prelude::*;
use crate::assets::SantaAssetPlugin;
use crate::levels::SantaLevelPlugin;
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use crate::camera::SantaCameraPlugin;
use crate::player::SantaPlayerPlugin;

mod assets;
mod levels;
mod camera;
mod player;

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
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}
