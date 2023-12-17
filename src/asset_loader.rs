use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SpriteAssets {
    pub ghost: Handle<Image>,
    pub knight: Handle<Image>,
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
    };
}
