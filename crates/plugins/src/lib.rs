pub struct Plugins {
    pub engine: wasm_bridge::Engine,
    pub loaded_plugins: slab::Slab<Plugin>,
}

pub struct Plugin {
    instance: wasm_bridge::component::Instance,
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
        let component = wasm_bridge::component::Component::new(&self.engine, plugin_bytes)?;
        let mut linker = wasm_bridge::component::Linker::new(&self.engine);

        linker.root().func_wrap("print", |_, (param,): (String,)| {
            println!("{param}",);
            Ok(())
        })?;

        let mut store = wasm_bridge::Store::new(&self.engine, ());
        let instance = linker.instantiate(&mut store, &component)?;

        let func = instance.get_typed_func::<(), ()>(&mut store, "run")?;
        func.call(&mut store, ())?;

        let plugin = Plugin { instance, store };
        let key = self.loaded_plugins.insert(plugin);

        Ok(key)
    }
}
