mod video_stream;
mod components;
mod devices;

pub use components::*;
use sycamore::prelude::{RcSignal, create_rc_signal};
pub use video_stream::VideoStream;
pub use devices::Devices;

#[derive(Debug)]
pub struct AppState {
    pub device_id: RcSignal<String>,
    pub devices: RcSignal<Devices>,
}

impl AppState {
    pub async fn new() -> Self {
        let device_id = create_rc_signal("".to_string());
        let devices = create_rc_signal(Devices::load().await);

        Self {
            device_id: device_id,
            devices: devices,
        }
    }
}
