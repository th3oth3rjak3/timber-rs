use bevy::prelude::*;

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

// Load the background at startup
pub fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = GameAssets {
        axe: asset_server.load("graphics/axe.png"),
        background: asset_server.load("graphics/background.png"),
        bee: asset_server.load("graphics/bee.png"),
        branch: asset_server.load("graphics/branch.png"),
        cloud: asset_server.load("graphics/cloud.png"),
        log: asset_server.load("graphics/log.png"),
        player: asset_server.load("graphics/player.png"),
        headstone: asset_server.load("graphics/rip.png"),
        tree: asset_server.load("graphics/tree.png"),
        tree_alt: asset_server.load("graphics/tree2.png"),
        death: asset_server.load("sounds/death.wav"),
        out_of_time: asset_server.load("sounds/out_of_time.wav"),
        chop: asset_server.load("sounds/chop.wav"),
        default_font: asset_server.load("fonts/KOMIKAP_.ttf"),
    };

    commands.insert_resource(assets);
}
