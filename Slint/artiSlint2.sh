#!/bin/bash
set -e

# Définir les couleurs pour les logs
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Fonction de log
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

PROJECT="universal_app_ruby_slint"
if [ ! -d "$PROJECT" ]; then
    echo -e "${RED}[ERROR]${NC} Le projet $PROJECT n'existe pas. Exécutez d'abord la partie 1."
    exit 1
fi

cd "$PROJECT"

log_info "📝 Création du script Ruby (scripts/main.rb)"
cat > scripts/main.rb <<'RUBY'
# Le script principal Ruby qui sera exécuté à chaque frame
puts "Ruby says: Frame time = #{DT}"

# Fonction pour calculer un résultat simple
def calculate_result(time)
  # Position du cercle animé
  x = Math.sin(time) * 10.0
  y = Math.cos(time) * 8.0
  
  # Petit texte aléatoire en fonction du temps
  texts = [
    "Bonjour de Ruby!",
    "Hello from Ruby!",
    "Ruby + Rust + Slint = ❤️",
    "Modifiez ce script!",
    "Artichoke fonctionne!",
    "Interface moderne!"
  ]
  
  text_index = (time * 0.2).to_i % texts.length
  current_text = texts[text_index]
  
  # Couleur qui change avec le temps (format RGB 0-1)
  r = Math.sin(time) * 0.5 + 0.5
  g = Math.sin(time + 2.0) * 0.5 + 0.5
  b = Math.sin(time + 4.0) * 0.5 + 0.5
  
  # Valeur qui peut être modifiée par l'utilisateur
  slider_value = USER_VALUE || 5.0
  
  # Calculer une valeur basée sur le slider
  calculated_value = slider_value * slider_value / 10.0
  
  # Traitement JSON simple
  data = '{ "name": "Jeezs", "value": 42 }'
  parsed = JSON.parse(data)
  
  # Générer quelques éléments de liste pour l'interface
  list_items = []
  (1..5).each do |i|
    list_items << "Item #{i}: valeur = #{(i * slider_value).round(1)}"
  end
  
  # Retourne un hash avec toutes les valeurs
  {
    x: x,
    y: y,
    text: current_text,
    color: [r, g, b],
    slider_value: slider_value,
    calculated_value: calculated_value,
    json_data: parsed,
    list_items: list_items
  }
end

# Calculer notre résultat
result = calculate_result(TIME)

# Afficher des informations dans la console
puts "Position: #{result[:x]}, #{result[:y]}"
puts "Text: #{result[:text]}"
puts "Color: #{result[:color].map { |c| (c * 255).to_i }.join(', ')}"
puts "Slider value: #{result[:slider_value]}"
puts "Calculated value: #{result[:calculated_value]}"
puts "JSON data: #{result[:json_data]}"
puts "List items: #{result[:list_items]}"

# Retourner le résultat pour Rust
RESULT = result
RUBY

log_success "✅ Script Ruby créé avec succès"
echo "Exécutez maintenant la partie 3 pour créer l'interface utilisateur Slint."