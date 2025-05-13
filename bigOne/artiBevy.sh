#!/bin/bash
set -e
PROJECT="universal_app_ruby_bevy"  # Nom du projet
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
  z = Math.sin(time * 0.5) * 5.0  # Ajout d'une coordonnée Z pour Bevy 3D
  
  # Petit texte aléatoire en fonction du temps
  texts = [
    "Bonjour de Ruby!",
    "Hello from Ruby!",
    "Ruby + Rust = ❤️",
    "Modifiez ce script!",
    "Artichoke fonctionne!",
    "Bevy est génial!"
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
    z: z,
    text: current_text,
    color: [r, g, b],
    slider_value: slider_value,
    json_data: parsed,
    rotation: time * 0.5  # Ajout de rotation pour l'objet 3D
  }
end

# Calculer notre résultat
result = calculate_result(TIME)

# Afficher des informations dans la console
puts "Position: #{result[:x]}, #{result[:y]}, #{result[:z]}"
puts "Text: #{result[:text]}"
puts "Color: #{result[:color]}"
puts "Slider value: #{result[:slider_value]}"
puts "JSON data: #{result[:json_data]}"

# Retourner le résultat pour Rust
RESULT = result
RUBY

echo "🛠 Writing Cargo.toml with Artichoke from Git and Bevy"
cat > Cargo.toml <<'TOML'
[package]
name = "universal_app_ruby_bevy"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy = "0.12.1"  # Version récente de Bevy
artichoke = { git = "https://github.com/artichoke/artichoke", branch = "trunk" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
# Pour ouvrir des fichiers dans l'éditeur par défaut
open = "3.2"

# Optimisations pour la compilation
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
TOML

echo "🧠 Writing main.rs with Bevy integration"
cp ../main.rs src/main.rs

echo "✅ Project $PROJECT is ready. You can now:"
echo "cd $PROJECT && cargo build && cargo run"
echo "🚀 Building and running the project..."
cargo build && cargo run