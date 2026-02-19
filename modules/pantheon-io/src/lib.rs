use std::sync::mpsc::{self, Receiver, Sender};

use pantheon_core::PantheonEvent;
use pantheon_log::{fatal, trace};
use pantheon_types::Vec2f;
use winit::{application::ApplicationHandler, error::EventLoopError, event::WindowEvent, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, window::Window};

pub enum IOEvent {
    IOStarted,
    CloseRequested,
    NewPhysicalSize(Vec2f),
    NewLogicalSize(Vec2f)
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
    size_factor: f64,
    physical_size: Vec2f,
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
        Ok(Self { window: None, event_sender, event_receiver, size_factor: 1.0 as f64, physical_size: Vec2f::default() })
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

    fn send(&self, event_loop: &ActiveEventLoop, event: IOEvent) {
        match self.event_sender.send(event) {
            Ok(..) => (),
            Err(_) => {
                fatal!("A SendError occured when trying to send an IOEvent to the engine thread.");
                fatal!("Assuming the engine thread has crashed!");
                event_loop.exit();
            }
        };
    }
}

impl ApplicationHandler for AppIO {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        trace!("Application resumed, creating window");
        let window_attributes = Window::default_attributes();
        self.window = Some(event_loop.create_window(window_attributes).unwrap());
        self.send(event_loop, IOEvent::IOStarted);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => self.send(event_loop, IOEvent::CloseRequested),
            WindowEvent::RedrawRequested => {
                // Handle incoming pantheon event
                for event in self.event_receiver.try_iter() {
                    match event {
                        PantheonEvent::Shutdown => {
                            trace!("Recieved PantheonEvent::Shutdown; Exiting EventLoop");
                            event_loop.exit();
                        }
                    }
                }
            }
            WindowEvent::Resized(size) => {
                self.physical_size = Vec2f::new(size.width as f32, size.height as f32);
                self.send(event_loop, IOEvent::NewPhysicalSize(self.physical_size));
                let size = size.to_logical(self.size_factor);
                self.send(event_loop, IOEvent::NewLogicalSize(Vec2f::new(size.width, size.height)));
            },
            WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer: _ } => {
                self.size_factor = scale_factor;
                self.send(event_loop, IOEvent::NewLogicalSize(self.physical_size / scale_factor as f32));
            }
            _ => ()
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.window.as_ref().unwrap().request_redraw();
    }
}


