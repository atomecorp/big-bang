use artichoke::prelude::*;
use bevy::{
    prelude::*,
    text::TextAlignment,
    render::mesh::shape,
    winit::WinitSettings,
};
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// Le chemin vers le script Ruby
const RUBY_SCRIPT_PATH: &str = "scripts/main.rb";

// Structure pour passer des données entre threads
struct RubyState {
    dt: f32,
    time: f32,
    x: f32,
    y: f32,
    z: f32,
    text: String,
    color: [f32; 3],
    slider_value: f32,
    json_data: String,
    script_output: String,
    script_error: String,
    running: bool,
    rotation: f32,
}

impl Default for RubyState {
    fn default() -> Self {
        Self {
            dt: 0.0,
            time: 0.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            text: "Chargement...".to_string(),
            color: [1.0, 1.0, 1.0],
            slider_value: 5.0,
            json_data: "{}".to_string(),
            script_output: String::new(),
            script_error: String::new(),
            running: true,
            rotation: 0.0,
        }
    }
}

// Composant pour notre ressource Ruby
#[derive(Resource)]
struct RubyResource {
    state: Arc<Mutex<RubyState>>,
    last_update: Instant,
}

// Différents états de l'application
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
enum AppState {
    #[default]
    Visualisation,
    Console,
    Aide,
}

// Composants pour nos entités
#[derive(Component)]
struct RubyObject;

#[derive(Component)]
struct DynamicText;

#[derive(Component)]
struct SliderText;

#[derive(Component)]
struct ConsoleText;

#[derive(Component)]
struct ErrorText;

#[derive(Component)]
struct DataDisplay;

// Le système principal qui met à jour les données Ruby
fn update_ruby_state(
    time: Res<Time>,
    ruby_resource: ResMut<RubyResource>,
) {
    let _now = Instant::now();
    let dt = time.delta_seconds();
    
    // Mettre à jour les données de temps dans l'état Ruby
    {
        let mut state = ruby_resource.state.lock().unwrap();
        state.time += dt;
        state.dt = dt;
    }
    
    // Dans Bevy 0.12, last_update doit être muté via 'get_mut()'
    // ou en utilisant un RefCell/Mutex
}

// Système pour mettre à jour l'objet 3D basé sur les données Ruby
fn update_ruby_object(
    ruby_resource: Res<RubyResource>,
    mut query: Query<(&mut Transform, &Handle<StandardMaterial>), With<RubyObject>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let state = ruby_resource.state.lock().unwrap();
    
    for (mut transform, mat_handle) in query.iter_mut() {
        // Mettre à jour la position
        transform.translation = Vec3::new(state.x, state.y, state.z);
        
        // Mettre à jour la rotation
        transform.rotation = Quat::from_rotation_y(state.rotation);
        
        // Mettre à jour la couleur
        if let Some(material) = materials.get_mut(mat_handle.id()) {
            material.base_color = Color::rgb(state.color[0], state.color[1], state.color[2]);
        }
    }
}

// Mise à jour du texte dynamique
fn update_dynamic_text(
    ruby_resource: Res<RubyResource>,
    mut query: Query<&mut Text, With<DynamicText>>,
) {
    let state = ruby_resource.state.lock().unwrap();
    
    for mut text in query.iter_mut() {
        text.sections[0].value = state.text.clone();
        text.sections[0].style.color = Color::rgb(
            state.color[0], 
            state.color[1], 
            state.color[2]
        );
    }
}

// Mise à jour du texte du slider
fn update_slider_text(
    ruby_resource: Res<RubyResource>,
    mut query: Query<&mut Text, With<SliderText>>,
) {
    let state = ruby_resource.state.lock().unwrap();
    
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Valeur ajustable: {:.1}", state.slider_value);
    }
}

// Mise à jour du texte de la console
fn update_console_text(
    ruby_resource: Res<RubyResource>,
    mut query: Query<&mut Text, With<ConsoleText>>,
) {
    let state = ruby_resource.state.lock().unwrap();
    
    for mut text in query.iter_mut() {
        text.sections[0].value = state.script_output.clone();
    }
}

// Mise à jour du texte d'erreur
fn update_error_text(
    ruby_resource: Res<RubyResource>,
    mut query: Query<&mut Text, With<ErrorText>>,
) {
    let state = ruby_resource.state.lock().unwrap();
    
    for mut text in query.iter_mut() {
        text.sections[0].value = state.script_error.clone();
    }
}

