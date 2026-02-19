use std::thread;
use pantheon_io::{IOError, IOEvent};
use pantheon_log::trace;
use pantheon_core::PantheonEvent;

#[derive(Debug)]
pub enum PantheonError {
    DuplicateApplicationNotAllowed,
    IOError(String)
}

pub fn run() -> Result<(), PantheonError> {
    // Create AppIO rx and tx, so we can communicate between the main / render thread, and the
    // engine thread
    let (mut appio, rx_io, tx_pe) = pantheon_io::create().unwrap();

    // Spawn the engine thread, which gets the IOEvent rx, and the PantheonEvent tx
    thread::spawn(move || {
        loop {
            let io_event = rx_io.recv().unwrap();
            trace!("Recieved IO event");
            match io_event {
                IOEvent::CloseRequested => {
                    let _ = tx_pe.send(PantheonEvent::Shutdown);
                    break;
                }
            }
        }
    });

    // Now we hand over control of the main thread to AppIO (winit::EventLoop)
    match appio.start() {
        Ok(..) => Ok(()),
        Err(IOError::DuplicateApplicationNotAllowed) => Err(PantheonError::DuplicateApplicationNotAllowed),
        Err(IOError::EventLoopError(err)) => Err(PantheonError::IOError(err))
    }
}

pub fn main () -> Result<(), PantheonError> {
    run()
}
