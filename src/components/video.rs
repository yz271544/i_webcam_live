use serde_json::json;
use sycamore::prelude::*;
use sycamore_futures::spawn_local_scoped;

use crate::{VideoStream, AppState, Controls};


#[component]
pub async fn Video<'a, G: Html>(ctx: Scope<'a>, state: &'a Signal<AppState>) -> View<G> {
    let video_ref = create_node_ref(ctx);

    state.track();
    let constraints = create_memo(ctx, || match state.get().device_id.get().as_str() {
        "" => json!({
            "facingMode": "user",
        }),
        id => json!({
            "deviceId": { "exact": id },
        }),
    });

    create_effect(ctx, move || {
        spawn_local_scoped(ctx, async move {
            let el = video_ref.get::<DomNode>().unchecked_into();
            let video_stream = VideoStream::new(el);
            video_stream.set_vidio_stream(&constraints.get()).await;
        });
    });

    view! {ctx,
        div(class="relative",
            on:mouseover = move |_| state.get().show_controls.set(true),
            on:mouseout = move |_| state.get().show_controls.set(false),
        ) {
            video(
                ref=video_ref,
                class="border border-gray-400 rounded-lg slign-top-middle",
                autoplay=true,
                width=640,
                height=480,
            )
            Controls(state)
        }
    }
}
