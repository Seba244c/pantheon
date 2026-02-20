pub mod types;

pub enum PantheonEvent {
    Shutdown
}

pub struct AppConfig {
    pub name: String,
    pub author: String,
    pub version: String
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            name: "Unkown".into(),
            version: "0.1.0".into(),
            author: "Unkown".into(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
    
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }
    
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = author.into();
        self
    }
}

pub const VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "-", env!("GIT_HASH"));
