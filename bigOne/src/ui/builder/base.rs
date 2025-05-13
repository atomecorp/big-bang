// src/ui/builder/base.rs
use bevy::{
    prelude::*,
    ui::{FocusPolicy, Style, UiRect, Val, JustifyContent, AlignItems, PositionType, FlexDirection},
    window::PrimaryWindow,
};
use std::collections::HashMap;

use crate::ui::components::*;
use crate::dsl::parser::*;

/// Gestionnaire de construction d'UI à partir des composants DSL
pub struct UIBuilder {
    // Registre des entités créées, mappées par ID de composant
    entity_registry: HashMap<String, Entity>,
    // Fonction de callback pour exécuter du code Ruby depuis l'UI
    ruby_callback: Option<Box<dyn Fn(String, HashMap<String, String>) -> Result<String, String> + Send + Sync>>,
}

impl UIBuilder {
    /// Crée un nouveau constructeur d'UI
    pub fn new() -> Self {
        Self {
            entity_registry: HashMap::new(),
            ruby_callback: None,
        }
    }

    /// Définit la fonction de callback pour exécuter du code Ruby
    pub fn set_ruby_callback<F>(&mut self, callback: F)
    where
        F: Fn(String, HashMap<String, String>) -> Result<String, String> + Send + Sync + 'static,
    {
        self.ruby_callback = Some(Box::new(callback));
    }

    /// Construit l'UI à partir des composants
    pub fn build_ui(&mut self, commands: &mut Commands, components: &[UIComponent], asset_server: &Res<AssetServer>) {
        // Nettoyer le registre pour la reconstruction
        self.entity_registry.clear();
        
        // Construire chaque composant racine
        for component in components {
            self.build_component(commands, component, None, asset_server);
        }
    }

    /// Construit un composant UI et ses enfants
    pub fn build_component(&mut self, commands: &mut Commands, component: &UIComponent, parent: Option<Entity>, asset_server: &Res<AssetServer>) -> Option<Entity> {
        match component {
            UIComponent::Window(props) => self.build_window(commands, props, asset_server),
            UIComponent::Button(props) => self.build_button(commands, props, parent, asset_server),
            UIComponent::Text(props) => self.build_text(commands, props, parent, asset_server),
            UIComponent::Image(props) => self.build_image(commands, props, parent, asset_server),
            UIComponent::Canvas(props) => self.build_canvas(commands, props, parent, asset_server),
            UIComponent::SVG(props) => self.build_svg(commands, props, parent, asset_server),
            UIComponent::ScrollView(props) => self.build_scrollview(commands, props, parent, asset_server),
            UIComponent::List(props) => self.build_list(commands, props, parent, asset_server),
            UIComponent::Grid(props) => self.build_grid(commands, props, parent, asset_server),
            UIComponent::Input(props) => self.build_input(commands, props, parent, asset_server),
            UIComponent::Viewport3D(props) => self.build_viewport3d(commands, props, parent, asset_server),
            UIComponent::Stack(props) => self.build_stack(commands, props, parent, asset_server),
            UIComponent::Row(props) => self.build_row(commands, props, parent, asset_server),
            UIComponent::Column(props) => self.build_column(commands, props, parent, asset_server),
        }
    }

    /// Construit une fenêtre
    fn build_window(&mut self, commands: &mut Commands, props: &WindowProps, asset_server: &Res<AssetServer>) -> Option<Entity> {
        // Créer une fenêtre en tant que nœud racine
        let window_entity = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: if let Some(x) = props.x { Val::Px(x) } else { Val::Auto },
                        top: if let Some(y) = props.y { Val::Px(y) } else { Val::Auto },
                        width: Val::Px(props.width),
                        height: Val::Px(props.height),
                        flex_direction: FlexDirection::Column,
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                },
                UIElement {
                    id: props.id.clone(),
                    component_type: "window".to_string(),
                },
            ))
            .id();

        // Ajouter les composants Draggable et Resizable si nécessaire
        if props.draggable.unwrap_or(true) {
            commands.entity(window_entity).insert(Draggable {
                dragging: false,
                offset: Vec2::ZERO,
            });
        }

        if props.resizable.unwrap_or(true) {
            commands.entity(window_entity).insert(Resizable {
                resizing: false,
                min_size: Vec2::new(100.0, 50.0),
                edge: ResizeEdge::None,
            });
        }

        // Créer l'en-tête de la fenêtre avec le titre
        let header_entity = commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(30.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                ..default()
            })
            .id();

        // Titre de la fenêtre
        let title_entity = commands
            .spawn(TextBundle::from_section(
                props.title.clone(),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ))
            .id();

        // Bouton de fermeture
        let close_button_entity = commands
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(20.0),
                        height: Val::Px(20.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.8, 0.2, 0.2).into(),
                    ..default()
                },
                EventCallback {
                    event_type: "click".to_string(),
                    callback: "close_window".to_string(),
                },
            ))
            .id();

        // Texte du bouton de fermeture (X)
        commands.entity(close_button_entity).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "X",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 14.0,
                    color: Color::WHITE,
                },
            ));
        });

        // Ajouter le titre et le bouton de fermeture à l'en-tête
        commands.entity(header_entity).push_children(&[title_entity, close_button_entity]);

        // Contenu de la fenêtre
        let content_entity = commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.0)),
                    overflow: bevy::ui::Overflow::clip(),
                    ..default()
                },
                background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                ..default()
            })
            .id();

        // Construire les enfants du contenu
        for child in &props.children {
            if let Some(child_entity) = self.build_component(commands, child, Some(content_entity), asset_server) {
                commands.entity(content_entity).add_child(child_entity);
            }
        }

        // Ajouter l'en-tête et le contenu à la fenêtre
        commands.entity(window_entity).push_children(&[header_entity, content_entity]);

        // Enregistrer l'entité créée
        self.entity_registry.insert(props.id.clone(), window_entity);

        Some(window_entity)
    }
}