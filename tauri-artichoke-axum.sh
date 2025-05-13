#!/bin/bash
set -e

# Créer un dossier temporaire pour l'analyse
TEMP_DIR="tauri_template_analysis"
mkdir -p "$TEMP_DIR"
cd "$TEMP_DIR"

# Créer un package.json minimal
cat > package.json <<'JSON'
{
  "name": "tauri_template",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "echo 'This is a placeholder dev script'"
  }
}
JSON

echo "🔍 Initializing a template Tauri project to analyze its structure..."
echo "📌 IMPORTANT: Répondez simplement aux questions interactives."
echo "📌 Utilisez le nom par défaut et le titre par défaut."
echo ""

# Initialiser un projet Tauri pour voir la structure générée
cargo tauri init

# Si l'initialisation réussit, analyser le projet
if [ -d "src-tauri" ] && [ -f "src-tauri/tauri.conf.json" ]; then
    echo ""
    echo "✅ Projet template créé avec succès!"
    echo ""
    echo "🔍 Structure du tauri.conf.json généré:"
    cat src-tauri/tauri.conf.json
    
    echo ""
    echo "🔍 Structure du Cargo.toml généré:"
    cat src-tauri/Cargo.toml
    
    echo ""
    echo "📋 Utilisons ces fichiers comme référence pour créer un projet compatible avec Artichoke et Axum."
    echo "🔄 Maintenant, nous pouvons créer un script qui reproduit cette structure tout en y intégrant Artichoke et Axum."
else
    echo "❌ L'initialisation du projet n'a pas créé la structure attendue."
fi