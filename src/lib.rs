mod video_stream;
mod components;
mod devices;

pub use components::*;
use sycamore::prelude::{RcSignal, create_rc_signal, Scope, create_signal, Signal};
pub use video_stream::VideoStream;
pub use devices::Devices;

#[derive(Debug)]
pub struct AppState {
    pub device_id: RcSignal<String>,
    pub devices: RcSignal<Devices>,
    pub show_controls: RcSignal<bool>,
}

impl AppState {
    pub async fn new() -> Self {

        let device_id = create_rc_signal("".to_string());
        let devices = create_rc_signal(Devices::load().await);
        let show_controls = create_rc_signal(false);

        Self {
            device_id: device_id,
            devices: devices,
            show_controls: show_controls,
        }
    }

    pub async fn wrap_state(ctx: Scope<'_>) -> &Signal<AppState> {
        create_signal(ctx, AppState::new().await)
    }
}
