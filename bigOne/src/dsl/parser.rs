// src/dsl/parser.rs
use artichoke::prelude::*;
use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde_json;
use crate::ui::components::*;

/// Résultat de l'évaluation du DSL
pub struct DSLEvaluationResult {
    pub components: Vec<UIComponent>,
    pub errors: Vec<String>,
}

/// Service pour l'évaluation du code Ruby DSL
pub struct DSLParser {
    interp: Artichoke,
    component_registry: Arc<Mutex<HashMap<String, UIComponent>>>,
}

impl DSLParser {
    /// Crée une nouvelle instance du parser DSL
    pub fn new() -> Result<Self, String> {
        // Initialiser l'interpréteur Ruby via Artichoke
        let interp = match artichoke::interpreter() {
            Ok(interp) => interp,
            Err(e) => return Err(format!("Erreur d'initialisation Ruby: {:?}", e)),
        };

        let component_registry = Arc::new(Mutex::new(HashMap::new()));
        
        Ok(Self {
            interp,
            component_registry,
        })
    }

    /// Initialise le DSL avec les définitions de base
    pub fn initialize_dsl(&mut self) -> Result<(), String> {
        // Charger les bibliothèques standard nécessaires
        self.interp.eval(b"require 'json'").map_err(|e| format!("Erreur lors du chargement de JSON: {:?}", e))?;

        // Définir les classes de base du DSL
        let dsl_setup = r#"
            # Namespace de base pour notre DSL UI
            module OS
              # Classe de base pour tous les composants
              class Component
                attr_reader :id, :props, :children
                
                def initialize(id, props = {})
                  @id = id.to_s
                  @props = props || {}
                  @children = []
                end
                
                def add_child(child)
                  @children << child
                  self
                end
                
                def to_hash
                  result = {
                    id: @id,
                    type: self.class.name.split('::').last.downcase,
                    props: @props,
                  }
                  
                  result[:children] = @children.map(&:to_hash) unless @children.empty?
                  result
                end
                
                def to_json
                  to_hash.to_json
                end
              end
              
              # Window component
              class Window < Component
                def initialize(id: nil, title: 'Window', width: 400, height: 300, x: nil, y: nil, resizable: true, draggable: true, props: {})
                  id ||= "window_#{rand(1000000)}"
                  super(id, props.merge(title: title, width: width, height: height, x: x, y: y, resizable: resizable, draggable: draggable))
                end
              end
              
              # Button component
              class Button < Component
                def initialize(id: nil, text: 'Button', on_click: nil, width: nil, height: nil, icon: nil, props: {})
                  id ||= "button_#{rand(1000000)}"
                  super(id, props.merge(text: text, on_click: on_click, width: width, height: height, icon: icon))
                end
              end
              
              # Text component
              class Text < Component
                def initialize(id: nil, content: '', size: 16, color: nil, align: 'left', font: nil, props: {})
                  id ||= "text_#{rand(1000000)}"
                  super(id, props.merge(content: content, size: size, color: color, align: align, font: font))
                end
              end
              
              # Image component
              class Image < Component
                def initialize(id: nil, source: '', width: nil, height: nil, scale: nil, props: {})
                  id ||= "image_#{rand(1000000)}"
                  super(id, props.merge(source: source, width: width, height: height, scale: scale))
                end
              end
              
              # Canvas component
              class Canvas < Component
                def initialize(id: nil, width: 200, height: 200, on_draw: nil, on_click: nil, props: {})
                  id ||= "canvas_#{rand(1000000)}"
                  super(id, props.merge(width: width, height: height, on_draw: on_draw, on_click: on_click))
                end
              end
              
              # SVG component
              class SVG < Component
                def initialize(id: nil, source: '', width: nil, height: nil, props: {})
                  id ||= "svg_#{rand(1000000)}"
                  super(id, props.merge(source: source, width: width, height: height))
                end
              end
              
              # ScrollView component
              class ScrollView < Component
                def initialize(id: nil, width: 200, height: 200, props: {})
                  id ||= "scrollview_#{rand(1000000)}"
                  super(id, props.merge(width: width, height: height))
                end
              end
              
              # List component
              class List < Component
                def initialize(id: nil, direction: 'vertical', spacing: 5, props: {})
                  id ||= "list_#{rand(1000000)}"
                  super(id, props.merge(direction: direction, spacing: spacing))
                end
              end
              
              # Grid component
              class Grid < Component
                def initialize(id: nil, columns: 2, rows: nil, spacing: 5, props: {})
                  id ||= "grid_#{rand(1000000)}"
                  super(id, props.merge(columns: columns, rows: rows, spacing: spacing))
                end
              end
              
              # Input component
              class Input < Component
                def initialize(id: nil, placeholder: '', value: '', width: nil, on_change: nil, on_submit: nil, props: {})
                  id ||= "input_#{rand(1000000)}"
                  super(id, props.merge(placeholder: placeholder, value: value, width: width, on_change: on_change, on_submit: on_submit))
                end
              end
              
              # Viewport3D component
              class Viewport3D < Component
                def initialize(id: nil, width: 300, height: 300, scene: nil, props: {})
                  id ||= "viewport3d_#{rand(1000000)}"
                  
                  # Configuration par défaut de la caméra
                  camera = props[:camera] || { position: [0, 5, 10], target: [0, 0, 0] }
                  
                  super(id, props.merge(width: width, height: height, scene: scene, camera: camera))
                end
              end
              
              # Layout components
              class Stack < Component
                def initialize(id: nil, props: {})
                  id ||= "stack_#{rand(1000000)}"
                  super(id, props)
                end
              end
              
              class Row < Component
                def initialize(id: nil, spacing: 5, align: 'center', props: {})
                  id ||= "row_#{rand(1000000)}"
                  super(id, props.merge(spacing: spacing, align: align))
                end
              end
              
              class Column < Component
                def initialize(id: nil, spacing: 5, align: 'center', props: {})
                  id ||= "column_#{rand(1000000)}"
                  super(id, props.merge(spacing: spacing, align: align))
                end
              end
              
              # Helper methods for DSL construction
              class << self
                # Store for all created components
                @@components = {}
                
                # Define all component creation methods
                %w(window button text image canvas svg scrollview list grid input viewport3d stack row column).each do |type|
                  define_method(type) do |**kwargs, &block|
                    # Convert type to class name (e.g., 'viewport3d' -> 'Viewport3D')
                    class_name = type.split('_').map(&:capitalize).join
                    
                    # Create component
                    component = OS.const_get(class_name).new(**kwargs)
                    
                    # Register component
                    @@components[component.id] = component
                    
                    # Process block if given (for nested components)
                    if block_given?
                      # Current component becomes parent for the block
                      @parent_stack ||= []
                      @parent_stack.push(component)
                      
                      # Execute block
                      block.call
                      
                      # Restore parent
                      @parent_stack.pop
                    end
                    
                    # Add to parent if within a block
                    unless @parent_stack.nil? || @parent_stack.empty?
                      @parent_stack.last.add_child(component)
                    end
                    
                    component
                  end
                end
                
                # Get all components
                def components
                  @@components
                end
                
                # Get component by ID
                def component(id)
                  @@components[id.to_s]
                end
                
                # Reset all components
                def reset!
                  @@components = {}
                end
                
                # Export all components to JSON
                def to_json
                  # Find root components (those that aren't children of others)
                  child_ids = []
                  @@components.each_value do |component|
                    component.children.each do |child|
                      child_ids << child.id
                    end
                  end
                  
                  root_components = @@components.values.reject { |c| child_ids.include?(c.id) }
                  root_components.map(&:to_hash).to_json
                end
              end
            end
            
            # Define global DSL methods
            %w(window button text image canvas svg scrollview list grid input viewport3d stack row column).each do |type|
              define_method(type) do |**kwargs, &block|
                OS.send(type, **kwargs, &block)
              end
            end
            
            # Other DSL utilities
            def rgb(r, g, b)
              "rgb(#{r}, #{g}, #{b})"
            end
            
            def rgba(r, g, b, a)
              "rgba(#{r}, #{g}, #{b}, #{a})"
            end
            
            # Helper for event handlers
            def on(event, &block)
              block.to_s
            end
        "#;

        // Évaluer le code de configuration du DSL
        self.interp.eval(dsl_setup.as_bytes())
            .map_err(|e| format!("Erreur lors de l'initialisation du DSL: {:?}", e))?;

        Ok(())
    }

