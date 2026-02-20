use std::thread::{self, JoinHandle};
use argus_io::{AppIO, IOError, IOEvent};
use pantheon_core::{AppConfig, PantheonEvent};
use hermes_log::{info, trace, warn};
use winit::keyboard::KeyCode;

#[derive(Debug)]
pub enum PantheonError {
    DuplicateApplicationNotAllowed,
    IOError(String),
    EngineThreadPaniced
}

pub struct Pantheon {
    app_config: AppConfig,
    app_io: AppIO,
    engine_thread: Option<JoinHandle<()>>,
}

pub trait Application {
    fn app_config(&self) -> AppConfig;
    fn on_start(&self) {

    }
    
    fn on_update(&self) {

    }

    fn on_exit(&self) {

    }
}

impl Pantheon {
    pub fn new(app: Box<dyn Application + Send>) -> Self {
        info!("Pantheon Version: {}", pantheon_core::VERSION);

        // Create AppIO rx and tx, so we can communicate between the main / render thread, and the
        // engine thread
        trace!("Creating AppIO");
        let (appio, rx_io, tx_pe) = argus_io::create().unwrap();
        
        // Spawn the engine thread, which gets the IOEvent rx, and the PantheonEvent tx
        trace!("Spawning engine thread...");
        let app_config = app.app_config();
        let join_handle = thread::spawn(move || {
            loop {
                let io_event = rx_io.recv().unwrap();
                match io_event {
                    IOEvent::IOStarted => {
                        trace!("Window was created, and the engine thread has started");
                        app.on_start();
                    },
                    IOEvent::CloseRequested => {
                        let _ = tx_pe.send(PantheonEvent::Shutdown);
                        break;
                    },
                    IOEvent::KeyPressed(key) => {
                        if key == KeyCode::KeyW {
                            let _ = tx_pe.send(PantheonEvent::Shutdown);
                            break;
                        }
                    }
                    _ => ()
                }
            }
        });

        Self { app_config, app_io: appio, engine_thread: Some(join_handle) }
    }

    pub fn run(&mut self) -> Result<(), PantheonError> {
        info!("Starting app: {}", self.app_config.name);
        info!(" By: {}", self.app_config.author);
        info!(" Version: {}", self.app_config.version);

        // Now we hand over control of the main thread to AppIO (winit::EventLoop)
        let result = self.app_io.start();

        // Return
        if let Some(handle) = self.engine_thread.take() {
            match handle.join() {
                Ok(..) => (),
                Err(..) => return Err(PantheonError::EngineThreadPaniced)
            }
        } else {
            warn!("There were no engine JoinHandle.");
        }

        match result {
            Ok(..) => Ok(()),
            Err(IOError::DuplicateApplicationNotAllowed) => Err(PantheonError::DuplicateApplicationNotAllowed),
            Err(IOError::EventLoopError(err)) => Err(PantheonError::IOError(err))
        }
    }
}
