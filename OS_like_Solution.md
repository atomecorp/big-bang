# Cahier des charges : UI pour OS modulaire (Bevy + DSL)

## 🎯 Objectif général

Créer une **interface utilisateur complète et modulaire** pour un OS nouvelle génération, capable de :

* Gérer des interfaces graphiques 2D/3D
* Consommer peu de ressources à l'inactivité
* S'exécuter sur toutes plateformes (macOS, Windows, Linux, iOS, Android, navigateur via WASM)
* Supporter un DSL déclaratif (idéalement Ruby-like)
* Intégrer des composants interactifs dynamiques (drag’n’drop, fenêtre, dock, HUD, etc.)
* Permettre l'affichage de scènes 3D, SVG, images, textes riches, canvas custom

## 🧱 Architecture technique

### A. Moteur graphique principal

* Utiliser **Bevy** comme moteur de rendu central (2D/3D, ECS, plugins)
* Modules Bevy à intégrer :

  * `bevy_render`, `bevy_ui`, `bevy_text`, `bevy_sprite`, `bevy_audio`
  * `bevy_prototype_lyon` (pour dessin vectoriel)
  * `resvg` (pour SVG → texture)
  * `image` crate (pour affichage/manipulation d'images)
  * `bevy_webgl2` (pour compatibilité WASM)

### B. Moteur de scripting DSL

* DSL Ruby-like (via `Artichoke` ou `Opal`)
* Syntaxe déclarative type SwiftUI :

  ```ruby
  window(id: :main, title: 'My App', width: 800, height: 600) do
    text(data: "Hello World")
    button(data: "Click", on_click: :my_handler)
  end
  ```
* Moteur d’évaluation : parser Ruby → AST → builder d’entités Bevy
* Système de handlers Ruby <-> Rust pour interactivité
* Mode hot-reload à implémenter (via watchers ou WebSocket)

### C. Composants UI de base

* ✅ `window` : résizable, draggable, avec header/footer, fermeture
* ✅ `button` : simple ou avec icône, cliquable
* ✅ `text` : texte brut, texte riche (couleur, taille, alignement)
* ✅ `canvas` : zone de dessin libre avec événements
* ✅ `image` : chargement dynamique, scaling
* ✅ `svg` : rendu vectoriel via `resvg`
* ✅ `scrollview`, `list`, `grid`, `stack` : layout et scrollables
* ✅ `input` : champ de texte

### D. Gestion des ressources

* Mise en cache des entités/éléments
* Rendu **uniquement sur changement d’état**
* Event-driven UI (pas de redraw loop permanente)
* Mode “pause/inactivité” qui désactive le redraw

### E. Intégration 3D

* Composant `viewport_3d` intégré
* Support caméra, scène, objets animés, interactions via raycast
* Permet la cohabitation UI 2D et contenu 3D (ex : HUD)

### F. Interopérabilité plateforme

* Desktop natif : `cargo run` standard
* Web (WASM) : compilation via `wasm-pack` ou `trunk`
* Mobile : compilation cross-plateforme via `cargo mobile`, `cargo-ndk`
* Interface de stockage universel (sandboxed file system / IndexedDB / local files)

### G. Système de layout

* Utiliser **Taffy** (Flexbox pour Bevy)
* Ajout d’une couche DSL par-dessus : `row`, `column`, `padding`, `margin`, etc.
* Responsive possible (ajustement en fonction de la taille de l'écran)

### H. Système d’événements UI

* Événements clavier/souris/tactile redirigés vers les composants DSL
* Event bubbling, focus, propagation personnalisable
* Système d’actions : `on_click`, `on_drag`, `on_enter`, etc.

### I. Modules dynamiques et extensibilité

* Chargement dynamique de "composants DSL"
* Capacité à créer des **modules UI plug’n’play** (ex : horloge, terminal, app launcher)
* Gestion de thèmes/styles dynamiques (CSS-like en Ruby)

### J. Debug / Devtools

* Affichage live de l’arbre des entités
* Console DSL Ruby interactive (REPL)
* Log des events + traces d’UI updates

### K. Sécurité / Isolations

* Sandboxing du DSL (interdire `eval`, accès direct au système)
* Autorisation de fonctions système spécifiques via whitelist

---

## 🧪 Tests

* Benchmarks de FPS, consommation CPU/GPU en idle et en interaction
* Stress test UI (plusieurs fenêtres, animations, etc.)
* Tests multiplateforme (Linux/macOS/Windows/iOS/Android/Web)
* Tests DSL : parsing, comportement, sécurité des scripts

## 📦 Livrables

* Lib Rust modulaire (`ui_core`, `dsl_parser`, `widget_library`)
* Frontend d’exemple (éditeur graphique ou bureau OS-like)
* Générateur de documentation automatique des composants DSL
* Démo exportable sur Web + desktop

## 📌 Évolution future

* Intégration de voice control / LLM local
* Multi-utilisateur (via events WebSocket)
* Système de fenêtres multi-process
* Compilation statique des UI DSL vers Rust

---

Ce cahier des charges est conçu pour permettre la construction d'une **UI complète, interactive, modulaire et multiplateforme**, capable d’agir comme la base graphique d’un **OS programmable** ou d’un **environnement d’applications universelles**.
