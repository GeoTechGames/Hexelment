use bevy::prelude::*;

use crate::plugins::designer::{HoveredTile, SelectedTile, TileButton};


pub fn tile_button_handle_system(
    mut button_query: Query<
        (&Interaction, &TileButton, &Style),
        (Changed<Interaction>, With<Button>),
    >,
    mut hovered_tile_query: Query<&mut Style, (With<HoveredTile>, Without<Button>, Without<SelectedTile>)>,
    mut selected_tile_query: Query<&mut Style, (With<SelectedTile>, Without<Button>, Without<HoveredTile>)>
) {
    for (interaction, _tile, btn) in button_query.iter_mut() {
        match *interaction {
            Interaction::None => {
                let mut selected_tile = hovered_tile_query.get_single_mut().unwrap();
                selected_tile.left = Val::Percent(-100.0);
                selected_tile.top = Val::Percent(-100.0);
            },
            Interaction::Hovered => {
                let mut selected_tile = hovered_tile_query.get_single_mut().unwrap();
                selected_tile.left = btn.left;
                selected_tile.top = btn.top;
            },
            Interaction::Pressed => {
                let mut selected_tile = selected_tile_query.get_single_mut().unwrap();
                selected_tile.left = btn.left;
                selected_tile.top = btn.top;
            },
        }
    }
}
