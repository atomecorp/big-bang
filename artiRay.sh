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
puts "Ruby says: Frame time = #{DT}"
data = '{ "name": "Jeezs" }'
name = JSON.parse(data)["name"]
puts "JSON parsed name: #{name}"
RUBY
echo "🛠 Writing Cargo.toml with Artichoke from Git"
cat > Cargo.toml <<'TOML'
[package]
name = "universal_app_ruby_extn"
version = "0.1.0"
edition = "2021"

[dependencies]
minifb = "0.25.0"
serde_json = "1.0"
artichoke = { git = "https://github.com/artichoke/artichoke", branch = "trunk" }
TOML
echo "🧠 Writing main.rs with minimal functionality"
mkdir -p src
cat > src/main.rs <<'RUST'
use artichoke::prelude::*;
use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Créer un buffer de pixels pour l'affichage
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    for i in 0..WIDTH * HEIGHT {
        buffer[i] = 0x000000; // Remplir de noir
    }

    // Créer une fenêtre
    let mut window = Window::new(
        "Universal App with Ruby",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Window creation failed: {}", e);
    });

    // Limite à ~60 fps
    window.limit_update_rate(Some(Duration::from_micros(16666)));

    // Initialiser l'interpréteur Ruby
    let mut interp = artichoke::interpreter()?;
    
    // Charger la bibliothèque standard JSON 
    interp.eval(b"require 'json'")?;
    
    // Variables pour mesurer le temps entre les frames
    let mut last_instant = Instant::now();
    
    // Boucle principale
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Calculer le delta time
        let current_instant = Instant::now();
        let delta = current_instant.duration_since(last_instant);
        let dt = delta.as_secs_f32();
        last_instant = current_instant;
        
        // Mettre à jour Ruby
        let dt_str = format!("DT = {}", dt);
        let _ = interp.eval(dt_str.as_bytes())?;
        let _ = interp.eval(include_str!("../scripts/main.rb").as_bytes())?;
        
        // Dessiner un texte à 20,20 (simulation simple)
        let message = "Ruby Artichoke Active";
        for y in 20..40 {
            for x in 20..(20 + message.len() * 8) {
                if y >= HEIGHT || x >= WIDTH {
                    continue;
                }
                buffer[y * WIDTH + x] = 0xFFFFFF; // Couleur blanche
            }
        }
        
        // Mettre à jour la fenêtre
        window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;
    }
    
    Ok(())
}
RUST
echo "✅ Project $PROJECT is ready. You can now:"
echo "cd $PROJECT && cargo build && cargo run"
echo "🚀 Building and running the project..."
cargo build && cargo run