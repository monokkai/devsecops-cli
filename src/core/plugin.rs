use libloading::{Library, Symbol};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Failed to load library: {0}")]
    LibraryError(#[from] libloading::Error),
    #[error("Plugin initialization failed")]
    InitError,
}

pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn execute(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
}

type PluginInit = unsafe fn() -> *mut dyn Plugin;

pub struct PluginManager {
    plugins: Vec<Arc<dyn Plugin>>,
    libraries: Vec<Library>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            libraries: Vec::new(),
        }
    }

    pub fn load(&mut self, path: impl AsRef<Path>) -> Result<(), PluginError> {
        let path = path.as_ref();
        unsafe {
            let lib = Library::new(path)?;
            let init: Symbol<PluginInit> = lib.get(b"init")?;
            let plugin = Box::from_raw(init());

            self.plugins.push(Arc::from(plugin));
            self.libraries.push(lib);

            Ok(())
        }
    }

    pub fn get_plugin(&self, name: &str) -> Option<&Arc<dyn Plugin>> {
        self.plugins.iter().find(|p| p.name() == name)
    }
}