// Mise à jour de l'affichage des données JSON
fn update_data_display(
    ruby_resource: Res<RubyResource>,
    mut query: Query<&mut Text, With<DataDisplay>>,
) {
    let state = ruby_resource.state.lock().unwrap();
    
    for mut text in query.iter_mut() {
        text.sections[0].value = format!(
            "Position: ({:.2}, {:.2}, {:.2})\nJSON: {}", 
            state.x, state.y, state.z,
            state.json_data
        );
    }
}

// Système pour gérer les interactions avec le slider
fn slider_interaction(
    mut ruby_resource: ResMut<RubyResource>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    // Simple contrôle au clavier pour le slider
    let mut state = ruby_resource.state.lock().unwrap();
    
    if keyboard_input.pressed(KeyCode::Up) {
        state.slider_value += 0.1;
        if state.slider_value > 10.0 {
            state.slider_value = 10.0;
        }
    }
    
    if keyboard_input.pressed(KeyCode::Down) {
        state.slider_value -= 0.1;
        if state.slider_value < 0.0 {
            state.slider_value = 0.0;
        }
    }
}

// Système pour changer d'état avec les touches de raccourci
fn state_keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Key1) {
        next_state.set(AppState::Visualisation);
    }
    
    if keyboard_input.just_pressed(KeyCode::Key2) {
        next_state.set(AppState::Console);
    }
    
    if keyboard_input.just_pressed(KeyCode::Key3) {
        next_state.set(AppState::Aide);
    }
}

// Système pour ouvrir le fichier Ruby dans l'éditeur par défaut
fn open_ruby_file(
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::E) {
        // Ouvrir le fichier dans l'éditeur par défaut
        if let Err(e) = open::that(RUBY_SCRIPT_PATH) {
            eprintln!("Erreur lors de l'ouverture du fichier: {:?}", e);
        }
    }
}

// Système de configuration pour l'état de visualisation
fn setup_visualisation(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    
    // Lumière
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    
    // Objet 3D contrôlé par Ruby
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::UVSphere { radius: 1.0, sectors: 32, stacks: 16 }.into()),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 1.0, 1.0),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        RubyObject,
    ));
    
    // Sol
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane { size: 20.0, subdivisions: 0 }.into()),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.3, 0.3, 0.3),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, -2.0, 0.0),
        ..default()
    });
    
    // Texte dynamique
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Texte dynamique de Ruby",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            text_anchor: bevy::sprite::Anchor::Center,
            ..default()
        },
        DynamicText,
    ));
    
    // Texte du slider
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Valeur ajustable: 5.0",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(0.0, 7.0, 0.0),
            text_anchor: bevy::sprite::Anchor::Center,
            ..default()
        },
        SliderText,
    ));
    
    // Affichage des données
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Position: (0.0, 0.0, 0.0)\nJSON: {}",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Left),
            transform: Transform::from_xyz(-9.0, 9.0, 0.0),
            text_anchor: bevy::sprite::Anchor::TopLeft,
            ..default()
        },
        DataDisplay,
    ));
    
    // Instructions
    commands.spawn(
        Text2dBundle {
            text: Text::from_section(
                "Touches: 1-Visualisation, 2-Console, 3-Aide, ↑↓-Ajuster, E-Éditer",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::YELLOW,
                },
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(0.0, -9.0, 0.0),
            text_anchor: bevy::sprite::Anchor::BottomCenter,
            ..default()
        },
    );
}

// Nettoyer la scène 3D quand on change d'état
fn cleanup_3d_scene(
    mut commands: Commands,
    query: Query<Entity, Or<(With<RubyObject>, With<DynamicText>, With<SliderText>, With<DataDisplay>)>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// Système de configuration pour l'état de console
fn setup_console(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    
    // Titre
    commands.spawn(
        TextBundle::from_section(
            "Console Ruby",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(20.0),
            ..default()
        }),
    );
    
    // Texte d'erreur
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::RED,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(60.0),
            left: Val::Px(20.0),
            ..default()
        }),
        ErrorText,
    ));
    
    // Console de sortie
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Regular.ttf"),
                font_size: 18.0,
                color: Color::rgb(0.8, 0.8, 0.8),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            left: Val::Px(20.0),
            right: Val::Px(20.0),
            bottom: Val::Px(50.0),
            ..default()
        }),
        ConsoleText,
    ));
    
    // Instructions
    commands.spawn(
        TextBundle::from_section(
            "Touches: 1-Visualisation, 2-Console, 3-Aide, E-Éditer",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::YELLOW,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(20.0),
            ..default()
        }),
    );
}