    /// Évalue un script DSL et retourne les composants générés
    pub fn evaluate_dsl(&mut self, script: &str) -> Result<DSLEvaluationResult, String> {
        // Réinitialiser les composants
        self.interp.eval(b"OS.reset!").map_err(|e| format!("Erreur lors de la réinitialisation des composants: {:?}", e))?;
        
        // Évaluer le script DSL
        match self.interp.eval(script.as_bytes()) {
            Ok(_) => {
                // Récupérer les composants générés au format JSON
                let components_json = self.interp.eval(b"OS.to_json")
                    .map_err(|e| format!("Erreur lors de l'exportation des composants: {:?}", e))?;
                
                let components_str: String = self.interp.try_convert_mut(components_json)
                    .map_err(|e| format!("Erreur lors de la conversion JSON: {:?}", e))?;
                
                // Convertir le JSON en structure de composants UI
                let components = self.parse_components_json(&components_str)?;
                
                Ok(DSLEvaluationResult {
                    components,
                    errors: Vec::new(),
                })
            },
            Err(e) => {
                // Erreur lors de l'évaluation du script
                Ok(DSLEvaluationResult {
                    components: Vec::new(),
                    errors: vec![format!("Erreur d'évaluation: {:?}", e)],
                })
            }
        }
    }

