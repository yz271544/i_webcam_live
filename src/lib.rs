use tracing::info;
use web_sys::HtmlVideoElement;

pub struct VideoStream {
    el: HtmlVideoElement,
}

impl VideoStream {
    pub fn new(el: HtmlVideoElement) -> VideoStream {
        VideoStream {
            el: el
        }
    }

    pub fn set_vidio_stream(&self, constraints: &serde_json::Value) {
        let window = web_sys::window().expect("no global `window` exists");
        let navigator = window.navigator();
        let devices = navigator.media_devices().expect("no `navigator.mediaDevices` exists");

        info!("devices (tracing_wasm): {:?}", devices);
        web_sys::console::log_1(&devices);
    }
}