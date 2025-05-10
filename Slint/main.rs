use artichoke::prelude::*;
use slint::{Color, ModelRc, SharedString, Timer, TimerMode, VecModel};
use std::fs;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// Définir l'interface UI directement en utilisant la macro slint!
slint::slint! {
    import { Button, Slider } from "std-widgets.slint";
    
    export component MainWindow inherits Window {
        title: "Artichoke Ruby + Slint";
        min-width: 800px;
        min-height: 600px;
        background: #121212;
        
        // Propriétés d'entrée (depuis Rust)
        in property <float> position-x: 0.0;
        in property <float> position-y: 0.0;
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
                
                Text {
                    text: "Artichoke Ruby + Slint";
                    font-size: 24px;
                    color: #2196f3;
                    horizontal-alignment: center;
                    vertical-alignment: center;
                }
            }
            
            // Onglets
            Rectangle {
                height: 40px;
                
                HorizontalLayout {
                    spacing: 8px;
                    
                    Rectangle {
                        width: 33.3%;
                        background: current-tab == 0 ? #1e88e5 : transparent;
                        border-radius: 4px;
                        
                        Text {
                            text: "Visualisation";
                            color: current-tab == 0 ? white : #e0e0e0;
                            font-size: 14px;
                            horizontal-alignment: center;
                            vertical-alignment: center;
                        }
                        
                        TouchArea {
                            clicked => {
                                current-tab = 0;
                            }
                        }
                    }
                    
                    Rectangle {
                        width: 33.3%;
                        background: current-tab == 1 ? #1e88e5 : transparent;
                        border-radius: 4px;
                        
                        Text {
                            text: "Console";
                            color: current-tab == 1 ? white : #e0e0e0;
                            font-size: 14px;
                            horizontal-alignment: center;
                            vertical-alignment: center;
                        }
                        
                        TouchArea {
                            clicked => {
                                current-tab = 1;
                            }
                        }
                    }
                    
                    Rectangle {
                        width: 33.3%;
                        background: current-tab == 2 ? #1e88e5 : transparent;
                        border-radius: 4px;
                        
                        Text {
                            text: "Aide";
                            color: current-tab == 2 ? white : #e0e0e0;
                            font-size: 14px;
                            horizontal-alignment: center;
                            vertical-alignment: center;
                        }
                        
                        TouchArea {
                            clicked => {
                                current-tab = 2;
                            }
                        }
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
                                text: "Valeur ajustable: " + round(slider-value * 10) / 10;
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
                                    text: "Position: (" + round(position-x * 100) / 100 + ", " + round(position-y * 100) / 100 + ")";
                                    color: white;
                                }
                                Text { 
                                    text: "Valeur calculée: " + calculated-value;
                                    color: white;
                                }
                            }
                        }
                        
                        // Cercle animé
                        Rectangle {
                            width: 60%;
                            height: 150px;
                            background: #1e1e1e;
                            border-radius: 8px;
                            
                            Rectangle {
                                width: 40px;
                                height: 40px;
                                border-radius: 20px;
                                background: text-color;
                                x: parent.width / 2 + position-x * 20px - 20px;
                                y: parent.height / 2 + position-y * 10px - 20px;
                            }
                        }
                    }
                    
                    // Liste d'items
                    Rectangle {
                        height: 200px;
                        background: #222222;
                        border-radius: 4px;
                        padding: 8px;
                        
                        VerticalLayout {
                            spacing: 8px;
                            
                            Text { 
                                text: "Éléments calculés:";
                                color: #888888;
                            }
                            
                            for item[index] in list-items : Rectangle {
                                height: 30px;
                                background: mod(index, 2) == 0 ? #2a2a2a : #262626;
                                border-radius: 4px;
                                padding-left: 8px;
                                
                                Text {
                                    text: item;
                                    color: white;
                                    vertical-alignment: center;
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
                            spacing: 8px;
                            
                            Rectangle {
                                width: 120px;
                                height: 30px;
                                background: #007acc;
                                border-radius: 4px;
                                
                                Text {
                                    text: "Éditer Script";
                                    color: white;
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                
                                TouchArea {
                                    clicked => {
                                        edit-ruby-script();
                                    }
                                }
                            }
                            
                            Rectangle {
                                width: 100px;
                                height: 30px;
                                background: #444444;
                                border-radius: 4px;
                                
                                Text {
                                    text: "Effacer";
                                    color: white;
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                
                                TouchArea {
                                    clicked => {
                                        clear-console();
                                    }
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
}

// Le chemin vers le script Ruby
const RUBY_SCRIPT_PATH: &str = "scripts/main.rb";

// Structure pour passer des données entre threads
struct RubyState {
    dt: f32,
    time: f32,
    x: f32,
    y: f32,
    text: String,
    color: [f32; 3],
    slider_value: f32,
    calculated_value: f32,
    json_data: String,
    list_items: Vec<String>,
    script_output: String,
    script_error: String,
    running: bool,
}

impl Default for RubyState {
    fn default() -> Self {
        Self {
            dt: 0.0,
            time: 0.0,
            x: 0.0,
            y: 0.0,
            text: "Chargement...".to_string(),
            color: [1.0, 1.0, 1.0],
            slider_value: 5.0,
            calculated_value: 2.5,
            json_data: "{}".to_string(),
            list_items: Vec::new(),
            script_output: String::new(),
            script_error: String::new(),
            running: true,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Affichage du logo de démarrage
    println!("┌─────────────────────────────────────────┐");
    println!("│       Artichoke Ruby + Slint GUI        │");
    println!("│           Version simplifiée            │");
    println!("└─────────────────────────────────────────┘");
    
    // Initialisation Slint
    println!("[INFO] Initialisation de l'interface Slint...");
    let ui = MainWindow::new().expect("Erreur lors de l'initialisation de Slint");
    
    // Créer notre état partagé
    println!("[INFO] Création de l'état partagé...");
    let state = Arc::new(Mutex::new(RubyState::default()));
    let ruby_state = Arc::clone(&state);
    
    // Configuration des callbacks Slint
    println!("[INFO] Configuration des callbacks...");
    
    // Callback pour le slider
    let state_slider = Arc::clone(&state);
    ui.on_slider_changed(move |value| {
        if let Ok(mut state) = state_slider.lock() {
            state.slider_value = value;
        }
    });
    
    // Callback pour éditer le script
    let ui_handle_edit = ui.as_weak();
    ui.on_edit_ruby_script(move || {
        println!("[INFO] Ouverture du script Ruby dans l'éditeur par défaut...");
        if let Err(e) = open::that(RUBY_SCRIPT_PATH) {
            println!("[ERROR] Impossible d'ouvrir le script: {:?}", e);
            if let Some(ui) = ui_handle_edit.upgrade() {
                ui.set_error_message(format!("Erreur: impossible d'ouvrir le script - {:?}", e).into());
            }
        }
    });
    
    // Callback pour effacer la console
    let ui_handle_clear = ui.as_weak();
    ui.on_clear_console(move || {
        println!("[INFO] Effacement de la console...");
        if let Some(ui) = ui_handle_clear.upgrade() {
            ui.set_console_output(SharedString::from(""));
            ui.set_error_message(SharedString::from(""));
        }
    });
    
    // Initialiser les éléments de liste
    let items_model = ModelRc::new(VecModel::from(vec![
        SharedString::from("Item 1: Chargement..."),
        SharedString::from("Item 2: Chargement..."),
        SharedString::from("Item 3: Chargement..."),
    ]));
    ui.set_list_items(items_model);
    
    // Thread Ruby
    println!("[INFO] Démarrage du thread Ruby...");
    thread::spawn(move || {
        println!("[RUBY] Initialisation de l'interpréteur Ruby...");
        match artichoke::interpreter() {
            Ok(mut interp) => {
                println!("[RUBY] Interpréteur Ruby initialisé avec succès");
                
                // Configurer la redirection de sortie
                println!("[RUBY] Configuration de la redirection de sortie...");
                let _ = interp.eval(r#"
                    require 'stringio'
                    $stdout = StringIO.new
                "#.as_bytes());
                
                // Charger la bibliothèque JSON
                println!("[RUBY] Chargement de la bibliothèque JSON...");
                let _ = interp.eval(b"require 'json'");
                
                println!("[RUBY] Entrée dans la boucle principale");
                // Boucle principale du thread Ruby
                while {
                    let running = ruby_state.lock().unwrap().running;
                    running
                } {
                    // Récupérer les valeurs actuelles
                    let (time, dt, slider_value) = {
                        let state = ruby_state.lock().unwrap();
                        (state.time, state.dt, state.slider_value)
                    };
                    
                    // Définir les variables Ruby
                    let _ = interp.eval(format!("TIME = {}; DT = {}; USER_VALUE = {}", 
                                               time, dt, slider_value).as_bytes());
                    
                    // Lire et exécuter le script Ruby
                    match fs::read_to_string(RUBY_SCRIPT_PATH) {
                        Ok(script_content) => {
                            // Exécuter le script
                            match interp.eval(script_content.as_bytes()) {
                                Ok(_) => {
                                    // Récupérer le résultat JSON
                                    match interp.eval(b"RESULT.to_json") {
                                        Ok(result) => {
                                            match interp.try_convert_mut(result) {
                                                Ok(json_str) => {
                                                    let json_str: String = json_str;
                                                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_str) {
                                                        // Mise à jour de l'état
                                                        let mut state = ruby_state.lock().unwrap();
                                                        
                                                        // Extraire les valeurs
                                                        if let Some(x) = json.get("x").and_then(|v| v.as_f64()) {
                                                            state.x = x as f32;
                                                        }
                                                        if let Some(y) = json.get("y").and_then(|v| v.as_f64()) {
                                                            state.y = y as f32;
                                                        }
                                                        if let Some(text) = json.get("text").and_then(|v| v.as_str()) {
                                                            state.text = text.to_string();
                                                        }
                                                        if let Some(color_array) = json.get("color").and_then(|v| v.as_array()) {
                                                            if color_array.len() >= 3 {
                                                                for (i, c) in color_array.iter().take(3).enumerate() {
                                                                    if let Some(value) = c.as_f64() {
                                                                        state.color[i] = value as f32;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        if let Some(value) = json.get("calculated_value").and_then(|v| v.as_f64()) {
                                                            state.calculated_value = value as f32;
                                                        }
                                                        if let Some(json_data) = json.get("json_data") {
                                                            state.json_data = json_data.to_string();
                                                        }
                                                        if let Some(list_items) = json.get("list_items").and_then(|v| v.as_array()) {
                                                            state.list_items.clear();
                                                            for item in list_items {
                                                                if let Some(s) = item.as_str() {
                                                                    state.list_items.push(s.to_string());
                                                                }
                                                            }
                                                        }
                                                        
                                                        // Récupérer la sortie
                                                        if let Ok(stdout) = interp.eval(b"$stdout.string") {
                                                            if let Ok(s) = interp.try_convert_mut(stdout) {
                                                                let s: String = s;
                                                                if !s.is_empty() {
                                                                    state.script_output.push_str(&s);
                                                                    
                                                                    // Limiter la taille
                                                                    if state.script_output.len() > 5000 {
                                                                        state.script_output = state.script_output
                                                                            .chars()
                                                                            .skip(state.script_output.len() - 2500)
                                                                            .collect();
                                                                    }
                                                                    
                                                                    // Réinitialiser stdout
                                                                    let _ = interp.eval(b"$stdout.string = ''");
                                                                }
                                                            }
                                                        }
                                                        
                                                        state.script_error = String::new();
                                                    }
                                                },
                                                Err(e) => {
                                                    println!("[RUBY ERROR] Erreur de conversion: {:?}", e);
                                                    let mut state = ruby_state.lock().unwrap();
                                                    state.script_error = format!("Erreur de conversion: {:?}", e);
                                                }
                                            }
                                        },
                                        Err(e) => {
                                            println!("[RUBY ERROR] Erreur de résultat: {:?}", e);
                                            let mut state = ruby_state.lock().unwrap();
                                            state.script_error = format!("Erreur de résultat: {:?}", e);
                                        }
                                    }
                                },
                                Err(e) => {
                                    println!("[RUBY ERROR] Erreur d'exécution: {:?}", e);
                                    let mut state = ruby_state.lock().unwrap();
                                    state.script_error = format!("Erreur Ruby: {:?}", e);
                                }
                            }
                        },
                        Err(e) => {
                            println!("[RUBY ERROR] Erreur de lecture: {:?}", e);
                            let mut state = ruby_state.lock().unwrap();
                            state.script_error = format!("Erreur de lecture: {:?}", e);
                        }
                    }
                    
                    // Petite pause pour ne pas surcharger le CPU
                    thread::sleep(Duration::from_millis(16));
                }
                
                println!("[RUBY] Sortie de la boucle principale");
            },
            Err(e) => {
                println!("[RUBY ERROR] Échec de l'initialisation Ruby: {:?}", e);
                if let Ok(mut state) = ruby_state.lock() {
                    state.script_error = format!("Erreur d'initialisation Ruby: {:?}", e);
                }
            }
        }
    });
    
    println!("[INFO] Démarrage du timer pour mise à jour de l'interface...");
    
    // Timer pour mettre à jour l'UI régulièrement
    let ui_handle = ui.as_weak();
    let state_ui = Arc::clone(&state);
    
    let last_time = Rc::new(Mutex::new(Instant::now()));
    let timer = Timer::default();
    
    timer.start(
        TimerMode::Repeated,
        Duration::from_millis(16),  // ~60 FPS
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                // Calculer le delta time
                let now = Instant::now();
                let dt = {
                    let mut last = last_time.lock().unwrap();
                    let dt = now.duration_since(*last).as_secs_f32();
                    *last = now;
                    dt
                };
                
                // Mettre à jour l'état
                if let Ok(mut state) = state_ui.lock() {
                    // Mettre à jour le temps
                    state.time += dt;
                    state.dt = dt;
                    
                    // Mettre à jour l'UI avec les valeurs actuelles
                    ui.set_position_x(state.x);
                    ui.set_position_y(state.y);
                    ui.set_dynamic_text(state.text.clone().into());
                    ui.set_text_color(Color::from_rgb_f32(
                        state.color[0],
                        state.color[1],
                        state.color[2]
                    ));
                    ui.set_slider_value(state.slider_value);
                    ui.set_calculated_value(state.calculated_value);
                    ui.set_json_data(state.json_data.clone().into());
                    ui.set_console_output(state.script_output.clone().into());
                    ui.set_error_message(state.script_error.clone().into());
                    
                    // Mettre à jour la liste des éléments
                    if !state.list_items.is_empty() {
                        let shared_strings: Vec<SharedString> = state.list_items
                            .iter()
                            .map(|s| s.clone().into())
                            .collect();
                        
                        let items_model = ModelRc::new(VecModel::from(shared_strings));
                        ui.set_list_items(items_model);
                    }
                }
            }
        },
    );
    
    println!("[INFO] Démarrage de la boucle principale Slint...");
    
    // Exécuter la boucle principale Slint (bloquante)
    ui.run()?;
    
    println!("[INFO] Terminaison de l'application...");
    
    // Signaler au thread Ruby de s'arrêter avant de quitter
    if let Ok(mut state) = state.lock() {
        state.running = false;
    }
    
    // Attendre un peu pour que le thread Ruby se termine proprement
    thread::sleep(Duration::from_millis(100));
    
    println!("[INFO] Application terminée.");
    
    Ok(())
}