use bevy::prelude::*;

use crate::plugins::designer::{DesignerEntity, TileButton};
use crate::resources::designer::ImageAssets;


pub fn init_designer(
    mut commands: Commands,
    designer_assets: Res<ImageAssets>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>
) {
    let window = window_query.get_single().unwrap();
    let entity = commands
        .spawn(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            image: UiImage::new(designer_assets.background.clone()),
            ..Default::default()
        })
        .with_children(|parent| {
            // plot_buttons(parent, designer_assets, asset_server, window);
            tile_buttons(parent, designer_assets);
        })
        .id();

    commands.insert_resource(DesignerEntity {
        entity,
    });
}

// fn plot_buttons(root: &mut ChildBuilder, designer_assets: Res<ImageAssets>, asset_server: Res<AssetServer>, window: &Window) {
//     root.spawn()
// }

fn tile_buttons(root: &mut ChildBuilder, designer_assets: Res<ImageAssets>) {
    let hex_buttons = vec![(22.5, 15.0), (42.5, 15.0), (12.5, 37.5), (32.5, 37.5), (52.5, 37.5), (22.5, 60.0), (42.5, 60.0)];
    for btn in hex_buttons {
        root.spawn(ButtonBundle{
            style: Style {
                width: Val::Percent(15.0),
                height: Val::Percent(22.5),
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                align_self: AlignSelf::FlexEnd,
                left: Val::Percent(btn.0 - 5.0),
                right: Val::Auto,
                top: Val::Percent(btn.1),
                bottom: Val::Auto,
                ..Default::default()
            },
            image: UiImage::new(designer_assets.grassland.clone()),
            ..Default::default()
        })
        .insert(TileButton);
    }
}
