use macroquad::color;

/* CAMERA SETTINGS */
pub const PAN_VEL: f32 = 100.;
pub const ZOOM_CHG_VEL: f32 = 1.;
pub const MAX_ZOOM: f32 = 10.;
pub const MIN_ZOOM: f32 = 0.01;

/* GUI SETTINGS */
pub const DEF_GUI_TXT_MAX_WIDTH: f32 = 200.;
pub const DEF_GUI_FONT_SZ: u16 = 20;
pub const DEF_GUI_TXT_RECT_COLOR: color::Color = color::Color::new(0.5, 0.5, 0.5, 0.5);
pub const GUI_MINITEXT_FONT_SZ: u16 = 16;
pub const GUI_MINITEXT_MAX_WIDTH: f32 = 100.;

/* OTHER */
pub const BG_COLOR: color::Color = color::LIGHTGRAY;
// TODO find how to disable vsync to make this work
pub const FRAME_TIME_MILIS: f32 = 16.0;
