use serde_json::json;
use sycamore::prelude::*;
use sycamore_futures::spawn_local_scoped;
use tracing::info;

use crate::{VideoStream, Devices, AppState, Controls};


#[component]
pub fn Video<'a, G: Html>(ctx: Scope<'a>, state: &'a Signal<AppState>) -> View<G> {
    // let mut video_ref = &NodeRef::default();
    let video_ref = create_node_ref(ctx);
    // let show_controls = create_signal(ctx, true);
    create_effect(ctx, move || {

        // video_ref.set(create_node_ref(ctx));
        state.track();

        let aaa = state.get().device_id.get().as_str();

        let none_id = "".to_string();
        let constraints = create_selector(ctx, move|| match state.get().device_id.get().as_str() {
            none_id => json!({
                "facingMode": "user",

            }),
            id => json!({
                "deviceId": { "exact": id },
            }),
        });

        // show_controls.track();


        spawn_local_scoped(ctx, async move {
            let el = video_ref.get::<DomNode>().unchecked_into();
            let video_stream = VideoStream::new(el);
            video_stream.set_vidio_stream(&constraints.get()).await;

            // let devices = Devices::load().await;
            // info!("devices (tracing_wasm): {:?}", devices);
        });
    });


    // create_effect(ctx, move || {
    //     constraints.track();
    //     ctx.spawn_future(async move {
    //         info!("future spawned: {:?}", constraints.get());
    //         let el = video_ref.get::<DomNode>().unchecked_into();
    //         let video_stream = VideoStream::new(el);
    //         video_stream.set_video_src(&constraints.get()).await;
    //     });
    // });



    // spawn_local_scoped(ctx, async move {
    //     let el = video_ref.get::<DomNode>().unchecked_into();
    //     let video_stream = VideoStream::new(el);
    //     video_stream.set_vidio_stream(&json!({
    //         "audio": false,
    //         "video": {
    //             "facingMode": "user",
    //             "width": 640,
    //             "height": 480
    //         }
    //     })).await;

    //     let devices = Devices::load().await;
    //     info!("devices (tracing_wasm): {:?}", devices);
    // });

    // ctx.spawn_future(async move {
    //     let el = video_ref.get::<DomNode>().unchecked_into();
    //     let video_stream = VideoStream::new(el);
    // });

    // let show_controls = create_signal(ctx, true);


    view! {ctx,
        // div(class="row"){
        //     div(class="col-12"){
        //         h1(class="text-center hero-text text-blue font-bold"){
        //             "Sycamore"
        //         }
        //     }
        // }
        div(class="relative",
            // on:mouseover = move |_| show_controls.set(true),
            // on:mouseout = move |_| show_controls.set(false),
            on:mouseover = move |_| state.get().show_controls.set(true),
            on:mouseout = move |_| state.get().show_controls.set(false),
        ) {
            video(
                ref=video_ref,
                class="border border-gray-400 rounded-lg slign-top-middle",
                autoplay=true,
                width=640,
                height=480,
                // src="http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ElephantsDream.mp4",
                // type="video/mp4"
            )
            Controls(state)
        }
    }
}
