use pantheon::{Application, Pantheon, PantheonError};
use pantheon_core::AppConfig;

struct Sandbox {}
impl Sandbox {
    fn new() -> Self {
        Self {  }
    }
}

impl Application for Sandbox {
    fn app_config(&self) -> AppConfig {
        AppConfig::new()
            .name("Sandbox")
            .author("ssnoer")
            .version(pantheon_core::VERSION)
    }
}

pub fn main () -> Result<(), PantheonError> {
    Pantheon::new(Box::new(Sandbox::new())).run()
}
