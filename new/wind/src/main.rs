mod main_window;
mod settings;

use main_window::MainWindowApp;
use winit::event_loop::EventLoop;

fn main() -> Result<(), winit::error::EventLoopError> {
    let event_loop = EventLoop::new()?;
    let mut app = MainWindowApp::new();
    event_loop.run_app(&mut app)
}
