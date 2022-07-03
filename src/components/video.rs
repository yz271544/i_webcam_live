use serde_json::json;
use sycamore::prelude::*;
use sycamore_futures::*;
use tracing::info;

use crate::{VideoStream, Devices};


#[component]
pub fn Video<G: Html>(ctx: Scope) -> View<G> {

    // let app_state = use_context::<AppState>(ctx);

    let video_ref = create_node_ref(ctx);

    spawn_local_scoped(ctx, async move {
        let el = video_ref.get::<DomNode>().unchecked_into();
        let video_stream = VideoStream::new(el);
        video_stream.set_vidio_stream(&json!({
            "audio": false,
            "video": {
                "facingMode": "user",
                "width": 640,
                "height": 480
            }
        })).await;

        let devices = Devices::load().await;
        info!("devices (tracing_wasm): {:?}", devices);
    });

    // ctx.spawn_future(async move {
    //     let el = video_ref.get::<DomNode>().unchecked_into();
    //     let video_stream = VideoStream::new(el);
    // });

    view! {ctx,
        // div(class="row"){
        //     div(class="col-12"){
        //         h1(class="text-center hero-text text-blue font-bold"){
        //             "Sycamore"
        //         }
        //     }
        // }
        div(class="relative") {
            video(
                ref=video_ref,
                class="border border-gray-400 rounded-lg slign-top-middle",
                autoplay=true,
                width=640,
                height=480,
                // src="http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ElephantsDream.mp4",
                // type="video/mp4"
            )
            Controls()
        }
    }
}

#[component]
fn Controls<G: Html>(ctx: Scope) -> View<G> {
    view! {ctx,
        div(class="absolute bottom-2 p-5 w-full") {
            div(class="flex justfy-center") {
                div(class="mb-4 xl:w-496") {
                    // button(class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded") {
                    //     "Start"
                    // }
                    select(class="form-select appearance-none
                    block
                    w-full
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
                    focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none", aria-label="Default select example") {
                        option(value="") {
                            "Select"
                        }
                        option(value="user") {
                            "User"
                        }
                        option(value="environment") {
                            "Environment"
                        }
                    }   
                }
            }
        }
    }
}