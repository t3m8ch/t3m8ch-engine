#![forbid(unsafe_code)]

use tracing::{error, info};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

#[derive(Default)]
struct Application {
    window: Option<Window>,
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("application resumed");

        if self.window.is_some() {
            return;
        }

        match event_loop.create_window(WindowAttributes::default().with_title("t3m8ch engine")) {
            Ok(window) => {
                info!(window_id = ?window.id(), "window created");
                self.window = Some(window);
            }
            Err(error) => {
                error!(%error, "failed to create window");
                event_loop.exit();
            }
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        info!("application suspended");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let is_application_window = self
            .window
            .as_ref()
            .is_some_and(|window| window.id() == window_id);

        match event {
            WindowEvent::CloseRequested if is_application_window => {
                info!(?window_id, "window close requested");
                event_loop.exit();
            }
            WindowEvent::Resized(size)
                if is_application_window && (size.width == 0 || size.height == 0) =>
            {
                info!(
                    ?window_id,
                    width = size.width,
                    height = size.height,
                    "window minimized or resized to zero"
                );
            }
            WindowEvent::Resized(size) if is_application_window => {
                info!(
                    ?window_id,
                    width = size.width,
                    height = size.height,
                    "window resized"
                );
            }
            WindowEvent::Occluded(occluded) if is_application_window => {
                info!(?window_id, occluded, "window occlusion changed");
            }
            _ => {}
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        info!("event loop exiting");
    }
}

fn main() -> Result<(), winit::error::EventLoopError> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut Application::default())
}
