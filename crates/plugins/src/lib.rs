pub struct Plugins {
    pub engine: wasm_bridge::Engine,
    pub loaded_plugins: slab::Slab<Plugin>,
}

pub struct Plugin {
    instance: wasm_bridge::component::Instance,
    store: wasm_bridge::Store<WasiCtx>,
}

struct WasiCtx {
    wasi_ctx: wasmtime_wasi::WasiCtx,
    table: wasmtime_wasi::ResourceTable,
}

impl wasmtime_wasi::WasiView for WasiCtx {
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut wasmtime_wasi::WasiCtx {
        &mut self.wasi_ctx
    }
}

impl Default for Plugins {
    fn default() -> Self {
        Self::new()
    }
}

fn get(v: &mut WasiCtx) -> &mut WasiCtx {
    v
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

        let wasi_ctx = wasmtime_wasi::WasiCtxBuilder::new().build();
        let table = wasmtime_wasi::ResourceTable::default();
        let ctx = WasiCtx { wasi_ctx, table };

        wasmtime_wasi::bindings::cli::environment::add_to_linker(&mut linker.0, get)?;
        wasmtime_wasi::bindings::cli::stdin::add_to_linker(&mut linker.0, get)?;
        wasmtime_wasi::bindings::cli::stdout::add_to_linker(&mut linker.0, get)?;
        wasmtime_wasi::bindings::cli::stderr::add_to_linker(&mut linker.0, get)?;
        wasmtime_wasi::bindings::cli::exit::add_to_linker(&mut linker.0, get)?;
        wasmtime_wasi::bindings::io::error::add_to_linker(&mut linker.0, get)?;
        wasmtime_wasi::bindings::sync_io::io::streams::add_to_linker(&mut linker.0, get)?;
        wasmtime_wasi::bindings::sync_io::filesystem::types::add_to_linker(&mut linker.0, get)?;
        wasmtime_wasi::bindings::filesystem::preopens::add_to_linker(&mut linker.0, get)?;

        linker.root().func_wrap("print", |_, (param,): (String,)| {
            println!("{param}",);
            Ok(())
        })?;

        let mut store = wasm_bridge::Store::new(&self.engine, ctx);
        let instance = linker.instantiate(&mut store, &component)?;

        let func = instance.get_typed_func::<(), ()>(&mut store, "run")?;
        func.call(&mut store, ())?;

        let plugin = Plugin { instance, store };
        let key = self.loaded_plugins.insert(plugin);

        Ok(key)
    }
}
