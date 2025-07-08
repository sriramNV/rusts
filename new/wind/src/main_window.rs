// src/main_window.rs
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

pub struct MainWindowApp {
    pub window: Option<Window>,
}

impl MainWindowApp {
    pub fn new() -> Self {
        Self { window: None }
    }
}

impl ApplicationHandler for MainWindowApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = Window::default_attributes().with_title("Window");
        self.window = Some(event_loop.create_window(attrs).unwrap());
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
