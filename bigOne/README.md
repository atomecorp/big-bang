# UI Modulaire OS-Like avec Bevy et DSL Ruby

Ce projet est une implémentation d'un système d'interface utilisateur modulaire et multiplateforme basé sur le moteur Bevy et un DSL (Domain Specific Language) inspiré de Ruby. Il permet de créer des interfaces graphiques riches, interactives et facilement personnalisables via un langage de script.

## 🎯 Fonctionnalités

- Interface graphique complète avec fenêtres, boutons, textes, images, etc.
- Langage DSL déclaratif pour définir l'interface utilisateur
- Support du drag-and-drop, redimensionnement et interactions utilisateur
- Rendu 2D et 3D intégré via Bevy
- Support SVG et canvas
- Système de layout flexible avec Flexbox
- Hot-reload des scripts d'interface
- Compatible multiplateforme (desktop, web via WASM, potentiellement mobile)

## 📦 Installation

### Prérequis

- [Rust](https://www.rust-lang.org/) (version 1.70.0 ou supérieure)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- Dépendances système pour Bevy (voir [documentation Bevy](https://bevyengine.org/learn/book/getting-started/setup/))

### Étapes d'installation

1. Cloner le dépôt :
   ```bash
   git clone https://github.com/votre-compte/oslike_ui_bevy_dsl.git
   cd oslike_ui_bevy_dsl
   ```

2. Créer le dossier de scripts et d'assets :
   ```bash
   mkdir -p scripts assets/fonts
   ```

3. Copier les polices nécessaires dans le dossier `assets/fonts` :
   ```bash
   # Exemple avec FiraSans
   curl -L https://github.com/mozilla/Fira/raw/master/ttf/FiraSans-Regular.ttf -o assets/fonts/FiraSans-Regular.ttf
   curl -L https://github.com/mozilla/Fira/raw/master/ttf/FiraSans-Bold.ttf -o assets/fonts/FiraSans-Bold.ttf
   ```

4. Créer un script UI de base (exemple dans `scripts/ui.rb`) :
   ```ruby
   window(id: "main", title: "Hello World", width: 400, height: 300) do
	 text(id: "hello", content: "Hello from Ruby DSL!")
	 button(id: "click_me", text: "Click Me!")
   end
   ```

5. Compiler et exécuter :
   ```bash
   cargo run
   ```

## 🚀 Utilisation

### Définir une interface utilisateur

Créez un fichier Ruby dans le dossier `scripts/`, par exemple `ui.rb`, et utilisez le DSL pour définir votre interface :

```ruby
# Fenêtre principale
window(id: "main_window", title: "Mon Application", width: 800, height: 600) do
  # En-tête avec logo et titre
  row(id: "header") do
	image(id: "logo", source: "assets/logo.png", width: 50, height: 50)
	text(id: "title", content: "Mon Application", size: 24)
  end
  
  # Contenu principal
  column(id: "content") do
	text(id: "info", content: "Bienvenue dans mon application!")
	
	# Formulaire
	grid(id: "form", columns: 2) do
	  text(id: "name_label", content: "Nom:")
	  input(id: "name_input", placeholder: "Entrez votre nom")
	  
	  text(id: "email_label", content: "Email:")
	  input(id: "email_input", placeholder: "Entrez votre email")
	end
	
	# Bouton
	button(id: "submit_btn", text: "Envoyer", on_click: "handle_submit")
  end
end

# Fonction pour gérer le clic sur le bouton
def handle_submit(params)
  puts "Formulaire soumis!"
  {
	updates: [
	  {
		id: "info",
		action: "setText",
		value: "Formulaire soumis avec succès!"
	  }
	]
  }.to_json
end
```

### Hot-reload

Le système prend en charge le rechargement à chaud des scripts UI. Modifiez simplement le fichier `scripts/ui.rb` pendant que l'application est en cours d'exécution, et l'interface se mettra à jour automatiquement.

### Composants disponibles

- `window` : Fenêtre avec titre, redimensionnable et déplaçable
- `button` : Bouton cliquable avec texte et/ou icône
- `text` : Texte avec style et alignement personnalisables
- `image` : Affichage d'images
- `canvas` : Zone de dessin personnalisé
- `svg` : Affichage de graphiques vectoriels SVG
- `input` : Champ de saisie de texte
- `scrollview` : Vue défilante pour contenu dépassant
- `list` : Liste d'éléments verticale ou horizontale
- `grid` : Grille d'éléments en lignes et colonnes
- `viewport3d` : Fenêtre de rendu 3D
- `stack`, `row`, `column` : Conteneurs pour disposition d'éléments

## 🧩 Architecture

Le projet est organisé selon l'architecture suivante :

- `src/ui/components.rs` : Définition des composants UI et leurs propriétés
- `src/ui/builder/` : Constructeurs pour convertir les composants DSL en entités Bevy
- `src/dsl/parser.rs` : Parseur pour le langage DSL Ruby
- `src/ui/systems.rs` : Systèmes Bevy pour gérer les interactions et mises à jour
- `src/main.rs` : Point d'entrée et configuration de l'application

## 💻 Plateformes supportées

- ✅ Windows
- ✅ macOS
- ✅ Linux
- ✅ Web (via WebAssembly)
- 🔄 Mobile (en développement)

## 🛠️ Compilation pour le Web (WASM)

Pour compiler le projet pour le web :

1. Installer les outils WASM :
   ```bash
   rustup target add wasm32-unknown-unknown
   cargo install wasm-bindgen-cli
   ```

2. Compiler avec la feature WASM :
   ```bash
   cargo build --release --target wasm32-unknown-unknown --features wasm
   ```

3. Générer les bindings JS/WASM :
   ```bash
   wasm-bindgen --out-dir ./web/pkg --target web target/wasm32-unknown-unknown/release/oslike_ui_bevy_dsl.wasm
   ```

4. Servir le dossier `web/` avec un serveur web statique.

## 📝 License

Ce projet est sous licence MIT ou Apache 2.0, au choix de l'utilisateur.

## 🙏 Remerciements

- [Bevy](https://bevyengine.org/) - Le moteur de jeu utilisé pour le rendu et l'ECS
- [Artichoke](https://www.artichokeruby.org/) - L'implémentation Ruby en Rust
- [Taffy](https://github.com/DioxusLabs/taffy) - Le moteur de layout Flexbox
- [RESVG](https://github.com/RazrFalcon/resvg) - Le moteur de rendu SVG