// Nettoyer la scène 2D quand on change d'état
fn cleanup_2d_scene(
    mut commands: Commands,
    query: Query<Entity, Or<(With<Camera2d>, With<ConsoleText>, With<ErrorText>)>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// Système de configuration pour l'état d'aide
fn setup_aide(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    
    // Titre
    commands.spawn(
        TextBundle::from_section(
            "À propos de l'application",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(20.0),
            ..default()
        }),
    );
    
    // Contenu d'aide
    let aide_text = "Cette application démontre l'intégration entre Ruby (via Artichoke) et Rust (via Bevy).\n\n\
                     Comment ça fonctionne:\n\
                     1. Le script Ruby dans 'scripts/main.rb' s'exécute en continu\n\
                     2. Les données calculées par Ruby sont affichées dans l'interface Bevy\n\
                     3. L'utilisateur peut ajuster des valeurs qui sont transmises à Ruby\n\
                     4. Les modifications du script Ruby sont détectées automatiquement\n\n\
                     Astuces:\n\
                     • Modifiez le script Ruby en temps réel pour voir les changements\n\
                     • Utilisez l'onglet Console pour voir la sortie standard de Ruby\n\
                     • Expérimentez avec différentes visualisations dans le script Ruby\n\n\
                     Créé avec ❤️ en Rust et Ruby";
    
    commands.spawn(
        TextBundle::from_section(
            aide_text,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                font_size: 24.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(70.0),
            left: Val::Px(20.0),
            right: Val::Px(20.0),
            ..default()
        }),
    );
    
    // Liens
    commands.spawn(
        TextBundle::from_section(
            "GitHub Artichoke: https://github.com/artichoke/artichoke\n\
             GitHub Bevy: https://github.com/bevyengine/bevy",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                font_size: 20.0,
                color: Color::rgb(0.5, 0.8, 1.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(50.0),
            left: Val::Px(20.0),
            ..default()
        }),
    );
    
    // Instructions
    commands.spawn(
        TextBundle::from_section(
            "Touches: 1-Visualisation, 2-Console, 3-Aide, E-Éditer",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::YELLOW,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(20.0),
            ..default()
        }),
    );
}

