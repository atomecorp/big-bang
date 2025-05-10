#!/bin/bash
set -e
PROJECT="universal_app_ruby_egui"  # Nouveau nom pour éviter les conflits
echo "📦 Creating project: $PROJECT"
cargo new "$PROJECT" --bin
cd "$PROJECT"
echo "📁 Creating script folder"
mkdir -p scripts
echo "📝 Writing example Ruby script"
cat > scripts/main.rb <<'RUBY'
# Le script principal Ruby qui sera exécuté à chaque frame
puts "Ruby says: Frame time = #{DT}"

# Fonction pour calculer un résultat simple
def calculate_result(time)
  x = Math.sin(time) * 10.0
  y = Math.cos(time) * 8.0
  
  # Petit texte aléatoire en fonction du temps
  texts = [
    "Bonjour de Ruby!",
    "Hello from Ruby!",
    "Ruby + Rust = ❤️",
    "Modifiez ce script!",
    "Artichoke fonctionne!",
    "Egui est génial!"
  ]
  
  text_index = (time * 0.2).to_i % texts.length
  current_text = texts[text_index]
  
  # Couleur qui change avec le temps
  r = (Math.sin(time) * 0.5 + 0.5) * 255.0
  g = (Math.sin(time + 2.0) * 0.5 + 0.5) * 255.0
  b = (Math.sin(time + 4.0) * 0.5 + 0.5) * 255.0
  
  # Valeur qui peut être modifiée par l'utilisateur (définie initialement à 5.0)
  slider_value = USER_VALUE || 5.0
  
  # Traitement JSON simple
  data = '{ "name": "Jeezs", "value": 42 }'
  parsed = JSON.parse(data)
  
  # Retourne un hash avec toutes les valeurs
  {
    x: x,
    y: y,
    text: current_text,
    color: [r, g, b],
    slider_value: slider_value,
    json_data: parsed
  }
end

# Calculer notre résultat
result = calculate_result(TIME)

# Afficher des informations dans la console
puts "Position: #{result[:x]}, #{result[:y]}"
puts "Text: #{result[:text]}"
puts "Color: #{result[:color]}"
puts "Slider value: #{result[:slider_value]}"
puts "JSON data: #{result[:json_data]}"

# Retourner le résultat pour Rust
RESULT = result
RUBY
echo "🛠 Writing Cargo.toml with Artichoke from Git and Egui"
cat > Cargo.toml <<'TOML'
[package]
name = "universal_app_ruby_egui"
version = "0.1.0"
edition = "2021"

[dependencies]
eframe = "0.22.0"  # Egui framework avec intégration native
egui = "0.22.0"    # La bibliothèque GUI elle-même
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
artichoke = { git = "https://github.com/artichoke/artichoke", branch = "trunk" }
# Pour ouvrir des fichiers dans l'éditeur par défaut
open = "3.2"
TOML
echo "🧠 Writing main.rs avec Egui pour l'interface utilisateur"
mkdir -p src
cat > src/main.rs <<'RUST'
use artichoke::prelude::*;
use eframe::egui;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// Le chemin vers le script Ruby
const RUBY_SCRIPT_PATH: &str = "scripts/main.rb";

// Les différents onglets de l'interface
#[derive(PartialEq, Clone, Copy)]
enum Tab {
    Visualisation,
    Console,
    Aide,
}

// Structure pour passer des données entre threads
struct RubyState {
    dt: f32,
    time: f32,
    x: f32,
    y: f32,
    text: String,
    color: [f32; 3],
    slider_value: f32,
    json_data: String,
    script_output: String,
    script_error: String,
    running: bool,
}

impl Default for RubyState {
    fn default() -> Self {
        Self {
            dt: 0.0,
            time: 0.0,
            x: 0.0,
            y: 0.0,
            text: "Chargement...".to_string(),
            color: [1.0, 1.0, 1.0],
            slider_value: 5.0,
            json_data: "{}".to_string(),
            script_output: String::new(),
            script_error: String::new(),
            running: true,
        }
    }
}

