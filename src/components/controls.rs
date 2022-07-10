use sycamore::prelude::*;
use sycamore_futures::spawn_local_scoped;
use tracing::info;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlSelectElement};
use crate::{AppState, devices::Device};


#[component]
pub async fn Controls<'a, G: Html>(ctx: Scope<'a>, state: &'a Signal<AppState>) -> View<G> {

    // state.track();
    let devices: &ReadSignal<Vec<Device>> = create_memo(ctx, || {
        state.get().devices.get().video_devices().cloned().collect()
    });

    create_effect(ctx, move || {
        state.track();
        spawn_local_scoped(ctx, async move {
            info!("controls {:?}", devices);
            // web_sys::console::log_1(&devices);
        });
    });

    let visible = create_memo(ctx, move || match *state.get().show_controls.get().as_ref() {
        true => "visible",
        false => "invisible",
    });
    let class = || format!("absolute bottom-2 p-5 w-full {}", visible.get());
    view! {ctx,
        div(class=class()) {
            div(class="flex justify-center") {
                div(class="xl:w-1/3") {
                    select(class="form-select appearance-none
                        block
                        px-3
                        py-1.5
                        text-base
                        font-normal
                        text-gray-700
                        bg-white bg-clip-padding bg-no-repeat
                        border border-solid border-gray-300
                        rounded
                        transition
                        ease-in-out
                        m-0
                        focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none",
                        on:change=|e: Event| {
                            let target: HtmlSelectElement = e.target().unwrap().unchecked_into();
                            let device_id = target.value();
                            state.get().device_id.set(device_id);
                        }) {
                            Keyed {
                                iterable: devices,
                                view: |ctx, device| view!{ctx, option(value=device.id) {
                                    (device.label)
                                }},
                                key: |device| device.id.clone(),
                            }
                    }
                }
            }
        }
    }
}