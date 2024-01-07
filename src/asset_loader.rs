use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SpriteAssets {
    pub ghost: Handle<Image>,
    pub knight: Handle<Image>,
    pub dagger: Handle<Image>,
    pub wall: WallSprites,
}

#[derive(Debug, Default)]
pub struct WallSprites {
    pub hwall_top_left: Handle<Image>,
    pub hwall_top_right: Handle<Image>,
    pub hwall_top_mid: Handle<Image>,
    pub hwall_face: Handle<Image>,
    pub vwall_left: Handle<Image>,
    pub vwall_right: Handle<Image>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut sprite_assets: ResMut<SpriteAssets>, asset_server: Res<AssetServer>) {
    *sprite_assets = SpriteAssets {
        ghost: asset_server.load("kenney_tiny-dungeon/Tiles/tile_0121.png"),
        knight: asset_server.load("kenney_tiny-dungeon/Tiles/tile_0097.png"),
        dagger: asset_server.load("kenney_tiny-dungeon/Tiles/tile_0103.png"),
        wall: WallSprites {
            hwall_top_left: asset_server.load("kenney_tiny-dungeon/Tiles/tile_0001.png"),
            hwall_top_right: asset_server.load("kenney_tiny-dungeon/Tiles/tile_0003.png"),
            hwall_top_mid: asset_server.load("kenney_tiny-dungeon/Tiles/tile_0006.png"),
            hwall_face: asset_server.load("kenney_tiny-dungeon/Tiles/tile_0040.png"),
            vwall_right: asset_server.load("kenney_tiny-dungeon/Tiles/tile_0015.png"),
            vwall_left: asset_server.load("kenney_tiny-dungeon/Tiles/tile_0013.png"),
        },
    };
}
