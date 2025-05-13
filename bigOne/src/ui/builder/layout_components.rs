// src/ui/builder/layout_components.rs
use bevy::{
    prelude::*,
    ui::{Style, UiRect, Val, FlexDirection, JustifyContent, AlignItems},
};
use std::collections::HashMap;

use crate::ui::components::*;

impl UIBuilder {
    /// Construit une vue défilante
    pub fn build_scrollview(&mut self, commands: &mut Commands, props: &ScrollViewProps, parent: Option<Entity>, asset_server: &Res<AssetServer>) -> Option<Entity> {
        // Créer un nœud pour la vue défilante
        let scrollview_entity = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(props.width),
                        height: Val::Px(props.height),
                        flex_direction: FlexDirection::Column,
                        overflow: bevy::ui::Overflow::scroll(),
                        margin: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                },
                UIElement {
                    id: props.id.clone(),
                    component_type: "scrollview".to_string(),
                },
            ))
            .id();

        // Construire le contenu de la vue défilante
        for child in &props.children {
            if let Some(child_entity) = self.build_component(commands, child, Some(scrollview_entity), asset_server) {
                commands.entity(scrollview_entity).add_child(child_entity);
            }
        }

        // Si un parent est spécifié, ajouter la vue défilante comme enfant
        if let Some(parent_entity) = parent {
            commands.entity(parent_entity).add_child(scrollview_entity);
        }

        // Enregistrer l'entité créée
        self.entity_registry.insert(props.id.clone(), scrollview_entity);

        Some(scrollview_entity)
    }

    /// Construit une liste
    pub fn build_list(&mut self, commands: &mut Commands, props: &ListProps, parent: Option<Entity>, asset_server: &Res<AssetServer>) -> Option<Entity> {
        // Déterminer la direction de la liste
        let flex_direction = match props.direction.as_deref() {
            Some("horizontal") => FlexDirection::Row,
            _ => FlexDirection::Column, // Vertical par défaut
        };

        // Espacement entre les éléments
        let spacing = props.spacing.unwrap_or(5.0);

        // Créer un nœud pour la liste
        let list_entity = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        flex_direction,
                        gap: Size::all(Val::Px(spacing)),
                        margin: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    ..default()
                },
                UIElement {
                    id: props.id.clone(),
                    component_type: "list".to_string(),
                },
            ))
            .id();

        // Construire les éléments de la liste
        for item in &props.items {
            if let Some(item_entity) = self.build_component(commands, item, Some(list_entity), asset_server) {
                commands.entity(list_entity).add_child(item_entity);
            }
        }

        // Si un parent est spécifié, ajouter la liste comme enfant
        if let Some(parent_entity) = parent {
            commands.entity(parent_entity).add_child(list_entity);
        }

        // Enregistrer l'entité créée
        self.entity_registry.insert(props.id.clone(), list_entity);

        Some(list_entity)
    }

    /// Construit une grille
    pub fn build_grid(&mut self, commands: &mut Commands, props: &GridProps, parent: Option<Entity>, asset_server: &Res<AssetServer>) -> Option<Entity> {
        // Nombre de colonnes
        let columns = props.columns;
        
        // Espacement entre les éléments
        let spacing = props.spacing.unwrap_or(5.0);

        // Créer un nœud pour la grille
        let grid_entity = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        gap: Size::all(Val::Px(spacing)),
                        margin: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    ..default()
                },
                UIElement {
                    id: props.id.clone(),
                    component_type: "grid".to_string(),
                },
            ))
            .id();

        // Créer les rangées de la grille
        let mut items = props.items.iter();
        let mut row_index = 0;

        // Continuer tant qu'il reste des éléments à placer
        while let Some(_) = items.next().map(|_| {}).or_else(|| None) {
            // Reculer d'un élément
            items = props.items.iter().skip(row_index * columns);
            
            // Créer une rangée
            let row_entity = commands
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        gap: Size::all(Val::Px(spacing)),
                        ..default()
                    },
                    ..default()
                })
                .id();

            // Ajouter jusqu'à 'columns' éléments dans cette rangée
            for (col_index, item) in items.clone().take(columns).enumerate() {
                let item_index = row_index * columns + col_index;
                if item_index < props.items.len() {
                    if let Some(item_entity) = self.build_component(commands, item, Some(row_entity), asset_server) {
                        commands.entity(row_entity).add_child(item_entity);
                    }
                }
            }

            // Ajouter la rangée à la grille
            commands.entity(grid_entity).add_child(row_entity);
            
            // Passer à la rangée suivante
            row_index += 1;
        }

        // Si un parent est spécifié, ajouter la grille comme enfant
        if let Some(parent_entity) = parent {
            commands.entity(parent_entity).add_child(grid_entity);
        }

        // Enregistrer l'entité créée
        self.entity_registry.insert(props.id.clone(), grid_entity);

        Some(grid_entity)
    }

    /// Construit une pile (stack)
    pub fn build_stack(&mut self, commands: &mut Commands, props: &StackProps, parent: Option<Entity>, asset_server: &Res<AssetServer>) -> Option<Entity> {
        // Créer un nœud pour la pile
        let stack_entity = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        position_type: bevy::ui::PositionType::Relative,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
                UIElement {
                    id: props.id.clone(),
                    component_type: "stack".to_string(),
                },
            ))
            .id();

        // Construire les enfants de la pile
        for child in &props.children {
            if let Some(child_entity) = self.build_component(commands, child, Some(stack_entity), asset_server) {
                commands.entity(stack_entity).add_child(child_entity);
            }
        }

        // Si un parent est spécifié, ajouter la pile comme enfant
        if let Some(parent_entity) = parent {
            commands.entity(parent_entity).add_child(stack_entity);
        }

        // Enregistrer l'entité créée
        self.entity_registry.insert(props.id.clone(), stack_entity);

        Some(stack_entity)
    }

    /// Construit une rangée (row)
    pub fn build_row(&mut self, commands: &mut Commands, props: &RowProps, parent: Option<Entity>, asset_server: &Res<AssetServer>) -> Option<Entity> {
        // Espacement entre les éléments
        let spacing = props.spacing.unwrap_or(5.0);

        // Déterminer l'alignement
        let justify_content = match props.align.as_deref() {
            Some("start") => JustifyContent::FlexStart,
            Some("center") => JustifyContent::Center,
            Some("end") => JustifyContent::FlexEnd,
            Some("spaceBetween") => JustifyContent::SpaceBetween,
            Some("spaceAround") => JustifyContent::SpaceAround,
            _ => JustifyContent::FlexStart,
        };

        // Créer un nœud pour la rangée
        let row_entity = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content,
                        gap: Size::all(Val::Px(spacing)),
                        margin: UiRect::all(Val::Px(4.0)),
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
                UIElement {
                    id: props.id.clone(),
                    component_type: "row".to_string(),
                },
            ))
            .id();

        // Construire les enfants de la rangée
        for child in &props.children {
            if let Some(child_entity) = self.build_component(commands, child, Some(row_entity), asset_server) {
                commands.entity(row_entity).add_child(child_entity);
            }
        }

        // Si un parent est spécifié, ajouter la rangée comme enfant
        if let Some(parent_entity) = parent {
            commands.entity(parent_entity).add_child(row_entity);
        }

        // Enregistrer l'entité créée
        self.entity_registry.insert(props.id.clone(), row_entity);

        Some(row_entity)
    }

    /// Construit une colonne (column)
    pub fn build_column(&mut self, commands: &mut Commands, props: &ColumnProps, parent: Option<Entity>, asset_server: &Res<AssetServer>) -> Option<Entity> {
        // Espacement entre les éléments
        let spacing = props.spacing.unwrap_or(5.0);

        // Déterminer l'alignement
        let justify_content = match props.align.as_deref() {
            Some("start") => JustifyContent::FlexStart,
            Some("center") => JustifyContent::Center,
            Some("end") => JustifyContent::FlexEnd,
            Some("spaceBetween") => JustifyContent::SpaceBetween,
            Some("spaceAround") => JustifyContent::SpaceAround,
            _ => JustifyContent::FlexStart,
        };

        // Créer un nœud pour la colonne
        let column_entity = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content,
                        gap: Size::all(Val::Px(spacing)),
                        margin: UiRect::all(Val::Px(4.0)),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
                UIElement {
                    id: props.id.clone(),
                    component_type: "column".to_string(),
                },
            ))
            .id();

        // Construire les enfants de la colonne
        for child in &props.children {
            if let Some(child_entity) = self.build_component(commands, child, Some(column_entity), asset_server) {
                commands.entity(column_entity).add_child(child_entity);
            }
        }

        // Si un parent est spécifié, ajouter la colonne comme enfant
        if let Some(parent_entity) = parent {
            commands.entity(parent_entity).add_child(column_entity);
        }

        // Enregistrer l'entité créée
        self.entity_registry.insert(props.id.clone(), column_entity);

        Some(column_entity)
    }
}