// Notre application principale
struct MyApp {
    state: Arc<Mutex<RubyState>>,
    last_time: Instant,
    current_tab: Tab,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Personnaliser l'apparence Egui ici
        let mut visuals = egui::Visuals::dark();
        visuals.window_rounding = 8.0.into();
        visuals.window_shadow.extrusion = 8.0;
        cc.egui_ctx.set_visuals(visuals);
        
        // Créer notre état
        let state = Arc::new(Mutex::new(RubyState::default()));
        
        // Clone de l'état pour le thread Ruby
        let ruby_state = Arc::clone(&state);
        
        // On va exécuter Ruby dans un thread séparé
        thread::spawn(move || {
            // Initialiser l'interpréteur Ruby
            match artichoke::interpreter() {
                Ok(mut interp) => {
                    // Rediriger la sortie standard (méthode alternative)
                    let _ = interp.eval(r#"
                        class << $stdout
                          alias_method :old_write, :write
                          def write(str)
                            old_write(str)
                            # La sortie sera capturée par Rust via la console standard
                            old_write(str)
                          end
                        end
                    "#.as_bytes());
                    
                    // Charger la bibliothèque standard JSON
                    let _ = interp.eval(b"require 'json'");
                    
                    // Boucle principale du thread Ruby
                    while {
                        let running = ruby_state.lock().unwrap().running;
                        running
                    } {
                        // Récupérer les valeurs actuelles
                        let (time, dt, slider_value) = {
                            let state = ruby_state.lock().unwrap();
                            (state.time, state.dt, state.slider_value)
                        };
                        
                        // Mise à jour des variables Ruby
                        let _ = interp.eval(format!("TIME = {}; DT = {}; USER_VALUE = {}", 
                                                   time, dt, slider_value).as_bytes());
                        
                        // Lire le contenu du fichier Ruby
                        match fs::read_to_string(RUBY_SCRIPT_PATH) {
                            Ok(script_content) => {
                                // Exécuter le script Ruby et capturer la sortie via stderr
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
                                                            let mut state = ruby_state.lock().unwrap();
                                                            
                                                            // Extraire les valeurs du JSON
                                                            if let Some(x) = json.get("x").and_then(|v| v.as_f64()) {
                                                                state.x = x as f32;
                                                            }
                                                            if let Some(y) = json.get("y").and_then(|v| v.as_f64()) {
                                                                state.y = y as f32;
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
                                                            
                                                            // Captures de sorties via stdout normale
                                                            state.script_error = String::new();
                                                        }
                                                    },
                                                    Err(e) => {
                                                        // Erreur de conversion
                                                        let mut state = ruby_state.lock().unwrap();
                                                        state.script_error = format!("Erreur de conversion: {:?}", e);
                                                    }
                                                }
                                            },
                                            Err(e) => {
                                                // RESULT n'est pas défini ou n'est pas un objet
                                                let mut state = ruby_state.lock().unwrap();
                                                state.script_error = format!("Erreur de résultat: {:?}", e);
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        // Erreur d'évaluation du script
                                        let mut state = ruby_state.lock().unwrap();
                                        state.script_error = format!("Erreur Ruby: {:?}", e);
                                    }
                                }
                            },
                            Err(e) => {
                                // Erreur de lecture du fichier
                                let mut state = ruby_state.lock().unwrap();
                                state.script_error = format!("Erreur de lecture: {:?}", e);
                            }
                        }
                        
                        // Petite pause pour ne pas surcharger le CPU
                        thread::sleep(Duration::from_millis(16));
                    }
                },
                Err(e) => {
                    // Erreur d'initialisation de Ruby
                    let mut state = ruby_state.lock().unwrap();
                    state.script_error = format!("Erreur d'initialisation Ruby: {:?}", e);
                }
            }
        });
        
        MyApp {
            state,
            last_time: Instant::now(),
            current_tab: Tab::Visualisation,
        }
    }
    
    // Fonction pour rendre l'onglet de visualisation
    fn render_tab_visualisation(&mut self, ui: &mut egui::Ui, x: f32, y: f32, text: &str, color: [f32; 3], slider_value: f32, json_data: &str) {
        // Visualisation des données de Ruby
        ui.heading("Données de Ruby");
        
        // Slider qui peut être modifié par l'utilisateur
        let mut slider_val = slider_value;
        ui.add(egui::Slider::new(&mut slider_val, 0.0..=10.0)
            .text("Valeur ajustable")
            .show_value(true));
        
        // Mettre à jour la valeur du slider si elle a changé
        if slider_val != slider_value {
            let mut state = self.state.lock().unwrap();
            state.slider_value = slider_val;
        }
        
        // Texte dynamique provenant de Ruby
        ui.add_space(20.0);
        ui.heading("Texte dynamique:");
        ui.label(
            egui::RichText::new(text)
                .size(24.0)
                .color(egui::Color32::from_rgb(
                    (color[0] * 255.0) as u8,
                    (color[1] * 255.0) as u8,
                    (color[2] * 255.0) as u8
                ))
        );
        
        // Afficher les valeurs numériques
        ui.add_space(20.0);
        ui.horizontal(|ui| {
            ui.strong("Position: ");
            ui.label(format!("({:.2}, {:.2})", x, y));
        });
        
        // Visualisation graphique - une petite animation
        ui.add_space(20.0);
        let rect = ui.available_rect_before_wrap();
        let painter = ui.painter();
        
        // Fond
        painter.rect_filled(
            rect, 
            8.0, 
            egui::Color32::from_gray(30)
        );
        
        // Point animé
        let center_x = rect.min.x + rect.width() * 0.5;
        let center_y = rect.min.y + rect.height() * 0.5;
        let pos_x = center_x + (x * rect.width() * 0.4 / 10.0);
        let pos_y = center_y + (y * rect.height() * 0.4 / 10.0);
        
        painter.circle_filled(
            egui::pos2(pos_x, pos_y),
            20.0,
            egui::Color32::from_rgb(
                (color[0] * 255.0) as u8,
                (color[1] * 255.0) as u8,
                (color[2] * 255.0) as u8
            )
        );
        
        // JSON data
        ui.add_space(20.0);
        ui.collapsing("Données JSON", |ui| {
            ui.label(format!("{}", json_data));
        });
    }
    
    // Fonction pour rendre l'onglet console
    fn render_tab_console(&mut self, ui: &mut egui::Ui, output: &str, error: &str) {
        // Console de sortie
        ui.heading("Sortie du script Ruby");
        
        if !error.is_empty() {
            ui.label(
                egui::RichText::new(error)
                    .color(egui::Color32::RED)
            );
            ui.separator();
        }
        
        // Zone de texte scrollable pour la sortie
        egui::ScrollArea::vertical()
            .stick_to_bottom(true)
            .max_height(ui.available_height() - 40.0)
            .show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut output.to_string())
                        .font(egui::FontId::monospace(14.0))
                        .desired_width(f32::INFINITY)
                        .interactive(false)
                );
            });
        
        // Bouton pour ouvrir le fichier Ruby dans l'éditeur par défaut
        ui.add_space(10.0);
        if ui.button("Éditer le script Ruby").clicked() {
            // Ouvrir le fichier dans l'éditeur par défaut
            if let Err(e) = open::that(RUBY_SCRIPT_PATH) {
                eprintln!("Erreur lors de l'ouverture du fichier: {:?}", e);
            }
        }
    }
    
    // Fonction pour rendre l'onglet aide
    fn render_tab_aide(&mut self, ui: &mut egui::Ui) {
        // Aide et informations
        ui.heading("À propos de l'application");
        
        ui.add_space(10.0);
        ui.label("Cette application démontre l'intégration entre Ruby (via Artichoke) et Rust (via Egui).");
        
        ui.add_space(20.0);
        ui.heading("Comment ça fonctionne");
        ui.label("1. Le script Ruby dans 'scripts/main.rb' s'exécute en continu");
        ui.label("2. Les données calculées par Ruby sont affichées dans l'interface Egui");
        ui.label("3. L'utilisateur peut ajuster des valeurs qui sont transmises à Ruby");
        ui.label("4. Les modifications du script Ruby sont détectées automatiquement");
        
        ui.add_space(20.0);
        ui.heading("Astuces");
        ui.label("• Modifiez le script Ruby en temps réel pour voir les changements");
        ui.label("• Utilisez l'onglet Console pour voir la sortie standard de Ruby");
        ui.label("• Expérimentez avec différentes visualisations dans le script Ruby");
        
        ui.add_space(20.0);
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.add_space(10.0);
            ui.hyperlink_to("GitHub Artichoke", "https://github.com/artichoke/artichoke");
            ui.hyperlink_to("GitHub Egui", "https://github.com/emilk/egui");
            ui.label("Créé avec ❤️ en Rust et Ruby");
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Calculer le delta time
        let now = Instant::now();
        let dt = now.duration_since(self.last_time).as_secs_f32();
        self.last_time = now;
        
        // Mettre à jour le temps et le delta time
        {
            let mut state = self.state.lock().unwrap();
            state.time += dt;
            state.dt = dt;
        }
        
        // Demander une mise à jour continue
        ctx.request_repaint();
        
        // Récupérer les valeurs actuelles
        let (x, y, text, color, slider_value, json_data, output, error) = {
            let state = self.state.lock().unwrap();
            (
                state.x, 
                state.y, 
                state.text.clone(), 
                state.color,
                state.slider_value,
                state.json_data.clone(),
                state.script_output.clone(),
                state.script_error.clone()
            )
        };
        
        // Panneau principal
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Artichoke Ruby + Egui");
            ui.horizontal(|ui| {
                ui.strong("FPS:");
                ui.label(format!("{:.1}", 1.0 / dt));
            });
            
            // Séparateur
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            
            // Interface avec onglets
            egui::TopBottomPanel::top("tabs").show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.selectable_label(self.current_tab == Tab::Visualisation, "Visualisation").clicked() {
                        self.current_tab = Tab::Visualisation;
                    }
                    if ui.selectable_label(self.current_tab == Tab::Console, "Console").clicked() {
                        self.current_tab = Tab::Console;
                    }
                    if ui.selectable_label(self.current_tab == Tab::Aide, "Aide").clicked() {
                        self.current_tab = Tab::Aide;
                    }
                });
            });
            
            ui.add_space(10.0);
            
            // Contenu en fonction de l'onglet
            match self.current_tab {
                Tab::Visualisation => self.render_tab_visualisation(ui, x, y, &text, color, slider_value, &json_data),
                Tab::Console => self.render_tab_console(ui, &output, &error),
                Tab::Aide => self.render_tab_aide(ui),
            }
        });
    }
    
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Signaler au thread Ruby de s'arrêter
        let mut state = self.state.lock().unwrap();
        state.running = false;
    }
}

fn main() -> Result<(), eframe::Error> {
    // Options pour l'application native
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(840.0, 640.0)),
        min_window_size: Some(egui::vec2(600.0, 400.0)),
        ..Default::default()
    };
    
    // Lancer l'application Egui
    eframe::run_native(
        "Universal App Ruby + Egui",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc)))
    )
}
RUST
echo "✅ Project $PROJECT is ready. You can now:"
echo "cd $PROJECT && cargo build && cargo run"
echo "🚀 Building and running the project..."
cargo build && cargo run