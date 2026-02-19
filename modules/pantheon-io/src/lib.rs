use std::sync::mpsc::{self, Receiver, Sender};

use pantheon_core::PantheonEvent;
use pantheon_log::{trace};
use winit::{application::ApplicationHandler, error::EventLoopError, event::WindowEvent, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, window::Window};

pub enum IOEvent {
    CloseRequested
}

#[derive(Debug)]
pub enum IOError {
    DuplicateApplicationNotAllowed,
    EventLoopError(String)
}

pub struct AppIO {
    window: Option<Window>,
    event_sender: Sender<IOEvent>,
    event_receiver: Receiver<PantheonEvent>,
}

pub fn create() -> Result<(AppIO, Receiver<IOEvent>, Sender<PantheonEvent>), IOError> {
    // Create tx and rx's
    let (tx_io, rx_io) = mpsc::channel();
    let (tx_pe, rx_pe) = mpsc::channel();

    let appio = match AppIO::new(tx_io, rx_pe) {
        Ok(value) => value,
        Err(err) => return Err(err),
    };
    
    return Ok((appio, rx_io, tx_pe));
}

impl AppIO {
    pub fn new(event_sender: Sender<IOEvent>, event_receiver: Receiver<PantheonEvent>) -> Result<Self, IOError> {
        Ok(Self { window: None, event_sender, event_receiver })
    }

    pub fn start(&mut self) -> Result<(), IOError> {
        // Create event_loop, might fail if there already is one
        let event_loop = match EventLoop::builder().build() {
            Ok(loop_instance) => loop_instance,
            Err(EventLoopError::RecreationAttempt) => return Err(IOError::DuplicateApplicationNotAllowed),
            Err(e) => return Err(IOError::EventLoopError(e.to_string()))
        };
        
        event_loop.set_control_flow(ControlFlow::Poll); // Best for games
        
        trace!("AppIO EventLoop taking over the control flow");
        match event_loop.run_app(self) {
            Ok(..) => Ok(()),
            Err(err) => Err(IOError::EventLoopError(err.to_string()))
        }
    }
}

impl ApplicationHandler for AppIO {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        trace!("Application resumed, creating window");
        let window_attributes = Window::default_attributes();
        self.window = Some(event_loop.create_window(window_attributes).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                self.event_sender.send(IOEvent::CloseRequested);
            }
            WindowEvent::RedrawRequested => {
                for event in self.event_receiver.try_iter() {
                    match event {
                        PantheonEvent::Shutdown => {
                            trace!("Recieved PantheonEvent::Shutdown; Exiting loop");
                            event_loop.exit();
                        }
                    }
                }
            }
            _ => ()
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.window.as_ref().unwrap().request_redraw();
    }
}


