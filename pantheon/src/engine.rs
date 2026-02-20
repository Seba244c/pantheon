use std::sync::mpsc::{Receiver, Sender};

use argus_io::IOEvent;
use hermes_log::trace;
use pantheon_core::PantheonEvent;
use winit::keyboard::KeyCode;

use crate::Application;

pub struct Engine {
    app: Box<dyn Application + Send>,
    app_io_rx: Receiver<IOEvent>,
    app_io_tx: Sender<PantheonEvent>
}

impl Engine {
    pub fn new(app: Box<dyn Application + Send>, app_io_rx: Receiver<IOEvent>,app_io_tx: Sender<PantheonEvent>) -> Self {
        Self { app, app_io_rx, app_io_tx }
    }

    pub fn run(&self) {
        // Prepare for the main loop
        self.app.on_start();

        // Wait until IO is ready
        loop {
            let io_event = self.app_io_rx.recv().unwrap();
            match io_event {
                IOEvent::IOStarted => {
                    trace!("Window was created, and the engine thread main loop has started");
                },
                IOEvent::CloseRequested => {
                    let _ = self.app_io_tx.send(PantheonEvent::Shutdown);
                    break;
                },
                IOEvent::KeyPressed(key) => {
                    if key == KeyCode::KeyW {
                        let _ = self.app_io_tx.send(PantheonEvent::Shutdown);
                        break;
                    }
                }
                _ => ()
            }
        }

        // Exit
        self.app.on_exit();
    }
}


