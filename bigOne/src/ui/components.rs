// src/ui/components.rs
use bevy::{
    prelude::*,
    ui::{FocusPolicy, Style, UiRect},
    window::PrimaryWindow,
    input::mouse::{MouseButton, MouseButtonInput},
};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// ==================== Composants de base ====================

/// Définition des composants UI supportés dans notre DSL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UIComponent {
    Window(WindowProps),
    Button(ButtonProps),
    Text(TextProps),
    Image(ImageProps),
    Canvas(CanvasProps),
    SVG(SVGProps),
    ScrollView(ScrollViewProps),
    List(ListProps),
    Grid(GridProps),
    Input(InputProps),
    Viewport3D(Viewport3DProps),
    Stack(StackProps),
    Row(RowProps),
    Column(ColumnProps),
}

// ==================== Propriétés des composants ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowProps {
    pub id: String,
    pub title: String,
    pub width: f32,
    pub height: f32,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub resizable: Option<bool>,
    pub draggable: Option<bool>,
    pub children: Vec<UIComponent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonProps {
    pub id: String,
    pub text: String,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub on_click: Option<String>,
    pub icon: Option<String>,
    pub style: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextProps {
    pub id: String,
    pub content: String,
    pub size: Option<f32>,
    pub color: Option<String>,
    pub align: Option<String>,
    pub font: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageProps {
    pub id: String,
    pub source: String,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub scale: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasProps {
    pub id: String,
    pub width: f32,
    pub height: f32,
    pub on_draw: Option<String>,
    pub on_click: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SVGProps {
    pub id: String,
    pub source: String,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollViewProps {
    pub id: String,
    pub width: f32,
    pub height: f32,
    pub children: Vec<UIComponent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListProps {
    pub id: String,
    pub items: Vec<UIComponent>,
    pub direction: Option<String>, // "vertical" or "horizontal"
    pub spacing: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridProps {
    pub id: String,
    pub columns: usize,
    pub rows: Option<usize>,
    pub items: Vec<UIComponent>,
    pub spacing: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputProps {
    pub id: String,
    pub placeholder: Option<String>,
    pub value: Option<String>,
    pub width: Option<f32>,
    pub on_change: Option<String>,
    pub on_submit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Viewport3DProps {
    pub id: String,
    pub width: f32,
    pub height: f32,
    pub scene: Option<String>,
    pub camera: Option<CameraProps>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraProps {
    pub position: [f32; 3],
    pub target: [f32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackProps {
    pub id: String,
    pub children: Vec<UIComponent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RowProps {
    pub id: String,
    pub children: Vec<UIComponent>,
    pub spacing: Option<f32>,
    pub align: Option<String>, // "start", "center", "end", "spaceBetween", "spaceAround"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnProps {
    pub id: String,
    pub children: Vec<UIComponent>,
    pub spacing: Option<f32>,
    pub align: Option<String>, // "start", "center", "end", "spaceBetween", "spaceAround"
}

// ==================== Composants Bevy ====================

/// Marqueur pour les composants créés par notre système UI
#[derive(Component)]
pub struct UIElement {
    pub id: String,
    pub component_type: String,
}

/// Marqueur pour les fenêtres draggables
#[derive(Component)]
pub struct Draggable {
    pub dragging: bool,
    pub offset: Vec2,
}

/// Marqueur pour les fenêtres redimensionnables
#[derive(Component)]
pub struct Resizable {
    pub resizing: bool,
    pub min_size: Vec2,
    pub edge: ResizeEdge,
}

/// Bords pour le redimensionnement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeEdge {
    None,
    Top,
    Right,
    Bottom,
    Left,
    TopRight,
    BottomRight,
    BottomLeft,
    TopLeft,
}

/// Stockage des callbacks pour les événements
#[derive(Component)]
pub struct EventCallback {
    pub event_type: String, // "click", "change", "drag", etc.
    pub callback: String,    // Nom de la fonction Ruby
}

// ==================== Systèmes ====================

pub fn setup_ui_systems(app: &mut App) {
    app
        .add_systems(Update, (
            window_drag_system,
            window_resize_system,
            button_click_system,
            update_canvas_system,
            handle_input_changes,
        ));
}

// Système pour gérer le drag & drop des fenêtres
fn window_drag_system(
    mut windows: Query<(&mut Style, &mut Draggable, &UIElement)>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mouse_position: Res<Input<MouseButton>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    // Si la souris est enfoncée, on commence le dragging
    for event in mouse_button_input_events.iter() {
        if event.button != MouseButton::Left {
            continue;
        }

        let pressed = event.state.is_pressed();
        for (mut style, mut draggable, _) in windows.iter_mut() {
            if pressed {
                if let Ok(window) = primary_window.get_single() {
                    if let Some(position) = window.cursor_position() {
                        // Vérifier si le clic est dans la fenêtre
                        if let (Some(left), Some(top), Some(width), Some(height)) = (
                            style.left.try_extract_pixels(),
                            style.top.try_extract_pixels(),
                            style.width.try_extract_pixels(),
                            style.height.try_extract_pixels(),
                        ) {
                            if position.x >= left
                                && position.x <= left + width
                                && position.y >= top
                                && position.y <= top + 30.0 // Hauteur de l'en-tête
                            {
                                draggable.dragging = true;
                                draggable.offset = Vec2::new(position.x - left, position.y - top);
                            }
                        }
                    }
                }
            } else {
                draggable.dragging = false;
            }
        }
    }

    // Si le drag est en cours, on déplace la fenêtre
    if mouse_position.pressed(MouseButton::Left) {
        if let Ok(window) = primary_window.get_single() {
            if let Some(position) = window.cursor_position() {
                for (mut style, draggable, _) in windows.iter_mut() {
                    if draggable.dragging {
                        style.left = Val::Px(position.x - draggable.offset.x);
                        style.top = Val::Px(position.y - draggable.offset.y);
                    }
                }
            }
        }
    }
}

// Système pour gérer le redimensionnement des fenêtres
fn window_resize_system(
    mut windows: Query<(&mut Style, &mut Resizable, &UIElement)>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mouse_position: Res<Input<MouseButton>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    // Implémentation similaire au système de drag pour le redimensionnement
    // ...
}

// Système pour gérer les clics sur les boutons
fn button_click_system(
    buttons: Query<(&UIElement, &EventCallback), With<Button>>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut ruby_event_writer: EventWriter<RubyCallbackEvent>,
) {
    for event in mouse_button_input_events.iter() {
        if event.button == MouseButton::Left && event.state.is_pressed() {
            // Détecter si un bouton a été cliqué et déclencher le callback Ruby
            // ...
        }
    }
}

// Système pour mettre à jour les canvas à partir du code Ruby
fn update_canvas_system(
    // Implémentation du système de canvas
    // ...
) {
}

// Système pour gérer les changements dans les champs de texte
fn handle_input_changes(
    // Implémentation du système de gestion des inputs
    // ...
) {
}

// ==================== Événements ====================

/// Événement pour déclencher un callback Ruby
#[derive(Event)]
pub struct RubyCallbackEvent {
    pub callback: String,
    pub arguments: HashMap<String, String>,
}

/// Événement pour mettre à jour un composant UI
#[derive(Event)]
pub struct UIUpdateEvent {
    pub id: String,
    pub update: UIComponentUpdate,
}

/// Types de mises à jour UI
#[derive(Debug, Clone)]
pub enum UIComponentUpdate {
    SetText(String),
    SetImage(String),
    SetPosition(f32, f32),
    SetSize(f32, f32),
    SetVisible(bool),
    SetStyle(HashMap<String, String>),
    Replace(UIComponent),
}