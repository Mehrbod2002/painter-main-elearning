use crate::WindowState;
use dioxus::desktop::tao::{self, event::StartCause};
use dioxus::desktop::UserWindowEvent;
use egui_wgpu::Renderer;
use glyphon::Resolution;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tao::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

pub struct Application {
    pub window_state: Option<WindowState>,
}

pub const DOUBLE_CLICK_THRESHOLD: Duration = Duration::from_millis(500);
pub const DOUBLE_CLICK_DISTANCE: f64 = 5.0;

impl Application {
    pub fn set_window(&mut self, window: Arc<Window>, uri: String) {
        self.window_state =
            Some(pollster::block_on(WindowState::new(window, uri)).expect("unable to create window"));
    }

    pub fn run_app(
        &mut self,
        event: &Event<'_, UserWindowEvent>,
        control_flow: &mut ControlFlow,
        _is_socket_event: bool,
        _socket_event: Option<()>,
    ) {
        let Some(state) = &mut self.window_state else {
            return;
        };

        const CURSOR_BLINK_INTERVAL: f32 = 0.5;
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                state.window.request_redraw();

                if state.start_typing
                    && state.cursor_timer.elapsed().as_secs_f32() >= CURSOR_BLINK_INTERVAL
                {
                    state.cursor_visible = !state.cursor_visible;
                    state.cursor_timer = Instant::now();
                    state.window.request_redraw();
                }

                if state.show_modal_fonts || state.show_modal_colors {
                    state.window.request_redraw();
                }
            }
            Event::WindowEvent {
                event, window_id, ..
            } => {
                if state.window.id() != *window_id {
                    return;
                }
                match event {
                    WindowEvent::CloseRequested => state.window.set_visible(false),
                    _ => {
                        let window = &state.window;
                        state.input(window.clone(), event);
                    }
                }
            }
            Event::Resumed => {
                state
                    .surface
                    .configure(&state.device, &state.surface_config);

                state.egui_renderer =
                    Renderer::new(&state.device, state.surface_config.format, None, 1, true);

                state.window.request_redraw();
            }
            Event::NewEvents(event) => {
                if event == &StartCause::Init {
                    state
                        .surface
                        .configure(&state.device, &state.surface_config);

                    state.egui_renderer =
                        Renderer::new(&state.device, state.surface_config.format, None, 1, true);

                    state.window.request_redraw();
                }
            }
            Event::RedrawRequested(window_id) => {
                if state.window.id() != *window_id {
                    return;
                }
                state.viewport.update(
                    &state.queue,
                    Resolution {
                        width: state.size.width,
                        height: state.size.height,
                    },
                );
                let _ = state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(egui_wgpu::wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(egui_wgpu::wgpu::SurfaceError::OutOfMemory) => {
                        *control_flow = ControlFlow::Exit
                    }
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::LoopDestroyed => *control_flow = ControlFlow::Exit,
            Event::Opened { .. } => {
                state.window.request_redraw();
            }
            _ => (),
        }
    }
}
