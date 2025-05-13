// src/ui/systems.rs
use bevy::{
    prelude::*,
    input::mouse::{MouseButton, MouseButtonInput},
    window::PrimaryWindow,
};
use std::collections::HashMap;

use crate::ui::components::*;
use crate::ui::builder::base::UIBuilder;
use crate::dsl::parser::DSLParser;

/// Plugin pour l'interface utilisateur DSL
pub struct UIDSLPlugin;

impl Plugin for UIDSLPlugin {
    fn build(&self, app: &mut App) {
        // Enregistrer les ressources nécessaires
        app.init_resource::<UIDSLState>()
            .init_resource::<UIHotReload>();

        // Enregistrer les événements personnalisés
        app.add_event::<RubyCallbackEvent>()
            .add_event::<UIUpdateEvent>();

        // Ajouter les systèmes
        app.add_systems(Update, (
            window_drag_system,
            window_resize_system,
            button_click_system,
            input_interaction_system,
            handle_ui_updates,
            handle_ruby_callbacks,
            hot_reload_system,
        ));
    }
}

/// État global de l'interface DSL
#[derive(Resource)]
pub struct UIDSLState {
    /// Constructeur d'UI
    pub builder: UIBuilder,
    /// Parser DSL
    pub parser: Option<DSLParser>,
    /// État de la dernière évaluation
    pub last_eval: Option<DSLEvaluationResult>,
}

impl Default for UIDSLState {
    fn default() -> Self {
        Self {
            builder: UIBuilder::new(),
            parser: None,
            last_eval: None,
        }
    }
}

/// Ressource pour le rechargement à chaud
#[derive(Resource)]
pub struct UIHotReload {
    /// Est-ce que le rechargement à chaud est activé
    pub enabled: bool,
    /// Chemin du script principal
    pub script_path: String,
    /// Dernier timestamp de modification
    pub last_modified: std::time::SystemTime,
}

impl Default for UIHotReload {
    fn default() -> Self {
        Self {
            enabled: true,
            script_path: "scripts/ui.rb".to_string(),
            last_modified: std::time::SystemTime::now(),
        }
    }
}

/// Initialise le système DSL avec un script
pub fn initialize_dsl(
    mut commands: Commands,
    mut dsl_state: ResMut<UIDSLState>,
    asset_server: Res<AssetServer>,
    script: &str,
) -> Result<(), String> {
    // Créer le parser DSL s'il n'existe pas déjà
    if dsl_state.parser.is_none() {
        let mut parser = DSLParser::new()?;
        parser.initialize_dsl()?;
        dsl_state.parser = Some(parser);
    }

    // Évaluer le script DSL
    if let Some(parser) = &mut dsl_state.parser {
        let eval_result = parser.evaluate_dsl(script)?;
        
        // S'il y a des erreurs, les renvoyer
        if !eval_result.errors.is_empty() {
            return Err(eval_result.errors.join("\n"));
        }
        
        // Construire l'UI à partir des composants générés
        dsl_state.builder.build_ui(&mut commands, &eval_result.components, &asset_server);
        
        // Sauvegarder le résultat d'évaluation
        dsl_state.last_eval = Some(eval_result);
    }

    Ok(())
}

/// Système pour la fonctionnalité de hot-reload
fn hot_reload_system(
    mut commands: Commands,
    mut hot_reload: ResMut<UIHotReload>,
    mut dsl_state: ResMut<UIDSLState>,
    asset_server: Res<AssetServer>,
    windows: Query<Entity, With<UIElement>>,
) {
    if !hot_reload.enabled {
        return;
    }

    // Vérifier si le fichier a été modifié
    if let Ok(metadata) = std::fs::metadata(&hot_reload.script_path) {
        if let Ok(modified) = metadata.modified() {
            // Si le fichier a été modifié depuis la dernière vérification
            if modified > hot_reload.last_modified {
                hot_reload.last_modified = modified;
                
                // Lire le contenu du fichier
                if let Ok(script) = std::fs::read_to_string(&hot_reload.script_path) {
                    // Despawn toutes les entités UI existantes
                    for entity in windows.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                    
                    // Tenter de réinitialiser l'UI avec le nouveau script
                    if let Err(error) = initialize_dsl(commands.reborrow(), dsl_state.reborrow(), asset_server.clone(), &script) {
                        error!("Erreur lors du rechargement DSL: {}", error);
                    } else {
                        info!("UI rechargée depuis {}", hot_reload.script_path);
                    }
                }
            }
        }
    }
}

/// Système pour gérer le drag & drop des fenêtres
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
                        // Vérifier si le clic est dans l'en-tête de la fenêtre
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

