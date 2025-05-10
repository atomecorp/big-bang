#!/bin/bash
set -e
PROJECT="universal_app_ruby_extn"
echo "📦 Creating project: $PROJECT"
cargo new "$PROJECT" --bin
cd "$PROJECT"
echo "📁 Creating script folder"
mkdir -p scripts
echo "📝 Writing example Ruby script"
cat > scripts/main.rb <<'RUBY'
# Le script principal Ruby qui sera exécuté à chaque frame
puts "Ruby says: Frame time = #{DT}"

# Mise à jour des positions selon le temps
@cube_x = Math.sin(TIME) * 2.0
@cube_y = Math.cos(TIME) * 1.5
@cube_z = Math.sin(TIME * 0.5) * 3.0 - 6.0

puts "Ruby says: Cube position = #{@cube_x}, #{@cube_y}, #{@cube_z}"

# Traitement JSON simple
data = '{ "name": "Jeezs" }'
name = JSON.parse(data)["name"]
puts "JSON parsed name: #{name}"

# Retourner les valeurs de position (seront ignorées par Rust)
[@cube_x, @cube_y, @cube_z]
RUBY
echo "🛠 Writing Cargo.toml with Artichoke from Git and Macroquad"
cat > Cargo.toml <<'TOML'
[package]
name = "universal_app_ruby_extn"
version = "0.1.0"
edition = "2021"

[dependencies]
macroquad = "0.4"
serde_json = "1.0"
artichoke = { git = "https://github.com/artichoke/artichoke", branch = "trunk" }
TOML
echo "🧠 Writing main.rs with chargement dynamique du script Ruby"
mkdir -p src
cat > src/main.rs <<'RUST'
use artichoke::prelude::*;
use macroquad::prelude::*;
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

// Variables globales pour la position du cube
static mut CUBE_X: f32 = 0.0;
static mut CUBE_Y: f32 = 0.0;
static mut CUBE_Z: f32 = -6.0;
static mut TIME: f32 = 0.0;

// Flag pour indiquer que le programme se termine
static QUIT_FLAG: AtomicBool = AtomicBool::new(false);

// Le chemin vers le script Ruby
const RUBY_SCRIPT_PATH: &str = "scripts/main.rb";

// Configuration de la fenêtre Macroquad
fn window_conf() -> Conf {
    Conf {
        window_title: "Universal App with Ruby & 3D".to_string(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        ..Default::default()
    }
}

// Animation du cube en fonction du temps
fn animate_cube(time: f32) -> (f32, f32, f32) {
    (
        f32::sin(time) * 2.0,
        f32::cos(time) * 1.5,
        f32::sin(time * 0.5) * 3.0 - 6.0
    )
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // On va exécuter Ruby dans un thread séparé
    thread::spawn(|| {
        // Initialiser l'interpréteur Ruby
        match artichoke::interpreter() {
            Ok(mut interp) => {
                // Charger la bibliothèque standard JSON
                let _ = interp.eval(b"require 'json'");
                
                // Boucle principale du thread Ruby
                while !QUIT_FLAG.load(Ordering::Relaxed) {
                    // Récupérer le temps actuel et le delta time depuis les variables globales
                    let (time, dt) = unsafe { (TIME, get_frame_time()) };
                    
                    // Mettre à jour les variables Ruby
                    let _ = interp.eval(format!("TIME = {}; DT = {}", time, dt).as_bytes());
                    
                    // Lire le contenu du fichier Ruby
                    match fs::read_to_string(RUBY_SCRIPT_PATH) {
                        Ok(script_content) => {
                            // Exécuter le script Ruby
                            match interp.eval(script_content.as_bytes()) {
                                Ok(_) => {
                                    // Le script s'est bien exécuté
                                    // On va simplement animer le cube dans le thread principal
                                    let (x, y, z) = animate_cube(time);
                                    unsafe {
                                        CUBE_X = x;
                                        CUBE_Y = y;
                                        CUBE_Z = z;
                                    }
                                },
                                Err(e) => {
                                    // Afficher l'erreur Ruby
                                    eprintln!("Erreur Ruby: {:?}", e);
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("Erreur lors de la lecture du fichier script: {:?}", e);
                        }
                    }
                    
                    // Petite pause pour ne pas surcharger le CPU
                    thread::sleep(Duration::from_millis(16));
                }
            },
            Err(e) => {
                eprintln!("Erreur lors de l'initialisation de Ruby: {:?}", e);
            }
        }
    });
    
    // Boucle principale de Macroquad
    loop {
        // Mise à jour du temps
        unsafe {
            TIME += get_frame_time();
        }
        
        // Quitter si la touche Escape est pressée
        if is_key_pressed(KeyCode::Escape) {
            QUIT_FLAG.store(true, Ordering::Relaxed);
            break;
        }
        
        // Effacer l'écran
        clear_background(BLACK);
        
        // Mode 3D
        set_camera(&Camera3D {
            position: vec3(0.0, 1.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(0.0, 0.0, -6.0),
            ..Default::default()
        });
        
        // Dessiner le sol
        draw_grid(20, 1.0, DARKGRAY, GRAY);
        
        // Dessiner un cube 3D qui est animé
        unsafe {
            draw_cube(vec3(CUBE_X, CUBE_Y, CUBE_Z), vec3(1.0, 1.0, 1.0), None, RED);
            draw_cube_wires(vec3(CUBE_X, CUBE_Y, CUBE_Z), vec3(1.0, 1.0, 1.0), WHITE);
        }
        
        // Retour au mode 2D pour l'interface
        set_default_camera();
        
        // Dessiner du texte 2D (avec des coordonnées f32)
        draw_text("Ruby + Macroquad 3D", 20.0, 20.0, 30.0, WHITE);
        
        // Informations FPS
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 50.0, 20.0, GREEN);
        
        // Afficher la position du cube
        unsafe {
            draw_text(&format!("Cube: ({:.2}, {:.2}, {:.2})", CUBE_X, CUBE_Y, CUBE_Z), 
                      20.0, 80.0, 20.0, YELLOW);
        }
        
        // Instructions pour l'utilisateur
        draw_text("Modifiez scripts/main.rb pour changer le comportement", 20.0, 110.0, 20.0, SKYBLUE);
        draw_text("Appuyez sur ESC pour quitter", 20.0, 130.0, 20.0, PINK);

        // Attendre la prochaine frame
        next_frame().await;
    }
    
    Ok(())
}
RUST
echo "✅ Project $PROJECT is ready. You can now:"
echo "cd $PROJECT && cargo build && cargo run"
echo "🚀 Building and running the project..."
cargo build && cargo run