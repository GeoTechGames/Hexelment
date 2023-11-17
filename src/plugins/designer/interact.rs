use bevy::prelude::*;

use crate::plugins::designer::TileButton;
use crate::resources::designer::ImageAssets;

pub fn tile_button_handle_system(
    mut commands: Commands,
    mut button_query: Query<
        (&Interaction, &TileButton, &Style),
        (Changed<Interaction>, With<Button>),
    >,
    designer_assets: Res<ImageAssets>,
) {
    for (interaction, tile, btn) in button_query.iter_mut() {
        match *interaction {
            Interaction::None => (),
            Interaction::Hovered => {
                commands.spawn(ImageBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: btn.width,
                        height: btn.height,
                        left: btn.left,
                        top: btn.top,
                        ..Default::default()
                    },
                    image: UiImage::new(designer_assets.highlighted.clone()),
                    z_index: ZIndex::Local(10),
                    ..Default::default()
                });
            },
            Interaction::Pressed => (),
        }
    }
}
