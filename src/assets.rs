use bevy::asset::{Asset, AssetPath};
use bevy::prelude::*;
use bevy::sprite::Rect;
use std::collections::HashMap;
use std::ops::DerefMut;

#[derive(Default)]
pub struct AssetsLoading {
    loaded_count: usize,
    error_count: usize,
    remaining: Vec<HandleUntyped>,
}

pub struct AssetsReady(pub bool);

pub struct Speech {
    pub audio: Handle<AudioSource>,
    pub text: String,
}

#[derive(Default)]
pub struct SantaAssets {
    // Fonts
    pub font: Handle<Font>,

    // Speech
    pub speech: HashMap<String, Speech>,

    // Textures
    pub santa: Handle<TextureAtlas>,
    pub snowflakes: Handle<TextureAtlas>,
    pub outside_background: Handle<TextureAtlas>,
    pub indoors_background: Handle<TextureAtlas>,
}

fn load_asset<'a, P: Into<AssetPath<'a>>, R: Asset>(
    server: &Res<AssetServer>,
    loading: &mut ResMut<AssetsLoading>,
    path: P,
) -> Handle<R> {
    let asset = server.load(path);
    loading.remaining.push(asset.clone_untyped());
    asset
}

fn load_speech<'a, P: Into<AssetPath<'a>> + ToString>(
    server: &Res<AssetServer>,
    loading: &mut ResMut<AssetsLoading>,
    assets: &mut SantaAssets,
    path: P,
    text: String,
) {
    let name = path.to_string();
    let name = name
        .split("/")
        .last()
        .unwrap()
        .split(".")
        .next()
        .unwrap()
        .to_owned();

    let audio = load_asset(server, loading, path);
    assets.speech.insert(name, Speech { audio, text });
}

fn load_assets_system(
    server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
) {
    // Textures
    let santa = load_asset(&server, &mut loading, "texture/santa_spritesheet.png");
    let mut santa = TextureAtlas::new_empty(santa, Vec2::new(111.0, 51.0));
    santa.add_texture(Rect {
        min: Vec2::new(0.0, 0.0),
        max: Vec2::new(36.0, 51.0),
    });
    santa.add_texture(Rect {
        min: Vec2::new(36.0, 0.0),
        max: Vec2::new(75.0, 51.0),
    });
    santa.add_texture(Rect {
        min: Vec2::new(75.0, 0.0),
        max: Vec2::new(111.0, 51.0),
    });
    let santa = texture_atlases.add(santa);

    let snowflake = load_asset(&server, &mut loading, "texture/snowflake_spritesheet.png");
    let mut snowflakes = TextureAtlas::new_empty(snowflake, Vec2::new(27.0, 13.0));
    snowflakes.add_texture(Rect {
        min: Vec2::new(0.0, 0.0),
        max: Vec2::new(13.0, 13.0),
    });
    snowflakes.add_texture(Rect {
        min: Vec2::new(13.0, 0.0),
        max: Vec2::new(20.0, 7.0),
    });
    snowflakes.add_texture(Rect {
        min: Vec2::new(20.0, 0.0),
        max: Vec2::new(27.0, 7.0),
    });
    snowflakes.add_texture(Rect {
        min: Vec2::new(13.0, 7.0),
        max: Vec2::new(17.0, 11.0),
    });
    let snowflakes = texture_atlases.add(snowflakes);

    let outside_background = load_asset(
        &server,
        &mut loading,
        "texture/background_outside_spritesheet.png",
    );
    let outside_background =
        TextureAtlas::from_grid(outside_background, Vec2::new(540.0, 210.0), 1, 1);
    let outside_background = texture_atlases.add(outside_background);

    let indoors_background = load_asset(&server, &mut loading, "texture/background_indoors.png");
    let indoors_background =
        TextureAtlas::from_grid(indoors_background, Vec2::new(540.0, 210.0), 1, 1);
    let indoors_background = texture_atlases.add(indoors_background);

    let mut assets = SantaAssets {
        // Fonts
        font: load_asset(&server, &mut loading, "font/square.ttf"),

        // Speech
        speech: Default::default(),

        // Textures
        santa,
        snowflakes,
        outside_background,
        indoors_background,
    };

    // Speech
    load_speech(
        &server,
        &mut loading,
        &mut assets,
        "speech/arrive_1.ogg",
        "You found the door! Press <F> when being close to enter the house!".to_owned(),
    );
    load_speech(
        &server,
        &mut loading,
        &mut assets,
        "speech/enter_house_1.ogg",
        "You are entering the house!".to_owned(),
    );
    load_speech(
        &server,
        &mut loading,
        &mut assets,
        "speech/hello_1.ogg",
        "Hello, I'm Santa!".to_owned(),
    );
    load_speech(
        &server,
        &mut loading,
        &mut assets,
        "speech/hello_2.ogg",
        "Help me distribute all the presents!".to_owned(),
    );
    load_speech(
        &server,
        &mut loading,
        &mut assets,
        "speech/hello_3.ogg",
        "And do not unwrap them yourself!".to_owned(),
    );
    load_speech(
        &server,
        &mut loading,
        &mut assets,
        "speech/tutorial_1.ogg",
        "But first, you have to walk to the right.".to_owned(),
    );
    load_speech(
        &server,
        &mut loading,
        &mut assets,
        "speech/tutorial_2.ogg",
        "To do that, press <D> on your keyboard.".to_owned(),
    );
    load_speech(
        &server,
        &mut loading,
        &mut assets,
        "speech/tutorial_3.ogg",
        "Do it now!".to_owned(),
    );

    commands.insert_resource(assets);
    commands.insert_resource(AssetsReady(false));
}

fn check_assets_ready_system(
    server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    mut assets_ready: ResMut<AssetsReady>,
) {
    use bevy::asset::LoadState;

    let AssetsLoading {
        loaded_count,
        error_count,
        remaining,
    } = &mut loading.deref_mut();

    let mut has_changed = false;
    remaining.retain(|handle| match server.get_load_state(handle) {
        LoadState::Failed => {
            error!("Could not load asset {:?}", server.get_handle_path(handle));
            *error_count += 1;
            has_changed = true;
            false
        }
        LoadState::Loaded => {
            *loaded_count += 1;
            has_changed = true;
            false
        }
        _ => true,
    });

    if remaining.is_empty() && has_changed {
        if *error_count == 0 {
            info!("Loaded all {} assets successfully", loaded_count);
        } else {
            error!(
                "Finished loading {} assets with {} errors",
                *loaded_count + *error_count,
                error_count
            )
        }

        assets_ready.0 = true;
    }
}

pub struct SantaAssetPlugin;

impl Plugin for SantaAssetPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<AssetsLoading>()
            .add_startup_stage_before(StartupStage::Startup, "load_assets", SystemStage::parallel().with_system(load_assets_system
                .system()
                .label("load_assets")
                .before("init_santa")))
            .add_system(
                check_assets_ready_system
                    .system()
                    .label("check_assets_ready"),
            );
    }
}
