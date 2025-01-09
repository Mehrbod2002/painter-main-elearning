use std::{collections::HashSet, sync::Arc, time::Instant};

use dioxus::desktop::{
    tao::{keyboard::Key, window::Window},
    wry::dpi::{PhysicalPosition, PhysicalSize},
};
use egui::{Context, ImageSource, RawInput};
use egui_wgpu::Renderer;
use glyphon::{FontSystem, SwashCache};
use stream::manager::GrpcConnectionManager;
use wgpu::SurfaceConfiguration;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

#[derive(Clone, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct Rectangle {
    pub first: [f32; 2],
    pub last: [f32; 2],
    pub color: [f32; 4],
}

impl Rectangle {
    pub fn to_vertices(self) -> Vec<Vertex> {
        let (x1, y1) = (self.first[0], self.first[1]);
        let (x2, y2) = (self.last[0], self.last[1]);

        vec![
            Vertex {
                position: [x1, y2],
                color: self.color,
            },
            Vertex {
                position: [x2, y2],
                color: self.color,
            },
            Vertex {
                position: [x2, y2],
                color: self.color,
            },
            Vertex {
                position: [x2, y1],
                color: self.color,
            },
            Vertex {
                position: [x2, y1],
                color: self.color,
            },
            Vertex {
                position: [x1, y1],
                color: self.color,
            },
            Vertex {
                position: [x1, y1],
                color: self.color,
            },
            Vertex {
                position: [x1, y2],
                color: self.color,
            },
        ]
    }
}

#[derive(Debug, Clone)]
pub struct TextEntries {
    pub position: [f32; 2],
    pub color: [u8; 4],
    pub text: String,
    pub pending: bool,
    pub bounds: Rect,
    pub font_size: i32,
}

impl TextEntries {
    pub fn null(color: [u8; 4], font_size: i32) -> Self {
        TextEntries {
            font_size,
            position: [0.0, 0.0],
            color,
            text: String::new(),
            pending: true,
            bounds: Rect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub enum ActionType {
    Stroke(Vec<Vertex>),
    Text(TextEntries),
    Shapes(Rectangle),
}

#[derive(Clone, Debug)]
pub struct Action {
    pub id: uuid::Uuid,
    pub action_type: ActionType,
}

pub struct WindowState {
    pub device: egui_wgpu::wgpu::Device,
    pub pressed_keys: HashSet<Key<'static>>,
    pub queue: egui_wgpu::wgpu::Queue,
    pub show_modal_fonts: bool,
    pub font_size: i32,
    pub undo_button: bool,
    pub events_id: HashSet<uuid::Uuid>,
    pub show_modal_colors: bool,
    pub surface: egui_wgpu::wgpu::Surface<'static>,
    pub surface_config: SurfaceConfiguration,
    pub last_cursor_position: PhysicalPosition<f64>,
    pub actions: Vec<Action>,
    pub scale_factor: f64,
    pub egui_renderer: Renderer,
    pub raw_input: RawInput,
    pub egui_context: Context,
    pub size: PhysicalSize<u32>,

    pub font_system: FontSystem,
    pub swash_cache: SwashCache,
    pub viewport: glyphon::Viewport,
    pub texts: Vec<TextEntries>,
    pub atlas: glyphon::TextAtlas,
    pub text_renderer: glyphon::TextRenderer,
    pub window: Arc<Window>,

    pub mouse_pressed: bool,
    pub strokes: Vec<Vec<Vertex>>,
    pub current_stroke: Vec<Vertex>,
    pub current_color: [f32; 4],

    pub render_pipeline: egui_wgpu::wgpu::RenderPipeline,
    pub rectangle_shader: Option<egui_wgpu::wgpu::RenderPipeline>,
    pub vertex_buffer: egui_wgpu::wgpu::Buffer,
    pub start_typing: bool,
    pub shape_positions: Vec<Vertex>,
    pub shapes: Vec<Rectangle>,
    pub create_rect: bool,
    pub cursor_visible: bool,
    pub cursor_timer: Instant,
    pub last_click_time: Option<Instant>,
    pub last_click_position: Option<PhysicalPosition<f64>>,
    pub editing_text_index: Option<usize>,

    pub color: ImageSource<'static>,
    pub rect: ImageSource<'static>,
    pub prev: ImageSource<'static>,
    pub font: ImageSource<'static>,

    pub stream_client: Arc<GrpcConnectionManager>,

    pub actions_changed: bool,
}
