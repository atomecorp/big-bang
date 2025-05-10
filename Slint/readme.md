# Artichoke Ruby + Slint

Une application universelle qui intègre Ruby (via [Artichoke](https://github.com/artichoke/artichoke)) avec une interface graphique moderne construite avec [Slint](https://slint.dev/).

## Instructions d'installation

L'installation se fait en plusieurs étapes pour éviter les problèmes de taille maximale des scripts. Suivez les parties dans l'ordre :

### 1. Création du projet

```bash
chmod +x artiRay_slint_part1.sh
./artiRay_slint_part1.sh
```

Cette étape crée la structure du projet et le fichier Cargo.toml.

### 2. Création du script Ruby

```bash
chmod +x artiRay_slint_part2.sh
./artiRay_slint_part2.sh
```

Cette étape crée le script Ruby qui sera exécuté par Artichoke.

### 3. Création de l'interface Slint

```bash
chmod +x artiRay_slint_part3.sh
./artiRay_slint_part3.sh
```

Cette étape crée l'interface utilisateur en utilisant le langage déclaratif de Slint.

### 4. Création du code Rust

```bash
chmod +x artiRay_slint_part4.sh
./artiRay_slint_part4.sh
chmod +x artiRay_slint_part4_suite.sh
./artiRay_slint_part4_suite.sh
```

Cette étape crée le code Rust qui intègre Artichoke et Slint.

### 5. Compilation et exécution

```bash
chmod +x artiRay_slint_part5.sh
./artiRay_slint_part5.sh
```

Cette étape compile et exécute l'application.

## Dépendances requises

- [Rust](https://www.rust-lang.org/tools/install) et Cargo
- Dépendances de développement pour Slint (peut varier selon le système)
  - Sur Ubuntu/Debian : `apt install libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`
  - Sur macOS : Aucune dépendance supplémentaire requise
  - Sur Windows : Aucune dépendance supplémentaire requise

## Utilisation

Une fois l'application démarrée :

1. **Onglet Visualisation** : Affiche les données calculées par Ruby
   - Un cercle animé qui se déplace selon les calculs Ruby
   - Un slider qui peut être ajusté pour modifier les valeurs
   - Une liste d'éléments générés dynamiquement

2. **Onglet Console** : Affiche la sortie du script Ruby
   - Bouton "Éditer Script" pour ouvrir le script dans votre éditeur par défaut
   - Bouton "Effacer" pour nettoyer la console

3. **Onglet Aide** : Informations sur l'application

## Modification du script Ruby en temps réel

Vous pouvez modifier le fichier `scripts/main.rb` dans votre éditeur préféré pendant que l'application est en cours d'exécution. Les modifications seront automatiquement détectées et appliquées.

Essayez de changer les formules pour la position ou la couleur du cercle, ou modifiez les textes affichés !

## Déboggage

Si vous rencontrez des problèmes :

1. Vérifiez les logs dans le terminal
2. Consultez l'onglet Console pour voir les erreurs Ruby
3. Assurez-vous que toutes les dépendances sont installées

## Architecture

L'application utilise une architecture basée sur des threads :

- **Thread principal** : Exécute Slint et gère l'interface utilisateur
- **Thread Ruby** : Exécute l'interpréteur Artichoke et le script Ruby

La communication entre les threads se fait via un état partagé protégé par un Mutex.

## Licence

Ce projet est distribué sous licence MIT.