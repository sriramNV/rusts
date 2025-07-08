// src/main_window.rs
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::settings::WindowSettings;

pub struct MainWindowApp {
    pub window: Option<Window>,
    pub settings: WindowSettings,
}

impl MainWindowApp {
    pub fn new() -> Self {
        Self {
            window: None,
            settings: WindowSettings::default(),
        }
    }
}

impl ApplicationHandler for MainWindowApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = winit::window::Window::default_attributes()
            .with_title(self.settings.title)
            .with_inner_size(winit::dpi::PhysicalSize::new(self.settings.width, self.settings.height));
        self.window = Some(event_loop.create_window(attrs).unwrap());
        // Later you can use settings.font_size, theme, etc for UI rendering
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(win) = &self.window {
            if win.id() == window_id {
                if let WindowEvent::CloseRequested = event {
                    event_loop.exit();
                }
            }
        }
    }
}
