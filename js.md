````markdown
# Cahier des charges pour la création d'un Prompt Engineer

## 1. Introduction
Ce document définit les spécifications fonctionnelles et techniques pour un prompt engineer chargé de concevoir et d'implémenter une solution complète combinant :

- Une application desktop/mobile basée sur **Tauri** (WebView + JS/TS)
- Un backend local **Rust** (DSP, calculs intensifs) exposé via **Axum** ou les commandes Tauri
- Un serveur distant **Node.js** (SaaS) avec API REST/GraphQL et DSL pré-transpilé

L'objectif est de garantir une performance maximale, une UX fluide, et une architecture modulaire, tout en offrant un DSL haut-niveau sans overhead runtime.

## 2. Objectifs

1. **UI/UX** : Interface réactive et légère avec DOM en JavaScript/TypeScript "vanilla"
2. **Performance locale** : Calculs DSP et GPU-bound en Rust natif, appelés via Tauri IPC
3. **Architecture SaaS** : Backend Node.js scalable pour gestion des utilisateurs, stockage, authentification
4. **DSL** : Langage métier haut-niveau pré-transpilé en JS pour le front et en Rust pour le serveur
5. **Sécurité et fiabilité** : Communication sécurisée, tests unitaires et d'intégration, CI/CD automatisé
6. **Extensibilité** : Modules indépendants, documentation exhaustive, pipeline de build clair

## 3. Périmètre fonctionnel

| Fonctionnalité                        | Description                                                         |
|---------------------------------------|---------------------------------------------------------------------|
| Initialisation Tauri                  | Création du squelette Tauri + config pour WebView                  |
| UI/DOM manipulable                    | Composants JavaScript/TS, gestion des événements via JS natif      |
| Module DSP Rust                       | Bibliothèque Rust pour traitements audio/vidéo intensifs           |
| Exposition Rust ↔ JS                  | Commandes Tauri (IPC) ou Axum embarqué pour appels locaux          |
| Serveur Node.js                       | API REST/GraphQL, auth JWT, stockage base de données                |
| DSL pré-transpilé (JS & Rust)         | Syntaxe Ruby-like, compilation au build pour JS et Rust            |
| Transpilation                         | Script CLI/Babel plugin pour générer code "vanilla" optimisé      |
| Tests et couverture                   | Tests unitaires, e2e, bench de perf                                |
| CI/CD                                 | GitHub Actions/GitLab CI pour build, lint, tests, déploiement      |

## 4. Architecture Technique

```mermaid
flowchart LR
  subgraph Client
    UI[WebView Tauri]
    JS[Vanilla JS/TS UI]
    RustLocal[Rust DSP Module]
    UI -- IPC --> RustLocal
  end

  subgraph SaaS
    NodeJS[Node.js Backend]
    DB[(Base de données)]
    NodeJS -- accès DB --> DB
    DSL_Server[DSL pré-transpilation Rust]
    NodeJS -- intègre --> DSL_Server
  end

  UI -- API REST/GraphQL --> NodeJS
````

### 4.1 Technologies

* **Front-end** : HTML5, CSS, JavaScript/TypeScript
* **UI Framework** : Aucun (DOM direct)
* **Desktop/Mobile** : Tauri (Rust + WebView)
* **Backend local** : Rust (crate DSP), Axum ou commandes Tauri
* **Backend SaaS** : Node.js (Express / Apollo / Fastify)
* **DSL** : Script de build (Node.js CLI / Babel plugin)
* **CI/CD** : GitHub Actions, Docker

## 5. Spécifications du DSL

### 5.1 Syntaxe du DSL

Le DSL adopte une syntaxe **DSL expressive**, simple à lire et à écrire :

```dsl
# Déclaration d'un objet avec table de hash et méthodes
object MonObjet {
  hash store               # initialisation d'une Map/HashMap

  define_method set(key, value) {
    this.store.set(key, value)
  }

  define_method get(key) {
    return this.store.get(key)
  }

  # Condition statique évaluée au constructeur
  if store.size === 0 {
    this.isEmpty = true
  }
}

