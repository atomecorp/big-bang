#!/bin/bash

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
if [ ! -d "$PROJECT" ]; then
    log_error "Le projet $PROJECT n'existe pas. Exécutez d'abord les parties 1 à 4."
    exit 1
fi

cd "$PROJECT"

log_info "🔧 Compilation du projet..."
cargo build

if [ $? -ne 0 ]; then
    log_error "La compilation a échoué."
    log_info "Suggestions de débug :"
    log_info "- Vérifiez que vous avez bien exécuté les parties 1 à 4 dans l'ordre."
    log_info "- Si des erreurs sont liées à Artichoke, vous pourriez avoir besoin d'installer des dépendances supplémentaires."
    log_info "- Si des erreurs sont liées à Slint, assurez-vous d'avoir les dépendances graphiques installées."
    exit 1
fi

log_success "✅ Compilation réussie!"
log_info "🚀 Exécution de l'application..."
echo "----------------------------------------"
echo "Appuyez sur Ctrl+C pour quitter"
echo "----------------------------------------"

cargo run

log_info "Instructions d'utilisation :"
log_info "1. Naviguez entre les onglets (Visualisation, Console, Aide)"
log_info "2. Ajustez le slider pour voir les valeurs changer en temps réel"
log_info "3. Modifiez le script Ruby (scripts/main.rb) dans un éditeur externe et les changements seront détectés automatiquement"
log_info "4. Consultez l'onglet Console pour voir la sortie du script Ruby"

log_info "Pour relancer l'application:"
echo "cd $PROJECT && cargo run"