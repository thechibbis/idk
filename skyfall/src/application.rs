use crate::WgpuState;
use std::sync::Arc;
use tracing::{error, info};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    wgpu_state: Option<WgpuState>,
}

impl App {
    pub async fn run() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();

        let event_loop = EventLoop::new()?;

        let mut app = App::default();

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}

// winit App Handler
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let win_attributes = Window::default_attributes().with_title("Skyfall Engine");
        let window = Arc::new(event_loop.create_window(win_attributes).unwrap());

        let wgpu_state = pollster::block_on(WgpuState::new(Arc::clone(&window))).unwrap();
        window.request_redraw();

        self.window = Some(window);
        self.wgpu_state = Some(wgpu_state);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let wgpu_state = self.wgpu_state.as_mut().unwrap();
        if id == wgpu_state.window.id() && !wgpu_state.input(&event) {
            match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                    ..
                } => {
                    info!("The close button was pressed; stopping");
                    event_loop.exit();
                }
                WindowEvent::Resized(physical_size) => wgpu_state.resize(physical_size),
                WindowEvent::RedrawRequested => {
                    wgpu_state.window.request_redraw();
                    wgpu_state.update();
                    match wgpu_state.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => wgpu_state.resize(wgpu_state.size),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => error!("{:?}", e),
                    }
                }
                _ => (),
            }
        }
    }
}