fn main() {
    // Créer notre état partagé pour Ruby
    let ruby_state = Arc::new(Mutex::new(RubyState::default()));
    
    // Clone de l'état pour le thread Ruby
    let ruby_state_thread = Arc::clone(&ruby_state);
    
    // On va exécuter Ruby dans un thread séparé
    thread::spawn(move || {
        // Initialiser l'interpréteur Ruby
        match artichoke::interpreter() {
            Ok(mut interp) => {
                // Charger la bibliothèque standard JSON
                let _ = interp.eval(b"require 'json'");
                
                // Boucle principale du thread Ruby
                while {
                    let running = ruby_state_thread.lock().unwrap().running;
                    running
                } {
                    // Récupérer les valeurs actuelles
                    let (time, dt, slider_value) = {
                        let state = ruby_state_thread.lock().unwrap();
                        (state.time, state.dt, state.slider_value)
                    };
                    
                    // Mise à jour des variables Ruby
                    let _ = interp.eval(format!("TIME = {}; DT = {}; USER_VALUE = {}", 
                                               time, dt, slider_value).as_bytes());
                    
                    // Lire le contenu du fichier Ruby
                    match fs::read_to_string(RUBY_SCRIPT_PATH) {
                        Ok(script_content) => {
                            // Capturer la sortie standard
                            let output = String::new();
                            let output_clone = output.clone();
                            
                            // Rediriger la sortie standard Ruby
                            let _ = interp.eval(r#"
                                class << $stdout
                                  alias_method :old_write, :write
                                  def write(str)
                                    old_write(str)
                                  end
                                end
                            "#.as_bytes());
                            
                            // Exécuter le script Ruby
                            match interp.eval(script_content.as_bytes()) {
                                Ok(_) => {
                                    // Essayer de récupérer les résultats depuis Ruby
                                    match interp.eval(b"RESULT.to_json") {
                                        Ok(result) => {
                                            // Récupérer la représentation JSON
                                            match interp.try_convert_mut(result) {
                                                Ok(json_str) => {
                                                    let json_str: String = json_str;
                                                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_str) {
                                                        let mut state = ruby_state_thread.lock().unwrap();
                                                        
                                                        // Extraire les valeurs du JSON
                                                        if let Some(x) = json.get("x").and_then(|v| v.as_f64()) {
                                                            state.x = x as f32;
                                                        }
                                                        if let Some(y) = json.get("y").and_then(|v| v.as_f64()) {
                                                            state.y = y as f32;
                                                        }
                                                        if let Some(z) = json.get("z").and_then(|v| v.as_f64()) {
                                                            state.z = z as f32;
                                                        }
                                                        if let Some(text) = json.get("text").and_then(|v| v.as_str()) {
                                                            state.text = text.to_string();
                                                        }
                                                        if let Some(color_array) = json.get("color").and_then(|v| v.as_array()) {
                                                            if color_array.len() >= 3 {
                                                                for (i, c) in color_array.iter().take(3).enumerate() {
                                                                    if let Some(value) = c.as_f64() {
                                                                        state.color[i] = (value as f32) / 255.0;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        if let Some(json_data) = json.get("json_data") {
                                                            state.json_data = json_data.to_string();
                                                        }
                                                        if let Some(rotation) = json.get("rotation").and_then(|v| v.as_f64()) {
                                                            state.rotation = rotation as f32;
                                                        }
                                                        
                                                        // Capturer la sortie standard
                                                        state.script_output = output_clone;
                                                        state.script_error = String::new();
                                                    }
                                                },
                                                Err(e) => {
                                                    // Erreur de conversion
                                                    let mut state = ruby_state_thread.lock().unwrap();
                                                    state.script_error = format!("Erreur de conversion: {:?}", e);
                                                }
                                            }
                                        },
                                        Err(e) => {
                                            // RESULT n'est pas défini ou n'est pas un objet
                                            let mut state = ruby_state_thread.lock().unwrap();
                                            state.script_error = format!("Erreur de résultat: {:?}", e);
                                        }
                                    }
                                },
                                Err(e) => {
                                    // Erreur d'évaluation du script
                                    let mut state = ruby_state_thread.lock().unwrap();
                                    state.script_error = format!("Erreur Ruby: {:?}", e);
                                }
                            }
                        },
                        Err(e) => {
                            // Erreur de lecture du fichier
                            let mut state = ruby_state_thread.lock().unwrap();
                            state.script_error = format!("Erreur de lecture: {:?}", e);
                        }
                    }
                    
                    // Petite pause pour ne pas surcharger le CPU
                    thread::sleep(Duration::from_millis(16));
                }
            },
            Err(e) => {
                // Erreur d'initialisation de Ruby
                let mut state = ruby_state_thread.lock().unwrap();
                state.script_error = format!("Erreur d'initialisation Ruby: {:?}", e);
            }
        }
    });
    
    // Configurer et lancer l'application Bevy
    App::new()
        // Ajout des plugins par défaut
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Universal App Ruby + Bevy".to_string(),
                    resolution: (1024.0, 768.0).into(),
                    ..default()
                }),
                ..default()
            }
        ))
        // Configuration pour éviter de surcharger le CPU
        .insert_resource(WinitSettings::desktop_app())
        // Ajouter notre ressource Ruby
        .insert_resource(RubyResource {
            state: ruby_state,
            last_update: Instant::now(),
        })
        // Configurer les états de l'application
        .add_state::<AppState>()
        // Ajouter les systèmes communs
        .add_systems(Update, (
            update_ruby_state,
            state_keyboard_input,
            open_ruby_file,
        ))
        // Systèmes pour l'état de visualisation
        .add_systems(OnEnter(AppState::Visualisation), setup_visualisation)
        .add_systems(OnExit(AppState::Visualisation), cleanup_3d_scene)
        .add_systems(Update, (
            update_ruby_object,
            update_dynamic_text,
            update_slider_text,
            update_data_display,
            slider_interaction,
        ).run_if(in_state(AppState::Visualisation)))
        // Systèmes pour l'état de console
        .add_systems(OnEnter(AppState::Console), setup_console)
        .add_systems(OnExit(AppState::Console), cleanup_2d_scene)
        .add_systems(Update, (
            update_console_text,
            update_error_text,
        ).run_if(in_state(AppState::Console)))
        // Systèmes pour l'état d'aide
        .add_systems(OnEnter(AppState::Aide), setup_aide)
        .add_systems(OnExit(AppState::Aide), cleanup_2d_scene)
        // Lancer l'application
        .run();
}