/// Système pour gérer le redimensionnement des fenêtres
fn window_resize_system(
    mut windows: Query<(&mut Style, &mut Resizable, &UIElement)>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mouse_position: Res<Input<MouseButton>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    // Marge de détection pour les bords
    const EDGE_MARGIN: f32 = 5.0;

    // Détecter l'activation du redimensionnement
    for event in mouse_button_input_events.iter() {
        if event.button != MouseButton::Left {
            continue;
        }

        let pressed = event.state.is_pressed();
        for (mut style, mut resizable, _) in windows.iter_mut() {
            if pressed && !resizable.resizing {
                if let Ok(window) = primary_window.get_single() {
                    if let Some(cursor_pos) = window.cursor_position() {
                        // Vérifier si le clic est sur un bord de la fenêtre
                        if let (Some(left), Some(top), Some(width), Some(height)) = (
                            style.left.try_extract_pixels(),
                            style.top.try_extract_pixels(),
                            style.width.try_extract_pixels(),
                            style.height.try_extract_pixels(),
                        ) {
                            let right = left + width;
                            let bottom = top + height;
                            
                            // Déterminer quel bord est cliqué
                            let on_left = (cursor_pos.x - left).abs() <= EDGE_MARGIN;
                            let on_right = (cursor_pos.x - right).abs() <= EDGE_MARGIN;
                            let on_top = (cursor_pos.y - top).abs() <= EDGE_MARGIN;
                            let on_bottom = (cursor_pos.y - bottom).abs() <= EDGE_MARGIN;
                            
                            let edge = match (on_top, on_right, on_bottom, on_left) {
                                (true, true, false, false) => ResizeEdge::TopRight,
                                (false, true, true, false) => ResizeEdge::BottomRight,
                                (false, false, true, true) => ResizeEdge::BottomLeft,
                                (true, false, false, true) => ResizeEdge::TopLeft,
                                (true, false, false, false) => ResizeEdge::Top,
                                (false, true, false, false) => ResizeEdge::Right,
                                (false, false, true, false) => ResizeEdge::Bottom,
                                (false, false, false, true) => ResizeEdge::Left,
                                _ => ResizeEdge::None,
                            };
                            
                            if edge != ResizeEdge::None {
                                resizable.resizing = true;
                                resizable.edge = edge;
                            }
                        }
                    }
                }
            } else if !pressed {
                resizable.resizing = false;
                resizable.edge = ResizeEdge::None;
            }
        }
    }

    // Appliquer le redimensionnement
    if mouse_position.pressed(MouseButton::Left) {
        if let Ok(window) = primary_window.get_single() {
            if let Some(cursor_pos) = window.cursor_position() {
                for (mut style, mut resizable, _) in windows.iter_mut() {
                    if resizable.resizing {
                        if let (Some(left), Some(top), Some(width), Some(height)) = (
                            style.left.try_extract_pixels(),
                            style.top.try_extract_pixels(),
                            style.width.try_extract_pixels(),
                            style.height.try_extract_pixels(),
                        ) {
                            match resizable.edge {
                                ResizeEdge::Right => {
                                    let new_width = (cursor_pos.x - left).max(resizable.min_size.x);
                                    style.width = Val::Px(new_width);
                                },
                                ResizeEdge::Bottom => {
                                    let new_height = (cursor_pos.y - top).max(resizable.min_size.y);
                                    style.height = Val::Px(new_height);
                                },
                                ResizeEdge::Left => {
                                    let right = left + width;
                                    let new_left = cursor_pos.x.min(right - resizable.min_size.x);
                                    style.left = Val::Px(new_left);
                                    style.width = Val::Px(right - new_left);
                                },
                                ResizeEdge::Top => {
                                    let bottom = top + height;
                                    let new_top = cursor_pos.y.min(bottom - resizable.min_size.y);
                                    style.top = Val::Px(new_top);
                                    style.height = Val::Px(bottom - new_top);
                                },
                                ResizeEdge::BottomRight => {
                                    let new_width = (cursor_pos.x - left).max(resizable.min_size.x);
                                    let new_height = (cursor_pos.y - top).max(resizable.min_size.y);
                                    style.width = Val::Px(new_width);
                                    style.height = Val::Px(new_height);
                                },
                                ResizeEdge::BottomLeft => {
                                    let right = left + width;
                                    let new_left = cursor_pos.x.min(right - resizable.min_size.x);
                                    let new_height = (cursor_pos.y - top).max(resizable.min_size.y);
                                    style.left = Val::Px(new_left);
                                    style.width = Val::Px(right - new_left);
                                    style.height = Val::Px(new_height);
                                },
                                ResizeEdge::TopRight => {
                                    let bottom = top + height;
                                    let new_top = cursor_pos.y.min(bottom - resizable.min_size.y);
                                    let new_width = (cursor_pos.x - left).max(resizable.min_size.x);
                                    style.top = Val::Px(new_top);
                                    style.height = Val::Px(bottom - new_top);
                                    style.width = Val::Px(new_width);
                                },
                                ResizeEdge::TopLeft => {
                                    let right = left + width;
                                    let bottom = top + height;
                                    let new_left = cursor_pos.x.min(right - resizable.min_size.x);
                                    let new_top = cursor_pos.y.min(bottom - resizable.min_size.y);
                                    style.left = Val::Px(new_left);
                                    style.top = Val::Px(new_top);
                                    style.width = Val::Px(right - new_left);
                                    style.height = Val::Px(bottom - new_top);
                                },
                                ResizeEdge::None => {},
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Système pour gérer les clics sur les boutons
fn button_click_system(
    buttons: Query<(Entity, &UIElement, &EventCallback), With<Button>>,
    mut interaction_query: Query<(Entity, &Interaction, Changed<Interaction>), With<Button>>,
    mut ruby_callback_events: EventWriter<RubyCallbackEvent>,
) {
    for (entity, interaction, changed) in interaction_query.iter_mut() {
        if changed && *interaction == Interaction::Pressed {
            // Trouver le callback associé à ce bouton
            for (button_entity, ui_element, callback) in buttons.iter() {
                if button_entity == entity && callback.event_type == "click" {
                    // Déclencher l'événement de callback Ruby
                    ruby_callback_events.send(RubyCallbackEvent {
                        callback: callback.callback.clone(),
                        arguments: HashMap::from([
                            ("id".to_string(), ui_element.id.clone()),
                            ("event".to_string(), "click".to_string()),
                        ]),
                    });
                    break;
                }
            }
        }
    }
}

/// Système pour gérer les interactions avec les champs de saisie
fn input_interaction_system(
    mut inputs: Query<(Entity, &UIElement, &EventCallback, &Children), With<UIElement>>,
    mut text_query: Query<&mut Text>,
    keys: Res<Input<KeyCode>>,
    mut ui_update_events: EventWriter<UIUpdateEvent>,
    mut ruby_callback_events: EventWriter<RubyCallbackEvent>,
) {
    // Cette implémentation est simplifiée, un système d'input complet nécessiterait
    // un focus, un curseur, une sélection, etc.
    for (entity, ui_element, callback, children) in inputs.iter_mut() {
        if ui_element.component_type == "input" {
            // Simuler une mise à jour du texte lorsque Enter est pressé
            if keys.just_pressed(KeyCode::Return) {
                // Déclencher le callback onSubmit
                if callback.event_type == "submit" {
                    // Récupérer le texte actuel
                    let mut current_text = String::new();
                    for &child in children.iter() {
                        if let Ok(mut text) = text_query.get_mut(child) {
                            if let Some(section) = text.sections.first() {
                                current_text = section.value.clone();
                            }
                        }
                    }
                    
                    // Déclencher l'événement de callback Ruby
                    ruby_callback_events.send(RubyCallbackEvent {
                        callback: callback.callback.clone(),
                        arguments: HashMap::from([
                            ("id".to_string(), ui_element.id.clone()),
                            ("event".to_string(), "submit".to_string()),
                            ("value".to_string(), current_text),
                        ]),
                    });
                }
            }
        }
    }
}

/// Système pour gérer les mises à jour de l'UI
fn handle_ui_updates(
    mut commands: Commands,
    mut ui_update_events: EventReader<UIUpdateEvent>,
    mut dsl_state: ResMut<UIDSLState>,
) {
    for event in ui_update_events.iter() {
        // Demander au builder de mettre à jour le composant
        if let Err(error) = dsl_state.builder.update_component(&mut commands, &event.id, &event.update) {
            error!("Erreur de mise à jour UI: {}", error);
        }
    }
}

/// Système pour gérer les callbacks Ruby
fn handle_ruby_callbacks(
    mut ruby_callback_events: EventReader<RubyCallbackEvent>,
    mut dsl_state: ResMut<UIDSLState>,
    mut ui_update_events: EventWriter<UIUpdateEvent>,
) {
    for event in ruby_callback_events.iter() {
        // Exécuter le callback Ruby
        if let Some(parser) = &mut dsl_state.parser {
            match parser.execute_callback(&event.callback, event.arguments.clone()) {
                Ok(result) => {
                    // Traiter le résultat du callback (par exemple, mettre à jour l'UI)
                    info!("Callback Ruby exécuté: {} -> {}", event.callback, result);
                    
                    // Si le résultat est un JSON qui contient une mise à jour d'UI, l'appliquer
                    if let Ok(update) = serde_json::from_str::<serde_json::Value>(&result) {
                        if let Some(updates) = update.get("updates").and_then(|u| u.as_array()) {
                            for update_item in updates {
                                if let (Some(id), Some(action), Some(value)) = (
                                    update_item.get("id").and_then(|i| i.as_str()),
                                    update_item.get("action").and_then(|a| a.as_str()),
                                    update_item.get("value"),
                                ) {
                                    // Créer un événement de mise à jour UI en fonction de l'action
                                    match action {
                                        "setText" => {
                                            if let Some(text) = value.as_str() {
                                                ui_update_events.send(UIUpdateEvent {
                                                    id: id.to_string(),
                                                    update: UIComponentUpdate::SetText(text.to_string()),
                                                });
                                            }
                                        },
                                        // Autres types de mises à jour...
                                        _ => {
                                            warn!("Action de mise à jour inconnue: {}", action);
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Err(error) => {
                    error!("Erreur d'exécution du callback Ruby: {}", error);
                }
            }
        }
    }
}