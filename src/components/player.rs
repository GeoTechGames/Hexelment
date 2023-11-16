use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerComponent {
    pub speed: f32,
    pub cereals: i64,
    pub vegetables: i64,
    pub livestock: i64,
    pub timber: i64,
    pub clay: i64,
    pub metals: i64,
    pub charcoal: i64,
    pub basic_pottery: i64,
    pub fine_pottery: i64,
    pub metal_goods: i64
}

// impl PlayerComponent {

//     pub fn new() -> Self {
//         PlayerComponent {
//             speed: 100.0,
//             cereals: 0,
//             vegetables: 0,
//             livestock: 0,
//             timber: 0,
//             clay: 0,
//             metals: 0,
//             charcoal: 0,
//             basic_pottery: 0,
//             fine_pottery: 0,
//             metal_goods: 0
//         }
//     }

// }
