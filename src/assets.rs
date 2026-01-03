use bevy::{
    asset::{embedded_asset, load_embedded_asset},
    prelude::*,
};

pub struct EmbeddedAssetPlugin;

impl Plugin for EmbeddedAssetPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "../assets/fonts/KOMIKAP_.ttf");
        embedded_asset!(app, "../assets/graphics/axe.png");
        embedded_asset!(app, "../assets/graphics/background.png");
        embedded_asset!(app, "../assets/graphics/bee.png");
        embedded_asset!(app, "../assets/graphics/branch.png");
        embedded_asset!(app, "../assets/graphics/cloud.png");
        embedded_asset!(app, "../assets/graphics/log.png");
        embedded_asset!(app, "../assets/graphics/player.png");
        embedded_asset!(app, "../assets/graphics/rip.png");
        embedded_asset!(app, "../assets/graphics/tree.png");
        embedded_asset!(app, "../assets/graphics/tree2.png");
        embedded_asset!(app, "../assets/sounds/chop.wav");
        embedded_asset!(app, "../assets/sounds/death.wav");
        embedded_asset!(app, "../assets/sounds/out_of_time.wav");
    }
}

#[derive(Resource, Clone)]
pub struct GameAssets {
    // Textures
    pub axe: Handle<Image>,
    pub background: Handle<Image>,
    pub bee: Handle<Image>,
    pub branch: Handle<Image>,
    pub cloud: Handle<Image>,
    pub log: Handle<Image>,
    pub player: Handle<Image>,
    pub headstone: Handle<Image>,
    pub tree: Handle<Image>,
    pub tree_alt: Handle<Image>,

    // Sounds
    pub death: Handle<AudioSource>,
    pub out_of_time: Handle<AudioSource>,
    pub chop: Handle<AudioSource>,

    // Fonts
    pub default_font: Handle<Font>,
}

// Load the assets at startup
pub fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = GameAssets {
        axe: load_embedded_asset!(&*asset_server, "../assets/graphics/axe.png"),
        background: load_embedded_asset!(&*asset_server, "../assets/graphics/background.png"),
        bee: load_embedded_asset!(&*asset_server, "../assets/graphics/bee.png"),
        branch: load_embedded_asset!(&*asset_server, "../assets/graphics/branch.png"),
        cloud: load_embedded_asset!(&*asset_server, "../assets/graphics/cloud.png"),
        log: load_embedded_asset!(&*asset_server, "../assets/graphics/log.png"),
        player: load_embedded_asset!(&*asset_server, "../assets/graphics/player.png"),
        headstone: load_embedded_asset!(&*asset_server, "../assets/graphics/rip.png"),
        tree: load_embedded_asset!(&*asset_server, "../assets/graphics/tree.png"),
        tree_alt: load_embedded_asset!(&*asset_server, "../assets/graphics/tree2.png"),
        death: load_embedded_asset!(&*asset_server, "../assets/sounds/death.wav"),
        out_of_time: load_embedded_asset!(&*asset_server, "../assets/sounds/out_of_time.wav"),
        chop: load_embedded_asset!(&*asset_server, "../assets/sounds/chop.wav"),
        default_font: load_embedded_asset!(&*asset_server, "../assets/fonts/KOMIKAP_.ttf"),
    };

    commands.insert_resource(assets);
}
