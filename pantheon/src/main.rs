use pantheon_log::{error, fatal, info, trace, warn};
use pantheon_types::{color::colors, Vec2f};

pub fn main () {
    let vec = Vec2f::new(2.0, 3.0);
    let color = colors::BABY_PINK;
    trace!("Vector {vec} has length {}", vec.length());
    trace!("My favorite color is {color}");
    trace!("This is the program running..");
    info!("I have to inform you.");
    warn!("A warning, an error might occur!");
    error!("An error did occur!!");
    fatal!("Yup, that was fatal.");
}
