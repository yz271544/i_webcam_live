use i_webcam_live::{Video, App};
use sycamore::{prelude::*, view};

// use sycamore::view;
// #[tokio::main]
fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    // sycamore::render(|ctx|
    //     view!{
    //         ctx,
    //         div(class="container p-2"){
    //             Video()
    //         }
    //     });

    sycamore::render(|ctx| view!(ctx, App()));

    // return Ok(());
}
