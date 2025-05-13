# scripts/ui.rb
# Exemple d'interface utilisateur décrite en DSL Ruby

# Définir quelques fonctions d'aide
def handle_button_click(params)
  puts "Bouton cliqué: #{params['id']}"
  
  # Créer une mise à jour d'UI
  {
    updates: [
      {
        id: "result_text",
        action: "setText",
        value: "Vous avez cliqué sur le bouton #{params['id']} à #{Time.now.strftime('%H:%M:%S')}"
      }
    ]
  }.to_json
end

def handle_input_submit(params)
  value = params['value']
  puts "Input soumis: #{value}"
  
  # Créer une mise à jour d'UI
  {
    updates: [
      {
        id: "result_text",
        action: "setText",
        value: "Vous avez saisi: #{value}"
      }
    ]
  }.to_json
end

# Fenêtre principale
window(id: "main_window", title: "Démo OS Like UI", width: 800, height: 600, x: 100, y: 50) do
  # En-tête avec logo et titre
  row(id: "header", spacing: 10) do
    image(id: "logo", source: "assets/logo.png", width: 50, height: 50)
    text(id: "title", content: "Application de démonstration", size: 24, color: "rgb(240, 240, 255)")
  end
  
  # Barre d'outils
  row(id: "toolbar", spacing: 5) do
    button(id: "new_btn", text: "Nouveau", on_click: "handle_button_click", icon: "assets/icons/new.png")
    button(id: "open_btn", text: "Ouvrir", on_click: "handle_button_click", icon: "assets/icons/open.png")
    button(id: "save_btn", text: "Enregistrer", on_click: "handle_button_click", icon: "assets/icons/save.png")
  end
  
  # Zone de contenu principale
  row(id: "content", spacing: 10) do
    # Panneau latéral avec navigation
    column(id: "sidebar", spacing: 5, width: 200) do
      text(id: "nav_title", content: "Navigation", size: 18, color: "rgb(200, 200, 255)")
      
      list(id: "nav_list", direction: "vertical", spacing: 2) do
        button(id: "nav_home", text: "Accueil", on_click: "handle_button_click")
        button(id: "nav_projects", text: "Projets", on_click: "handle_button_click")
        button(id: "nav_settings", text: "Paramètres", on_click: "handle_button_click")
        button(id: "nav_help", text: "Aide", on_click: "handle_button_click")
      end
      
      # Visualisation 3D simple
      text(id: "preview_title", content: "Aperçu 3D", size: 18, color: "rgb(200, 200, 255)")
      viewport3d(id: "preview_3d", width: 180, height: 150, camera: { position: [0, 3, 5], target: [0, 0, 0] })
    end
    
    # Zone de contenu principale
    column(id: "main_content", spacing: 10) do
      # Formulaire de saisie
      text(id: "form_title", content: "Formulaire d'exemple", size: 20, color: "rgb(220, 220, 255)")
      
      grid(id: "form_grid", columns: 2, spacing: 10) do
        text(id: "name_label", content: "Nom:", align: "right")
        input(id: "name_input", placeholder: "Entrez votre nom", width: 250, on_submit: "handle_input_submit")
        
        text(id: "email_label", content: "Email:", align: "right")
        input(id: "email_input", placeholder: "Entrez votre email", width: 250, on_submit: "handle_input_submit")
        
        text(id: "age_label", content: "Âge:", align: "right")
        input(id: "age_input", placeholder: "Entrez votre âge", width: 250, on_submit: "handle_input_submit")
      end
      
      # Bouton de soumission
      button(id: "submit_btn", text: "Soumettre", on_click: "handle_button_click")
      
      # Zone de résultat
      text(id: "result_text", content: "Les résultats s'afficheront ici", color: "rgb(180, 180, 255)")
      
      # Exemple de canvas
      text(id: "canvas_title", content: "Zone de dessin", size: 18, color: "rgb(200, 200, 255)")
      canvas(id: "drawing_canvas", width: 400, height: 200)
    end
  end
  
  # Pied de page
  row(id: "footer", spacing: 5) do
    text(id: "footer_text", content: "UI Modulaire avec Bevy et DSL Ruby © 2025", size: 12, color: "rgb(150, 150, 150)")
  end
end

# Fenêtre de dialogue (exemple)
window(id: "dialog", title: "Exemple de dialogue", width: 400, height: 200, x: 300, y: 200) do
  column(id: "dialog_content", spacing: 10) do
    text(id: "dialog_text", content: "Ceci est un exemple de fenêtre de dialogue.\nVous pouvez la déplacer et la redimensionner.", align: "center")
    
    row(id: "dialog_buttons", spacing: 10, align: "center") do
      button(id: "dialog_ok", text: "OK", on_click: "handle_button_click")
      button(id: "dialog_cancel", text: "Annuler", on_click: "handle_button_click")
    end
  end
end