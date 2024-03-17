use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SpriteAssets {
    pub tiles: Handle<TextureAtlas>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(
    mut sprite_assets: ResMut<SpriteAssets>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tilemap = asset_server.load("kenney_tiny-dungeon/Tilemap/tilemap.png");
    let atlas = TextureAtlas::from_grid(
        tilemap,
        Vec2::splat(16.0),
        12,
        11,
        Some(Vec2::ONE),
        Some(Vec2::ONE),
    );
    let atlas_handle = texture_atlases.add(atlas);

    *sprite_assets = SpriteAssets {
        tiles: atlas_handle,
    };
}
