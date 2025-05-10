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
if [ ! -d "$PROJECT" ]; then
    log_error "Le projet $PROJECT n'existe pas. Exécutez d'abord la partie 1."
    exit 1
fi

cd "$PROJECT"

# Vérifier si le fichier main.rs existe dans le répertoire parent
if [ ! -f "../main.rs" ]; then
    log_error "Le fichier main.rs n'existe pas dans le répertoire parent."
    log_info "Assurez-vous que le fichier main.rs existe dans le même dossier que ce script."
    exit 1
fi

# Copier le fichier main.rs dans le dossier src du projet
log_info "📝 Copie du fichier main.rs dans le projet..."
cp "../main.rs" "src/main.rs"

# Vérifier si la copie a réussi
if [ $? -eq 0 ]; then
    log_success "✅ Fichier main.rs copié avec succès"
    echo "Exécutez maintenant la partie 5 pour compiler et exécuter le projet."
else
    log_error "❌ Erreur lors de la copie du fichier main.rs"
    exit 1
fi