    /// Convertit le JSON généré par le DSL en structure de composants Rust
    fn parse_components_json(&self, json: &str) -> Result<Vec<UIComponent>, String> {
        let json_components: Vec<serde_json::Value> = serde_json::from_str(json)
            .map_err(|e| format!("Erreur lors de la désérialisation JSON: {:?}", e))?;
        
        let mut components = Vec::new();
        
        for json_component in json_components {
            if let Some(component) = self.parse_component_value(&json_component)? {
                components.push(component);
            }
        }
        
        Ok(components)
    }

    /// Convertit une valeur JSON en composant UI
    fn parse_component_value(&self, value: &serde_json::Value) -> Result<Option<UIComponent>, String> {
        let component_type = value["type"].as_str()
            .ok_or_else(|| "Type de composant manquant".to_string())?;
        
        let component_id = value["id"].as_str()
            .ok_or_else(|| "ID de composant manquant".to_string())?
            .to_string();
        
        let props = &value["props"];
        
        // Parse children recursively if they exist
        let mut children = Vec::new();
        if let Some(json_children) = value.get("children") {
            if let Some(children_array) = json_children.as_array() {
                for child_value in children_array {
                    if let Some(child) = self.parse_component_value(child_value)? {
                        children.push(child);
                    }
                }
            }
        }
        
        // Parse component based on its type
        match component_type {
            "window" => {
                let title = props["title"].as_str().unwrap_or("Window").to_string();
                let width = props["width"].as_f64().unwrap_or(400.0) as f32;
                let height = props["height"].as_f64().unwrap_or(300.0) as f32;
                let x = props["x"].as_f64().map(|v| v as f32);
                let y = props["y"].as_f64().map(|v| v as f32);
                let resizable = props["resizable"].as_bool();
                let draggable = props["draggable"].as_bool();
                
                Ok(Some(UIComponent::Window(WindowProps {
                    id: component_id,
                    title,
                    width,
                    height,
                    x,
                    y,
                    resizable,
                    draggable,
                    children,
                })))
            },
            "button" => {
                let text = props["text"].as_str().unwrap_or("Button").to_string();
                let width = props["width"].as_f64().map(|v| v as f32);
                let height = props["height"].as_f64().map(|v| v as f32);
                let on_click = props["on_click"].as_str().map(|s| s.to_string());
                let icon = props["icon"].as_str().map(|s| s.to_string());
                
                let mut style = HashMap::new();
                if let Some(style_obj) = props.get("style") {
                    if let Some(style_map) = style_obj.as_object() {
                        for (key, value) in style_map {
                            if let Some(value_str) = value.as_str() {
                                style.insert(key.clone(), value_str.to_string());
                            }
                        }
                    }
                }
                
                Ok(Some(UIComponent::Button(ButtonProps {
                    id: component_id,
                    text,
                    width,
                    height,
                    on_click,
                    icon,
                    style: Some(style),
                })))
            },
            "text" => {
                let content = props["content"].as_str().unwrap_or("").to_string();
                let size = props["size"].as_f64().map(|v| v as f32);
                let color = props["color"].as_str().map(|s| s.to_string());
                let align = props["align"].as_str().map(|s| s.to_string());
                let font = props["font"].as_str().map(|s| s.to_string());
                
                Ok(Some(UIComponent::Text(TextProps {
                    id: component_id,
                    content,
                    size,
                    color,
                    align,
                    font,
                })))
            },
            // Les autres types de composants suivent le même modèle...
            "image" => {
                let source = props["source"].as_str().unwrap_or("").to_string();
                let width = props["width"].as_f64().map(|v| v as f32);
                let height = props["height"].as_f64().map(|v| v as f32);
                let scale = props["scale"].as_f64().map(|v| v as f32);
                
                Ok(Some(UIComponent::Image(ImageProps {
                    id: component_id,
                    source,
                    width,
                    height,
                    scale,
                })))
            },
            "canvas" => {
                let width = props["width"].as_f64().unwrap_or(200.0) as f32;
                let height = props["height"].as_f64().unwrap_or(200.0) as f32;
                let on_draw = props["on_draw"].as_str().map(|s| s.to_string());
                let on_click = props["on_click"].as_str().map(|s| s.to_string());
                
                Ok(Some(UIComponent::Canvas(CanvasProps {
                    id: component_id,
                    width,
                    height,
                    on_draw,
                    on_click,
                })))
            },
            "svg" => {
                let source = props["source"].as_str().unwrap_or("").to_string();
                let width = props["width"].as_f64().map(|v| v as f32);
                let height = props["height"].as_f64().map(|v| v as f32);
                
                Ok(Some(UIComponent::SVG(SVGProps {
                    id: component_id,
                    source,
                    width,
                    height,
                })))
            },
            "scrollview" => {
                let width = props["width"].as_f64().unwrap_or(200.0) as f32;
                let height = props["height"].as_f64().unwrap_or(200.0) as f32;
                
                Ok(Some(UIComponent::ScrollView(ScrollViewProps {
                    id: component_id,
                    width,
                    height,
                    children,
                })))
            },
            "list" => {
                let direction = props["direction"].as_str().map(|s| s.to_string());
                let spacing = props["spacing"].as_f64().map(|v| v as f32);
                
                Ok(Some(UIComponent::List(ListProps {
                    id: component_id,
                    items: children,
                    direction,
                    spacing,
                })))
            },
            "grid" => {
                let columns = props["columns"].as_u64().unwrap_or(2) as usize;
                let rows = props["rows"].as_u64().map(|v| v as usize);
                let spacing = props["spacing"].as_f64().map(|v| v as f32);
                
                Ok(Some(UIComponent::Grid(GridProps {
                    id: component_id,
                    columns,
                    rows,
                    items: children,
                    spacing,
                })))
            },
            "input" => {
                let placeholder = props["placeholder"].as_str().map(|s| s.to_string());
                let value = props["value"].as_str().map(|s| s.to_string());
                let width = props["width"].as_f64().map(|v| v as f32);
                let on_change = props["on_change"].as_str().map(|s| s.to_string());
                let on_submit = props["on_submit"].as_str().map(|s| s.to_string());
                
                Ok(Some(UIComponent::Input(InputProps {
                    id: component_id,
                    placeholder,
                    value,
                    width,
                    on_change,
                    on_submit,
                })))
            },
            "viewport3d" => {
                let width = props["width"].as_f64().unwrap_or(300.0) as f32;
                let height = props["height"].as_f64().unwrap_or(300.0) as f32;
                let scene = props["scene"].as_str().map(|s| s.to_string());
                
                let camera = if let Some(cam) = props.get("camera") {
                    let position = if let Some(pos) = cam.get("position").and_then(|p| p.as_array()) {
                        if pos.len() >= 3 {
                            [
                                pos[0].as_f64().unwrap_or(0.0) as f32,
                                pos[1].as_f64().unwrap_or(5.0) as f32,
                                pos[2].as_f64().unwrap_or(10.0) as f32,
                            ]
                        } else {
                            [0.0, 5.0, 10.0]
                        }
                    } else {
                        [0.0, 5.0, 10.0]
                    };
                    
                    let target = if let Some(tgt) = cam.get("target").and_then(|t| t.as_array()) {
                        if tgt.len() >= 3 {
                            [
                                tgt[0].as_f64().unwrap_or(0.0) as f32,
                                tgt[1].as_f64().unwrap_or(0.0) as f32,
                                tgt[2].as_f64().unwrap_or(0.0) as f32,
                            ]
                        } else {
                            [0.0, 0.0, 0.0]
                        }
                    } else {
                        [0.0, 0.0, 0.0]
                    };
                    
                    Some(CameraProps { position, target })
                } else {
                    None
                };
                
                Ok(Some(UIComponent::Viewport3D(Viewport3DProps {
                    id: component_id,
                    width,
                    height,
                    scene,
                    camera,
                })))
            },
            "stack" => {
                Ok(Some(UIComponent::Stack(StackProps {
                    id: component_id,
                    children,
                })))
            },
            "row" => {
                let spacing = props["spacing"].as_f64().map(|v| v as f32);
                let align = props["align"].as_str().map(|s| s.to_string());
                
                Ok(Some(UIComponent::Row(RowProps {
                    id: component_id,
                    children,
                    spacing,
                    align,
                })))
            },
            "column" => {
                let spacing = props["spacing"].as_f64().map(|v| v as f32);
                let align = props["align"].as_str().map(|s| s.to_string());
                
                Ok(Some(UIComponent::Column(ColumnProps {
                    id: component_id,
                    children,
                    spacing,
                    align,
                })))
            },
            _ => {
                // Type de composant inconnu
                Err(format!("Type de composant inconnu: {}", component_type))
            }
        }
    }

    /// Exécute un callback Ruby et retourne le résultat
    pub fn execute_callback(&mut self, callback_name: &str, args: HashMap<String, String>) -> Result<String, String> {
        // Convertir les arguments en hash Ruby
        let args_ruby = format!(
            "{{ {} }}", 
            args.iter()
                .map(|(k, v)| format!("'{}' => '{}'", k, v.replace("'", "\\'")))
                .collect::<Vec<String>>()
                .join(", ")
        );
        
        // Construire et exécuter l'appel de fonction
        let ruby_call = format!("{}({})", callback_name, args_ruby);
        match self.interp.eval(ruby_call.as_bytes()) {
            Ok(result) => {
                // Convertir le résultat en String
                let result_str: String = self.interp.try_convert_mut(result)
                    .map_err(|e| format!("Erreur lors de la conversion du résultat: {:?}", e))?;
                
                Ok(result_str)
            },
            Err(e) => {
                Err(format!("Erreur lors de l'exécution du callback: {:?}", e))
            }
        }
    }
}

// Implémentation du trait Resource pour DSLParser
impl Resource for DSLParser {}