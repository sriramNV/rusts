// src/settings.rs

pub struct WindowSettings {
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
    pub font_size: f32,
    pub font_name: &'static str,
    pub theme: Theme,
}

#[derive(Debug, Clone, Copy)]
pub enum Theme {
    Light,
    Dark,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: "Window",
            width: 800,
            height: 600,
            font_size: 16.0,
            font_name: "Arial",
            theme: Theme::Light,
        }
    }
}
