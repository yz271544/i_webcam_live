use std::ops::Deref;

use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{MediaDeviceKind, MediaDevices, MediaDeviceInfo};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Devices(Vec<Device>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    pub kind: MediaDeviceKind,
    pub label: String,
    pub id: String,
}

impl Deref for Devices {
    type Target = Vec<Device>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Iterator for Devices {
    type Item = Device;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl Devices {
    pub async fn load() -> Self {
        let devices = Self::get_media_devices();

        let all_devices = JsFuture::from(devices.enumerate_devices().unwrap()).await.unwrap();

        Self::from(&all_devices)
    }

    pub fn video_devices(&self) -> impl Iterator<Item = &Device> {
        self.iter_by_kind(MediaDeviceKind::Videoinput)
    }

    pub fn audio_device(&self) -> impl Iterator<Item = &Device> {
        self.iter_by_kind(MediaDeviceKind::Audioinput)
    }

    fn iter_by_kind(&self, kind: MediaDeviceKind) -> impl Iterator<Item = &Device> {
        self.iter()
            .filter(move |d| d.kind == kind)
    }

    pub fn get_media_devices() -> MediaDevices {
        let window = web_sys::window().expect("no global `window` exists");
        let navigator = window.navigator();
        navigator
            .media_devices()
            .expect("no `navigator.mediaDevices` exists")
    }
}

impl From<&JsValue> for Devices {
    fn from(v: &JsValue) -> Self {
        match js_sys::try_iter(v) {
            Ok(Some(v)) => {
                let devices = v.into_iter()
                    .filter(|item| item.is_ok())
                    .map(|v| Device::from(v.unwrap()))
                    .collect::<Vec<_>>();
                Devices(devices)
            },
            _ => Default::default(),
        }
    }
}

impl From<JsValue> for Device {
    fn from(v: JsValue) -> Self {
        let device = v.unchecked_into::<MediaDeviceInfo>();
        Device {
            kind: device.kind(),
            label: device.label(),
            id: device.device_id(),
        }
    }
}
