use bevy::{
    prelude::*,
    window::WindowPlugin,
    winit::WinitSettings,
};
use std::fs;
use std::path::Path;

mod ui;
mod dsl;

use ui::systems::{UIDSLPlugin, UIDSLState, UIHotReload, initialize_dsl};

fn main() {
    // Configurer et lancer l'application Bevy
    App::new()
        // Ajout des plugins par défaut
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "OS Like UI - Bevy + DSL Ruby".to_string(),
                    resolution: (1280.0, 720.0).into(),
                    ..default()
                }),
                ..default()
            }
        ))
        // Configuration pour éviter de surcharger le CPU
        .insert_resource(WinitSettings::desktop_app())
        
        // Ajouter notre plugin UI DSL
        .add_plugins(UIDSLPlugin)
        
        // Configurer le hot-reload
        .insert_resource(UIHotReload {
            enabled: true,
            script_path: "scripts/ui.rb".to_string(),
            last_modified: std::time::SystemTime::now(),
        })
        
        // Système d'initialisation
        .add_systems(Startup, setup_system)
        
        // Lancer l'application
        .run();
}

/// Système d'initialisation de l'application
fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut dsl_state: ResMut<UIDSLState>
) {
    // Charger la police par défaut
    asset_server.load("fonts/FiraSans-Regular.ttf");
    asset_server.load("fonts/FiraSans-Bold.ttf");
    
    // Créer une caméra 2D pour l'interface utilisateur
    commands.spawn(Camera2dBundle::default());
    
    // Créer une caméra 3D pour les viewports 3D (comme décrit dans le DSL)
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    
    // Ajouter une lumière globale pour les scènes 3D
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 10.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Configurer le callback Ruby pour le parser DSL
    let ruby_callback = move |callback_name: String, args: std::collections::HashMap<String, String>| -> Result<String, String> {
        if let Some(parser) = &mut dsl_state.parser {
            parser.execute_callback(&callback_name, args)
        } else {
            Err("Parser DSL non initialisé".to_string())
        }
    };
    
    dsl_state.builder.set_ruby_callback(ruby_callback);
    
    // Charger le script UI initial
    let script_path = Path::new("scripts/ui.rb");
    if script_path.exists() {
        match fs::read_to_string(script_path) {
            Ok(script) => {
                // Initialiser l'UI avec le script
                if let Err(error) = initialize_dsl(commands, dsl_state, asset_server, &script) {
                    error!("Erreur lors de l'initialisation UI: {}", error);
                } else {
                    info!("UI initialisée avec succès depuis {}", script_path.display());
                }
            },
            Err(error) => {
                error!("Erreur lors de la lecture du script UI: {}", error);
                // Créer une interface par défaut
                create_default_ui(&mut commands, &asset_server);
            }
        }
    } else {
        warn!("Fichier script UI non trouvé: {}", script_path.display());
        // Créer une interface par défaut
        create_default_ui(&mut commands, &asset_server);
    }
}

/// Crée une interface utilisateur par défaut si le script n'a pas pu être chargé
fn create_default_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // Fenêtre principale
    let window_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(100.0),
                    top: Val::Px(100.0),
                    width: Val::Px(400.0),
                    height: Val::Px(300.0),
                    flex_direction: FlexDirection::Column,
                    border: UiRect::all(Val::Px(1.0)),
                    // À la place de gap, utilisez padding, margin ou un autre système
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
            },
            ui::components::UIElement {
                id: "default_window".to_string(),
                component_type: "window".to_string(),
            },
            ui::components::Draggable {
                dragging: false,
                offset: Vec2::ZERO,
            },
            ui::components::Resizable {
                resizing: false,
                min_size: Vec2::new(100.0, 50.0),
                edge: ui::components::ResizeEdge::None,
            },
        ))
        .id();

    // En-tête de la fenêtre
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
            "Interface par défaut",
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
            ui::components::EventCallback {
                event_type: "click".to_string(),
                callback: "close_window".to_string(),
            },
        ))
        .id();

    // Texte du bouton de fermeture
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

    // Ajouter les éléments à l'en-tête
    commands.entity(header_entity).push_children(&[title_entity, close_button_entity]);

    // Contenu de la fenêtre
    let content_entity = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                // Remplacer gap par padding ou margin entre les éléments
                // Dans certaines versions de Bevy, row_gap et column_gap sont disponibles
                // sinon, ajouter des marges individuelles aux enfants
                margin: UiRect::all(Val::Px(0.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        })
        .id();

    // Message d'erreur
    let message_entity = commands
        .spawn(TextBundle::from_section(
            "Impossible de charger le script UI.\nCette interface par défaut est affichée à la place.",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                font_size: 16.0,
                color: Color::rgb(1.0, 0.8, 0.8),
            },
        ).with_text_alignment(TextAlignment::Center))
        .id();

    // Ajouter une marge au message pour espacer les éléments
    commands.entity(message_entity).insert(
        Style {
            margin: UiRect::bottom(Val::Px(10.0)), // espacement entre le message et le bouton
            ..default()
        }
    );

    // Bouton d'exemple
    let button_entity = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.25, 0.5, 0.85).into(),
                ..default()
            },
            ui::components::UIElement {
                id: "default_button".to_string(),
                component_type: "button".to_string(),
            },
        ))
        .id();

    // Texte du bouton
    let button_text_entity = commands
        .spawn(TextBundle::from_section(
            "Exemple de bouton",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                font_size: 16.0,
                color: Color::WHITE,
            },
        ))
        .id();

    // Ajouter le texte au bouton
    commands.entity(button_entity).add_child(button_text_entity);

    // Ajouter les éléments au contenu
    commands.entity(content_entity).push_children(&[message_entity, button_entity]);

    // Ajouter l'en-tête et le contenu à la fenêtre
    commands.entity(window_entity).push_children(&[header_entity, content_entity]);
}