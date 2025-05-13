import React from 'react';

const ArchitectureDiagram = () => {
  return (
    <div className="flex flex-col items-center justify-center w-full h-full p-4 text-gray-800 bg-gray-50 rounded-lg shadow-lg">
      <h2 className="text-2xl font-bold mb-6 text-center">Architecture OSlike - Bevy + Ruby DSL</h2>
      
      <div className="w-full relative">
        {/* Core Engine */}
        <div className="p-4 bg-blue-100 border-2 border-blue-500 rounded-lg mb-4 relative">
          <h3 className="text-lg font-semibold text-blue-800 mb-2">Moteur Bevy Core</h3>
          <div className="grid grid-cols-3 gap-4">
            <div className="p-2 bg-blue-50 rounded border border-blue-300">
              <h4 className="font-medium">Rendu</h4>
              <ul className="ml-4 text-sm">
                <li>bevy_render</li>
                <li>bevy_sprite</li>
                <li>bevy_ui</li>
              </ul>
            </div>
            <div className="p-2 bg-blue-50 rounded border border-blue-300">
              <h4 className="font-medium">Interface</h4>
              <ul className="ml-4 text-sm">
                <li>bevy_text</li>
                <li>bevy_window</li>
                <li>resvg (SVG)</li>
              </ul>
            </div>
            <div className="p-2 bg-blue-50 rounded border border-blue-300">
              <h4 className="font-medium">Système</h4>
              <ul className="ml-4 text-sm">
                <li>ECS</li>
                <li>Événements</li>
                <li>Plugins</li>
              </ul>
            </div>
          </div>
        </div>
        
        {/* Middle Layer */}
        <div className="flex gap-4 mb-4">
          {/* DSL Engine */}
          <div className="flex-1 p-4 bg-purple-100 border-2 border-purple-500 rounded-lg">
            <h3 className="text-lg font-semibold text-purple-800 mb-2">Moteur DSL Ruby</h3>
            <div className="grid grid-cols-1 gap-2">
              <div className="p-2 bg-purple-50 rounded border border-purple-300">
                <h4 className="font-medium">Parser Ruby</h4>
                <p className="text-sm">Artichoke Ruby → AST → Entités Bevy</p>
              </div>
              <div className="p-2 bg-purple-50 rounded border border-purple-300">
                <h4 className="font-medium">Hot-Reload</h4>
                <p className="text-sm">Watcher de fichier + événements</p>
              </div>
            </div>
          </div>
          
          {/* Widget Library */}
          <div className="flex-1 p-4 bg-green-100 border-2 border-green-500 rounded-lg">
            <h3 className="text-lg font-semibold text-green-800 mb-2">Bibliothèque Widgets</h3>
            <div className="grid grid-cols-2 gap-2">
              <div className="p-2 bg-green-50 rounded border border-green-300">
                <h4 className="font-medium">Composants de base</h4>
                <ul className="ml-4 text-sm">
                  <li>window</li>
                  <li>button</li>
                  <li>text</li>
                </ul>
              </div>
              <div className="p-2 bg-green-50 rounded border border-green-300">
                <h4 className="font-medium">Composants avancés</h4>
                <ul className="ml-4 text-sm">
                  <li>viewport_3d</li>
                  <li>grid</li>
                  <li>scrollview</li>
                </ul>
              </div>
            </div>
          </div>
        </div>
        
        {/* User Application Layer */}
        <div className="p-4 bg-amber-100 border-2 border-amber-500 rounded-lg">
          <h3 className="text-lg font-semibold text-amber-800 mb-2">Applications Utilisateur</h3>
          <div className="grid grid-cols-2 gap-4">
            <div className="p-2 bg-amber-50 rounded border border-amber-300">
              <h4 className="font-medium text-center">Scripts Ruby</h4>
              <div className="text-sm p-2 bg-gray-100 rounded my-1 font-mono">
                window title: "App" do<br/>
                &nbsp;&nbsp;button text: "Click"<br/>
                end
              </div>
            </div>
            <div className="p-2 bg-amber-50 rounded border border-amber-300">
              <h4 className="font-medium text-center">UI rendue</h4>
              <div className="flex items-center justify-center h-16 bg-gray-100 rounded mt-1">
                <div className="w-4/5 h-4/5 border border-gray-400 rounded bg-gray-50 flex flex-col">
                  <div className="h-6 bg-gray-300 flex items-center px-2 text-xs">App</div>
                  <div className="flex-1 flex items-center justify-center">
                    <div className="px-3 py-1 bg-blue-500 text-white text-xs rounded">Click</div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        
        {/* Platform Layer */}
        <div className="mt-4 grid grid-cols-5 gap-2">
          <div className="p-2 bg-gray-100 rounded text-center border border-gray-400">
            <div className="text-xs font-medium">Windows</div>
          </div>
          <div className="p-2 bg-gray-100 rounded text-center border border-gray-400">
            <div className="text-xs font-medium">macOS</div>
          </div>
          <div className="p-2 bg-gray-100 rounded text-center border border-gray-400">
            <div className="text-xs font-medium">Linux</div>
          </div>
          <div className="p-2 bg-gray-100 rounded text-center border border-gray-400">
            <div className="text-xs font-medium">Web/WASM</div>
          </div>
          <div className="p-2 bg-gray-100 rounded text-center border border-gray-400">
            <div className="text-xs font-medium">Mobile</div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ArchitectureDiagram;
