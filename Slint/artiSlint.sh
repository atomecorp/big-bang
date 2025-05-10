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

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

PROJECT="universal_app_ruby_slint"
log_info "📦 Création du projet: $PROJECT"

if [ -d "$PROJECT" ]; then
    log_warning "Le dossier $PROJECT existe déjà. Voulez-vous le supprimer ? (o/n)"
    read -r response
    if [[ "$response" =~ ^[oO]$ ]]; then
        log_info "Suppression du dossier existant..."
        rm -rf "$PROJECT"
    else
        log_error "Opération annulée. Veuillez choisir un autre nom ou supprimer le dossier."
        exit 1
    fi
fi

cargo new "$PROJECT" --bin
cd "$PROJECT"

log_info "📁 Création du dossier scripts"
mkdir -p scripts

log_info "📝 Ajout de Cargo.toml"
cat > Cargo.toml <<'TOML'
[package]
name = "universal_app_ruby_slint"
version = "0.1.0"
edition = "2021"

[dependencies]
slint = "1.2.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
artichoke = { git = "https://github.com/artichoke/artichoke", branch = "trunk" }
open = "3.2"
TOML

log_success "✅ Préparation du projet terminée"
echo "Exécutez maintenant la partie 2 pour créer les fichiers de script et d'interface."