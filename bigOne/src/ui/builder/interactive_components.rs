// src/ui/builder/interactive_components.rs
use bevy::{
    prelude::*,
    render::mesh::shape,
    ui::{Style, UiRect, Val, JustifyContent, AlignItems, PositionType},
};
use std::collections::HashMap;

use crate::ui::components::*;

impl UIBuilder {
    /// Construit un champ de saisie
    pub fn build_input(&mut self, commands: &mut Commands, props: &InputProps, parent: Option<Entity>, asset_server: &Res<AssetServer>) -> Option<Entity> {
        // Largeur du champ
        let width = props.width.unwrap_or(200.0);

        // Créer un nœud pour le champ de saisie
        let input_entity = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(width),
                        height: Val::Px(30.0),
                        border: UiRect::all(Val::Px(1.0)),
                        padding: UiRect::all(Val::Px(5.0)),
                        margin: UiRect::all(Val::Px(4.0)),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                    ..default()
                },
                UIElement {
                    id: props.id.clone(),
                    component_type: "input".to_string(),
                },
            ))
            .id();

        // Texte du champ (valeur ou placeholder)
        let text_value = props.value.clone().unwrap_or_else(|| {
            props.placeholder.clone().unwrap_or_else(|| "".to_string())
        });

        let text_color = if props.value.is_some() {
            Color::WHITE
        } else {
            Color::rgb(0.5, 0.5, 0.5) // Gris pour le placeholder
        };

        let text_entity = commands
            .spawn(TextBundle::from_section(
                text_value,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 16.0,
                    color: text_color,
                },
            ))
            .id();

        // Ajouter le texte au champ
        commands.entity(input_entity).add_child(text_entity);

        // Ajouter les callbacks si spécifiés
        if let Some(on_change) = &props.on_change {
            commands.entity(input_entity).insert(EventCallback {
                event_type: "change".to_string(),
                callback: on_change.clone(),
            });
        }

        if let Some(on_submit) = &props.on_submit {
            commands.entity(input_entity).insert(EventCallback {
                event_type: "submit".to_string(),
                callback: on_submit.clone(),
            });
        }

        // Si un parent est spécifié, ajouter le champ comme enfant
        if let Some(parent_entity) = parent {
            commands.entity(parent_entity).add_child(input_entity);
        }

        // Enregistrer l'entité créée
        self.entity_registry.insert(props.id.clone(), input_entity);

        Some(input_entity)
    }

    /// Construit un viewport 3D
    pub fn build_viewport3d(&mut self, commands: &mut Commands, props: &Viewport3DProps, parent: Option<Entity>, asset_server: &Res<AssetServer>) -> Option<Entity> {
        // Créer un nœud pour le viewport 3D (conteneur)
        let viewport_entity = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(props.width),
                        height: Val::Px(props.height),
                        margin: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                    ..default()
                },
                UIElement {
                    id: props.id.clone(),
                    component_type: "viewport3d".to_string(),
                },
            ))
            .id();

        // Créer une caméra pour la vue 3D
        let camera_position = if let Some(camera) = &props.camera {
            Vec3::new(
                camera.position[0],
                camera.position[1],
                camera.position[2],
            )
        } else {
            Vec3::new(0.0, 5.0, 10.0) // Position par défaut
        };

        let camera_target = if let Some(camera) = &props.camera {
            Vec3::new(
                camera.target[0],
                camera.target[1],
                camera.target[2],
            )
        } else {
            Vec3::ZERO // Cible par défaut
        };

        // Créer la caméra 3D
        // Note: Dans une implémentation complète, il faudrait gérer un RenderTarget spécifique
        // pour cette vue, plutôt que d'utiliser la caméra principale
        let camera_entity = commands
            .spawn(Camera3dBundle {
                transform: Transform::from_translation(camera_position)
                    .looking_at(camera_target, Vec3::Y),
                ..default()
            })
            .id();

        // Créer une lumière pour la scène 3D
        let light_entity = commands
            .spawn(PointLightBundle {
                point_light: PointLight {
                    intensity: 1500.0,
                    shadows_enabled: true,
                    ..default()
                },
                transform: Transform::from_xyz(4.0, 8.0, 4.0),
                ..default()
            })
            .id();

        // Créer un objet 3D simple pour la démonstration (sphère)
        let sphere_entity = commands
            .spawn(PbrBundle {
                mesh: commands.world.resource::<Assets<Mesh>>().add(shape::UVSphere {
                    radius: 1.0,
                    sectors: 32,
                    stacks: 16,
                }.into()),
                material: commands.world.resource::<Assets<StandardMaterial>>().add(StandardMaterial {
                    base_color: Color::rgb(0.8, 0.2, 0.3),
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            })
            .id();

        // Ajouter les entités 3D au viewport
        commands.entity(viewport_entity).push_children(&[camera_entity, light_entity, sphere_entity]);

        // Si un parent est spécifié, ajouter le viewport comme enfant
        if let Some(parent_entity) = parent {
            commands.entity(parent_entity).add_child(viewport_entity);
        }

        // Enregistrer l'entité créée
        self.entity_registry.insert(props.id.clone(), viewport_entity);

        Some(viewport_entity)
    }

    /// Méthode pour mettre à jour un composant existant
    pub fn update_component(&mut self, commands: &mut Commands, component_id: &str, update: &UIComponentUpdate) -> Result<(), String> {
        if let Some(entity) = self.entity_registry.get(component_id) {
            match update {
                UIComponentUpdate::SetText(text) => {
                    // Trouver le composant Text enfant et mettre à jour son contenu
                    if let Ok(mut text_query) = commands.world.query_filtered::<&mut Text, With<Parent>>(entity) {
                        for mut text_component in text_query.iter_mut(commands.world) {
                            if let Some(section) = text_component.sections.first_mut() {
                                section.value = text.clone();
                            }
                        }
                    }
                },
                UIComponentUpdate::SetImage(source) => {
                    // Mettre à jour la source de l'image
                    // Cette implémentation est simplifiée, il faudrait gérer le chargement des assets
                },
                UIComponentUpdate::SetPosition(x, y) => {
                    // Mettre à jour la position d'un composant
                    if let Ok(mut style) = commands.world.query::<&mut Style>().get_mut(commands.world, *entity) {
                        style.left = Val::Px(*x);
                        style.top = Val::Px(*y);
                    }
                },
                UIComponentUpdate::SetSize(width, height) => {
                    // Mettre à jour la taille d'un composant
                    if let Ok(mut style) = commands.world.query::<&mut Style>().get_mut(commands.world, *entity) {
                        style.width = Val::Px(*width);
                        style.height = Val::Px(*height);
                    }
                },
                UIComponentUpdate::SetVisible(visible) => {
                    // Mettre à jour la visibilité d'un composant
                    if *visible {
                        commands.entity(*entity).insert(Visibility::Visible);
                    } else {
                        commands.entity(*entity).insert(Visibility::Hidden);
                    }
                },
                UIComponentUpdate::SetStyle(style_map) => {
                    // Mettre à jour le style d'un composant
                    if let Ok(mut style) = commands.world.query::<&mut Style>().get_mut(commands.world, *entity) {
                        for (key, value) in style_map {
                            match key.as_str() {
                                "backgroundColor" => {
                                    if let Ok(mut background) = commands.world.query::<&mut BackgroundColor>().get_mut(commands.world, *entity) {
                                        // Parser la couleur (simplifié)
                                        if value.starts_with("rgb(") {
                                            // Format rgb(r, g, b)
                                            let rgb_values: Vec<&str> = value
                                                .trim_start_matches("rgb(")
                                                .trim_end_matches(")")
                                                .split(',')
                                                .collect();
                                            if rgb_values.len() >= 3 {
                                                let r = rgb_values[0].trim().parse::<f32>().unwrap_or(255.0) / 255.0;
                                                let g = rgb_values[1].trim().parse::<f32>().unwrap_or(255.0) / 255.0;
                                                let b = rgb_values[2].trim().parse::<f32>().unwrap_or(255.0) / 255.0;
                                                *background = Color::rgb(r, g, b).into();
                                            }
                                        }
                                    }
                                },
                                // Autres propriétés de style à gérer...
                                _ => {
                                    // Propriété non reconnue
                                }
                            }
                        }
                    }
                },
                UIComponentUpdate::Replace(new_component) => {
                    // Remplacement complet d'un composant
                    // Ce cas est plus complexe et nécessiterait de despawn l'entité existante
                    // et de créer une nouvelle entité avec le nouveau composant
                },
            }
            Ok(())
        } else {
            Err(format!("Composant avec ID '{}' non trouvé", component_id))
        }
    }

    /// Exécute un callback Ruby
    pub fn execute_ruby_callback(&self, callback_name: &str, args: HashMap<String, String>) -> Result<String, String> {
        if let Some(callback_fn) = &self.ruby_callback {
            callback_fn(callback_name.to_string(), args)
        } else {
            Err("Fonction de callback Ruby non définie".to_string())
        }
    }
}