# Instanciation et utilisation
a = MonObjet.new()
a.set("foo", 42)
print(a.get("foo"))  # → 42
```

**Exemple complet DSL à transcrire :**

```dsl
# frozen_string_literal: true

a = new element({
  renderers: [:html], attach: :view, id: :my_test_box,
  type: :shape, apply: [:shape_color],
  left: 120, top: 0, width: 100, smooth: 15,
  height: 100, overflow: :visible, fasten: [], center: true
})

b = box({ left: 666, color: :blue, smooth: 6,
          id: :the_box2, depth: 1, top: 66 })
cc = circle({ color: :red, left: 0, top: 0 })
clone = ""

b.drag(:start) do
  b.color(:black)
  b.height(123)
  clone = grab(:view).circle({ color: :white,
    left: b.left, top: b.top, depth: 3 })
end

b.drag(:stop) do
  b.color(:purple)
  b.height = b.height + 100
  clone.delete(true)
end

b.drag(:locked) do |event|
  dx = event[:dx]; dy = event[:dy]
  x = (clone.left || 0) + dx.to_f
  y = (clone.top  || 0) + dy.to_f
  clone.left(x); clone.top(y)
  puts "x: #{x}"; puts "y: #{y}"
end

cc.drag({ restrict: { max: { left: 240, top: 190 } } }) do |event|
end

c = circle

c.drag({ restrict: a.id }) do |event|
end

t = text({ data: 'touch me to unbind drag stop for b (clone will not deleted anymore)', left: 250 })
t.touch(true) do
  b.drag({ remove: :stop })
end

tt = text({ data: "remove drag on circles", top: 99 })
tt.touch(true) do
  cc.drag(false)
  c.drag(false)
end
```

### 5.2 Réalisation du DSL

# Déclaration d'un objet avec table de hash et méthodes

object MonObjet {
hash store               # initialisation d'une Map/HashMap

define\_method set(key, value) {
this.store.set(key, value)
}

define\_method get(key) {
return this.store.get(key)
}

# Condition statique évaluée au constructeur

if store.size === 0 {
this.isEmpty = true
}
}

# Instanciation et utilisation

a = MonObjet.new()
a.set("foo", 42)
print(a.get("foo"))  # → 42

````

- **object \<Nom\> { ... }** : définit une classe/constructeur.
- **hash \<nom\>** : crée une table de hachage interne.
- **define_method \<nom\>(...){ ... }** : génère une méthode de classe.
- **if \<condition\> { ... }** : bloc évalué au sein du constructeur.

### 5.2 Réalisation du DSL

Le DSL est **pré-transpilé** lors du build en JavaScript classique et en module Rust :

1. **Parsing**
   - Utiliser un parser léger (ex. [nearley.js](https://nearley.js.org/) ou `pegjs`) ou un plugin Babel.
   - Définir une grammaire EBNF pour `object`, `hash`, `define_method`, `if` et les appels.

2. **AST → Génération de code**
   - Pour **JS** : générer une classe ES6 avec `constructor()`, champs `this.store = new Map()`, méthodes prototypes.
   - Pour **Rust** : générer un `struct` + `impl` avec `HashMap` et méthodes associées.

3. **Outils de compilation**
   - **Node.js CLI** : script `dsl-compiler.js` qui lit `*.dsl`, génère `*.js` et `*.rs` dans les dossiers front/back.
   - **Plugin Babel** : transforme inline les blocs DSL dans le code source `.ts` ou `.js`.

4. **Intégration au pipeline**
   - Ajouter une étape dans `package.json` / `build.rs` :
     ```js
// shapes.js — généré par ton compiler

import { element, box, circle, text, grab } from "./dsl_runtime.js";

const a = new element({
  renderers: ["html"],
  attach: "view",
  id: "my_test_box",
  type: "shape",
  apply: ["shape_color"],
  left: 120,
  top: 0,
  width: 100,
  smooth: 15,
  height: 100,
  overflow: "visible",
  fasten: [],
  center: true,
});

const b  = box({ left: 666, color: "blue", smooth: 6, id: "the_box2", depth: 1, top: 66 });
const cc = circle({ color: "red", left: 0, top: 0 });
let clone = "";

