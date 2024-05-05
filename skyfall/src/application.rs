use std::sync::Arc;
use tracing::info;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};
use crate::WgpuState;

#[derive(Default)]
pub struct App {
    pub window: Option<Arc<Window>>,
    pub wgpu_state: Option<WgpuState>
}

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
            },
            _ => (),
        }
    }
}
