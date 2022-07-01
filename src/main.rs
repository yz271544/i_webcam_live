use i_webcam_live::VideoStream;
use sycamore::prelude::*;

// use sycamore::view;

fn main() {
    // console_error_panic_hook::set_once();
    // tracing_wasm::set_as_global_default();

    sycamore::render(|ctx|
        view!{
            ctx,
            div(class="container p-2"){
                Video()
            }
        });
}

#[component]
fn Video<G: Html>(ctx: Scope) -> View<G> {

    let app_state = use_context::<AppState>(ctx);

    let video_ref = create_node_ref(ctx);
    ctx.spawn_future(async move {
        let el = video_ref.get::<DomNode>().unchecked_into();
        let video_stream = VideoStream::new(el);
    });

    view! {ctx,
        // div(class="row"){
        //     div(class="col-12"){
        //         h1(class="text-center hero-text text-blue font-bold"){
        //             "Sycamore"
        //         }
        //     }
        // }
        div {
            video(
                ref=video_ref,
                class="border border-gray-400 rounded-lg",
                autoplay=true,
                width=1280,
                height=720,
                src="http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ElephantsDream.mp4",
                type="video/mp4"
            )

        }
    }
}