b.drag("start", () => {
  b.color("black");
  b.height(123);
  clone = grab("view")
    .circle({ color: "white", left: b.left, top: b.top, depth: 3 });
});

b.drag("stop", () => {
  b.color("purple");
  b.height = b.height + 100;
  clone.delete(true);
});

b.drag("locked", (event) => {
  const dx = event.dx;
  const dy = event.dy;
  const x  = (clone.left || 0) + dx;
  const y  = (clone.top  || 0) + dy;
  clone.left(x);
  clone.top(y);
  console.log("x:", x);
  console.log("y:", y);
});

cc.drag({ restrict: { max: { left: 240, top: 190 } } }, (event) => {});

const c = circle();

c.drag({ restrict: a.id }, (event) => {});

const t = text({ data: "touch me to unbind drag stop for b (clone will not deleted anymore)", left: 250 });
t.touch(true, () => {
  b.drag({ remove: "stop" });
});

const tt = text({ data: "remove drag on circles", top: 99 });
tt.touch(true, () => {
  cc.drag(false);
  c.drag(false);
});
````

* S’assurer que les artefacts générés sont commités ou re-générés à chaque CI.

5. **Tests de conformité**

    * Écrire des tests unitaires sur le parser (Jest pour JS, `cargo test` pour Rust).
    * Valider que la sortie JS/Rust compile et produit le même comportement.

6. **Documentation et exemples**

    * Fournir un guide DSL (`docs/DSL.md`) avec grammaire, exemples et mapping vers le code généré.

## 6. Communication et API## 6. Communication et API

* **Tauri IPC** : Rust ↔ JS

    * Commandes synchrones/async pour lancer DSP
* **HTTP** : UI ↔ Node.js

    * Endpoints REST (`/api/v1/...`) ou GraphQL Schema
    * Auth JWT, CORS, validation avec Zod / Joi

## 7. Sécurité

* TLS obligatoire pour SaaS
* JWT + rafraîchissement de token
* Sanitization des inputs DSL
* Linting et analyse statique (ESLint, Clippy)

## 8. Performance

* **Front** : build minifié (esbuild/webpack), cache agressif
* **Rust DSP** : profils release, vectorisation SIMD
* **Node.js** : clustering, load balancing, mise en cache (Redis)

## 9. Tests et Qualité

* **Tests unitaires** : Jest (JS), Cargo test (Rust), Mocha/Chai (Node.js)
* **Tests e2e** : Playwright/Cypress
* **Benchmark** : Bench-rs (Rust), Benchmark.js (JS)

## 10. CI/CD et Déploiement

* **Pipelines** :

    * Lint, build, tests, bench
    * Docker builds pour Rust, Node.js
    * Déploiement Tauri (packaging Windows/macOS/Linux)
    * Déploiement SaaS (Docker Compose, Kubernetes)

## 11. Planning Prévisionnel

| Phase             | Durée estimée | Livrable                               |
| ----------------- | ------------- | -------------------------------------- |
| Conception (spec) | 1 semaine     | Cahier des charges détaillé            |
| Setup projet      | 1 semaine     | Repos initiaux, config Tauri & Node.js |
| DSL Compiler      | 2 semaines    | CLI / plugin Babel + tests             |
| Modules DSP Rust  | 3 semaines    | Lib, API Tauri, bench                  |
| UI & Intégration  | 2 semaines    | Pages, composants, IPC                 |
| Backend SaaS      | 3 semaines    | API, auth, DB, DSL Rust intégré        |
| Tests & CI/CD     | 2 semaines    | Pipelines, e2e, couverture > 90%       |
| Packaging & Docs  | 1 semaine     | Packaging Tauri, doc utilisateur & dev |

## 12. Livrables

1. **Repo Front-end Tauri** avec code source et packaging
2. **DSL Compiler** (Node.js CLI / plugin)
3. **Lib DSP Rust** avec exemples et benchmarks
4. **Backend SaaS Node.js** (API, auth, DB)
5. **Documentation** : guide d'installation, API reference, examples
6. **CI/CD** opérationnel pour toutes les briques

---

*Fin du cahier des charges*

```
```
