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

log_info "📝 Création de l'interface Slint (ui.slint) - Version simplifiée"
cat > ui.slint <<'SLINT'
import { Button, Slider, ListView } from "std-widgets.slint";

component TabButton inherits Rectangle {
    in property <string> text;
    in property <bool> selected;
    callback clicked();
    height: 36px;
    width: 120px;
    background: selected ? #1e88e5 : transparent;
    border-radius: 4px;
    
    Text {
        text: root.text;
        color: selected ? white : #e0e0e0;
        font-size: 14px;
        font-weight: 500;
        horizontal-alignment: center;
        vertical-alignment: center;
    }
    
    TouchArea {
        clicked => {
            root.clicked();
        }
    }
}

component AnimatedCircle inherits Rectangle {
    in property <float> x: 0;
    in property <float> y: 0;
    in property <color> circle-color: #ff0000;
    
    width: 100%;
    height: 100%;
    background: #1e1e1e;
    border-radius: 8px;
    
    Rectangle {
        x: parent.width / 2 + root.x * parent.width * 0.04 - self.width / 2;
        y: parent.height / 2 + root.y * parent.height * 0.04 - self.height / 2;
        width: 40px;
        height: 40px;
        border-radius: self.width / 2;
        background: root.circle-color;
    }
}

export component MainWindow inherits Window {
    title: "Artichoke Ruby + Slint";
    min-width: 800px;
    min-height: 600px;
    background: #121212;
    
    // Propriétés d'entrée (depuis Rust)
    in property <float> x: 0;
    in property <float> y: 0;
    in property <string> dynamic-text: "Chargement...";
    in property <color> text-color: #ffffff;
    in property <float> slider-value: 5.0;
    in property <float> calculated-value: 2.5;
    in property <string> json-data: "{}";
    in property <string> console-output: "";
    in property <string> error-message: "";
    in property <[string]> list-items: [];
    
    // Callbacks (vers Rust)
    callback slider-changed(float);
    callback edit-ruby-script();
    callback clear-console();
    
    // État interne
    property <int> current-tab: 0;
    
    VerticalLayout {
        padding: 16px;
        spacing: 16px;
        
        // En-tête
        Rectangle {
            height: 60px;
            
            HorizontalLayout {
                Text {
                    text: "Artichoke Ruby + Slint";
                    font-size: 24px;
                    color: #2196f3;
                }
            }
        }
        
        // Onglets
        Rectangle {
            height: 40px;
            
            HorizontalLayout {
                alignment: start;
                spacing: 8px;
                
                TabButton {
                    text: "Visualisation";
                    selected: current-tab == 0;
                    clicked => { current-tab = 0; }
                }
                
                TabButton {
                    text: "Console";
                    selected: current-tab == 1;
                    clicked => { current-tab = 1; }
                }
                
                TabButton {
                    text: "Aide";
                    selected: current-tab == 2;
                    clicked => { current-tab = 2; }
                }
            }
        }
        
        // Contenu principal
        Rectangle {
            background: #1a1a1a;
            border-radius: 8px;
            
            if current-tab == 0 : VerticalLayout {
                padding: 16px;
                spacing: 16px;
                
                Text {
                    text: "Données Ruby";
                    font-size: 20px;
                    color: white;
                }
                
                // Texte dynamique
                Rectangle {
                    height: 60px;
                    background: #222222;
                    border-radius: 4px;
                    padding: 8px;
                    
                    VerticalLayout {
                        Text { 
                            text: "Texte dynamique:";
                            color: #888888;
                        }
                        Text { 
                            text: dynamic-text;
                            color: text-color;
                            font-size: 18px;
                        }
                    }
                }
                
                // Slider
                Rectangle {
                    height: 60px;
                    background: #222222;
                    border-radius: 4px;
                    padding: 8px;
                    
                    VerticalLayout {
                        Text { 
                            text: "Valeur ajustable: " + Math.round(slider-value * 10) / 10;
                            color: #888888;
                        }
                        Slider {
                            value: slider-value;
                            minimum: 0;
                            maximum: 10;
                            changed(val) => {
                                slider-changed(val);
                            }
                        }
                    }
                }
                
                // Positions et animation
                HorizontalLayout {
                    spacing: 16px;
                    
                    // Info positions
                    Rectangle {
                        width: 40%;
                        background: #222222;
                        border-radius: 4px;
                        padding: 8px;
                        
                        VerticalLayout {
                            Text { 
                                text: "Position: (" + Math.round(x * 100) / 100 + ", " + Math.round(y * 100) / 100 + ")";
                                color: white;
                            }
                            Text { 
                                text: "Valeur calculée: " + calculated-value;
                                color: white;
                            }
                        }
                    }
                    
                    // Cercle animé
                    AnimatedCircle {
                        width: 60%;
                        height: 150px;
                        x: root.x;
                        y: root.y;
                        circle-color: root.text-color;
                    }
                }
                
                // Liste d'items
                Rectangle {
                    height: 200px;
                    background: #222222;
                    border-radius: 4px;
                    padding: 8px;
                    
                    VerticalLayout {
                        Text { 
                            text: "Éléments calculés:";
                            color: #888888;
                        }
                        ListView {
                            for item in list-items : Rectangle {
                                height: 30px;
                                Text {
                                    text: item;
                                    color: white;
                                    vertical-alignment: center;
                                }
                            }
                        }
                    }
                }
            }
            
            if current-tab == 1 : VerticalLayout {
                padding: 16px;
                spacing: 16px;
                
                HorizontalLayout {
                    Text {
                        text: "Console Ruby";
                        font-size: 20px;
                        color: white;
                    }
                    
                    HorizontalLayout {
                        alignment: end;
                        Button {
                            text: "Éditer Script";
                            clicked => {
                                edit-ruby-script();
                            }
                        }
                        Button {
                            text: "Effacer";
                            clicked => {
                                clear-console();
                            }
                        }
                    }
                }
                
                if error-message != "" : Rectangle {
                    background: #500000;
                    border-radius: 4px;
                    padding: 8px;
                    
                    Text {
                        text: error-message;
                        color: #ff6666;
                        wrap: word-wrap;
                    }
                }
                
                Rectangle {
                    background: #222222;
                    border-radius: 4px;
                    padding: 8px;
                    
                    Text {
                        text: console-output;
                        color: #33ff33;
                        font-family: "monospace";
                        wrap: word-wrap;
                    }
                }
            }
            
            if current-tab == 2 : VerticalLayout {
                padding: 16px;
                spacing: 16px;
                
                Text {
                    text: "À propos";
                    font-size: 20px;
                    color: white;
                }
                
                Rectangle {
                    background: #222222;
                    border-radius: 4px;
                    padding: 16px;
                    
                    VerticalLayout {
                        spacing: 8px;
                        
                        Text {
                            text: "Cette application démontre l'intégration entre Ruby (via Artichoke) et Rust (via Slint).";
                            color: white;
                            wrap: word-wrap;
                        }
                        
                        Text {
                            text: "Le script Ruby (scripts/main.rb) s'exécute en continu et peut être modifié en temps réel.";
                            color: white;
                            wrap: word-wrap;
                        }
                        
                        Text {
                            text: "L'interface utilisateur est construite avec Slint, une bibliothèque moderne pour Rust.";
                            color: white;
                            wrap: word-wrap;
                        }
                    }
                }
            }
        }
    }
}
SLINT

log_success "✅ Interface Slint créée avec succès (version simplifiée)"
echo "Exécutez maintenant la partie 4 pour créer le code Rust principal."