pub struct Plugins {
    pub engine: wasm_bridge::Engine,
    pub loaded_plugins: slab::Slab<Plugin>,
}

pub struct Plugin {
    instance: wasm_bridge::Instance,
    store: wasm_bridge::Store<()>,
}

impl Default for Plugins {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugins {
    pub fn new() -> Self {
        let engine = wasm_bridge::Engine::default();
        let loaded_plugins = slab::Slab::new();
        Self {
            engine,
            loaded_plugins,
        }
    }

    #[allow(deprecated)]
    pub fn load_plugin(&mut self, plugin_bytes: &[u8]) -> wasm_bridge::Result<usize> {
        let module = wasm_bridge::Module::new(&self.engine, plugin_bytes)?;
        let mut linker = wasm_bridge::Linker::new(&self.engine);

        let mut store = wasm_bridge::Store::new(&self.engine, ());

        let instance = linker.instantiate(&mut store, &module)?;

        let plugin = Plugin { instance, store };
        let key = self.loaded_plugins.insert(plugin);

        todo!();
    }
}
