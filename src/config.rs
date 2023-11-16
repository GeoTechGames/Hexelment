use std::collections::HashMap;


// Window config
pub const TITLE: &str = "Hexelment";
pub const WINDOW_WIDTH: f32 = 720.0;
pub const WINDOW_HEIGHT: f32 = 360.0;

// Monitor information
pub const MONITOR_HEIGHT: i32 = 1080;
pub const MONITOR_WIDTH: i32 = 1920;

lazy_static! {
    pub static ref TILEMAP: HashMap<&'static str, u32> = {
        let mut i = HashMap::new();
        i.insert("grassland", 0);
        i.insert("woodland", 1);
        i.insert("highland", 2);
        i
    };
}
