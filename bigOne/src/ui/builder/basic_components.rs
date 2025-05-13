// src/ui/builder/basic_components.rs
use bevy::{
    prelude::*,
    ui::{FocusPolicy, Style, UiRect, Val, JustifyContent, AlignItems, PositionType, FlexDirection},
};
use std::collections::HashMap;

use crate::ui::components::*;

impl UIBuilder {
    /// Construit un bouton
    pub fn build_button(&mut self, commands: &mut Commands, props: &ButtonProps, parent: Option<Entity>, asset_server: &Res<AssetServer>) -> Option<Entity> {
        // Calculer le style du bouton
        let mut style = Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(8.0)),
            margin: UiRect::all(Val::Px(4.0)),
            ..default()
        };

        // Appliquer les dimensions personnalisées si spécifiées
        if let Some(width) = props.width {
            style.width = Val::Px(width);
        } else {
            style.width = Val::Auto;
        }

        if let Some(height) = props.height {
            style.height = Val::Px(height);
        } else {
            style.height = Val::Auto;
        }

        // Créer le bouton
        let button_entity = commands
            .spawn((
                ButtonBundle {
                    style,
                    background_color: Color::rgb(0.25, 0.5, 0.85).into(),
                    ..default()
                },
                UIElement {
                    id: props.id.clone(),
                    component_type: "button".to_string(),
                },
            ))
            .id();

        // Ajouter le callback de clic si spécifié
        if let Some(on_click) = &props.on_click {
            commands.entity(button_entity).insert(EventCallback {
                event_type: "click".to_string(),
                callback: on_click.clone(),
            });
        }

        // Créer le contenu du bouton (texte et/ou icône)
        let mut button_content = Vec::new();

        // Ajouter l'icône si spécifiée
        if let Some(icon) = &props.icon {
            let icon_entity = commands
                .spawn(ImageBundle {
                    style: Style {
                        width: Val::Px(20.0),
                        height: Val::Px(20.0),
                        margin: UiRect::right(Val::Px(5.0)),
                        ..default()
                    },
                    image: UiImage::new(asset_server.load(icon)),
                    ..default()
                })
                .id();
            button_content.push(icon_entity);
        }

        // Ajouter le texte
        let text_entity = commands
            .spawn(TextBundle::from_section(
                props.text.clone(),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ))
            .id();
        button_content.push(text_entity);

        // Ajouter le contenu au bouton
        commands.entity(button_entity).push_children(&button_content);

        // Si un parent est spécifié, ajouter le bouton comme enfant
        if let Some(parent_entity) = parent {
            commands.entity(parent_entity).add_child(button_entity);
        }

        // Enregistrer l'entité créée
        self.entity_registry.insert(props.id.clone(), button_entity);

        Some(button_entity)
    }

    /// Construit un texte
    pub fn build_text(&mut self, commands: &mut Commands, props: &TextProps, parent: Option<Entity>, asset_server: &Res<AssetServer>) -> Option<Entity> {
        // Configurer le style de texte
        let font_size = props.size.unwrap_or(16.0);
        let color = match &props.color {
            Some(color_str) => {
                // Parser la couleur (exemple simple, à améliorer)
                if color_str.starts_with("rgb(") {
                    // Format rgb(r, g, b)
                    let rgb_values: Vec<&str> = color_str
                        .trim_start_matches("rgb(")
                        .trim_end_matches(")")
                        .split(',')
                        .collect();
                    if rgb_values.len() >= 3 {
                        let r = rgb_values[0].trim().parse::<f32>().unwrap_or(255.0) / 255.0;
                        let g = rgb_values[1].trim().parse::<f32>().unwrap_or(255.0) / 255.0;
                        let b = rgb_values[2].trim().parse::<f32>().unwrap_or(255.0) / 255.0;
                        Color::rgb(r, g, b)
                    } else {
                        Color::WHITE
                    }
                } else {
                    // Couleurs nommées simples
                    match color_str.to_lowercase().as_str() {
                        "red" => Color::RED,
                        "green" => Color::GREEN,
                        "blue" => Color::BLUE,
                        "yellow" => Color::YELLOW,
                        "white" => Color::WHITE,
                        "black" => Color::BLACK,
                        _ => Color::WHITE,
                    }
                }
            }
            None => Color::WHITE,
        };

        // Définir l'alignement du texte
        let alignment = match props.align.as_deref() {
            Some("left") => TextAlignment::Left,
            Some("center") => TextAlignment::Center,
            Some("right") => TextAlignment::Right,
            _ => TextAlignment::Left,
        };

        // Charger la police spécifiée ou utiliser la police par défaut
        let font = asset_server.load(
            props
                .font
                .as_deref()
                .unwrap_or("fonts/FiraSans-Regular.ttf"),
        );

        // Créer l'entité de texte
        let text_entity = commands
            .spawn((
                TextBundle::from_section(
                    props.content.clone(),
                    TextStyle {
                        font,
                        font_size,
                        color,
                    },
                )
                .with_text_alignment(alignment)
                .with_style(Style {
                    margin: UiRect::all(Val::Px(4.0)),
                    ..default()
                }),
                UIElement {
                    id: props.id.clone(),
                    component_type: "text".to_string(),
                },
            ))
            .id();

        // Si un parent est spécifié, ajouter le texte comme enfant
        if let Some(parent_entity) = parent {
            commands.entity(parent_entity).add_child(text_entity);
        }

        // Enregistrer l'entité créée
        self.entity_registry.insert(props.id.clone(), text_entity);

        Some(text_entity)
    }

    /// Construit une image
    pub fn build_image(&mut self, commands: &mut Commands, props: &ImageProps, parent: Option<Entity>, asset_server: &Res<AssetServer>) -> Option<Entity> {
        // Charger l'image
        let image_handle = asset_server.load(&props.source);

        // Configurer le style de l'image
        let mut style = Style {
            margin: UiRect::all(Val::Px(4.0)),
            ..default()
        };

        // Appliquer les dimensions personnalisées si spécifiées
        if let Some(width) = props.width {
            style.width = Val::Px(width);
        } else {
            style.width = Val::Auto;
        }

        if let Some(height) = props.height {
            style.height = Val::Px(height);
        } else {
            style.height = Val::Auto;
        }

        // Créer l'entité d'image
        let image_entity = commands
            .spawn((
                ImageBundle {
                    style,
                    image: UiImage::new(image_handle),
                    ..default()
                },
                UIElement {
                    id: props.id.clone(),
                    component_type: "image".to_string(),
                },
            ))
            .id();

        // Si un parent est spécifié, ajouter l'image comme enfant
        if let Some(parent_entity) = parent {
            commands.entity(parent_entity).add_child(image_entity);
        }

        // Enregistrer l'entité créée
        self.entity_registry.insert(props.id.clone(), image_entity);

        Some(image_entity)
    }

    /// Construit un canevas
    pub fn build_canvas(&mut self, commands: &mut Commands, props: &CanvasProps, parent: Option<Entity>, asset_server: &Res<AssetServer>) -> Option<Entity> {
        // Créer un nœud pour le canevas
        let canvas_entity = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(props.width),
                        height: Val::Px(props.height),
                        margin: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.9, 0.9, 0.9).into(),
                    ..default()
                },
                UIElement {
                    id: props.id.clone(),
                    component_type: "canvas".to_string(),
                },
            ))
            .id();

        // Ajouter les callbacks si spécifiés
        if let Some(on_draw) = &props.on_draw {
            commands.entity(canvas_entity).insert(EventCallback {
                event_type: "draw".to_string(),
                callback: on_draw.clone(),
            });
        }

        if let Some(on_click) = &props.on_click {
            commands.entity(canvas_entity).insert(EventCallback {
                event_type: "click".to_string(),
                callback: on_click.clone(),
            });
        }

        // Si un parent est spécifié, ajouter le canevas comme enfant
        if let Some(parent_entity) = parent {
            commands.entity(parent_entity).add_child(canvas_entity);
        }

        // Enregistrer l'entité créée
        self.entity_registry.insert(props.id.clone(), canvas_entity);

        Some(canvas_entity)
    }

    /// Construit un SVG
    pub fn build_svg(&mut self, commands: &mut Commands, props: &SVGProps, parent: Option<Entity>, asset_server: &Res<AssetServer>) -> Option<Entity> {
        // Pour simplifier, on charge simplement une image SVG via Bevy
        // Dans une implémentation complète, on utiliserait resvg pour rendre le SVG

        // Charger le SVG comme une image
        let svg_handle = asset_server.load(&props.source);

        // Configurer le style du SVG
        let mut style = Style {
            margin: UiRect::all(Val::Px(4.0)),
            ..default()
        };

        // Appliquer les dimensions personnalisées si spécifiées
        if let Some(width) = props.width {
            style.width = Val::Px(width);
        } else {
            style.width = Val::Auto;
        }

        if let Some(height) = props.height {
            style.height = Val::Px(height);
        } else {
            style.height = Val::Auto;
        }

        // Créer l'entité SVG
        let svg_entity = commands
            .spawn((
                ImageBundle {
                    style,
                    image: UiImage::new(svg_handle),
                    ..default()
                },
                UIElement {
                    id: props.id.clone(),
                    component_type: "svg".to_string(),
                },
            ))
            .id();

        // Si un parent est spécifié, ajouter le SVG comme enfant
        if let Some(parent_entity) = parent {
            commands.entity(parent_entity).add_child(svg_entity);
        }

        // Enregistrer l'entité créée
        self.entity_registry.insert(props.id.clone(), svg_entity);

        Some(svg_entity)
    }
}