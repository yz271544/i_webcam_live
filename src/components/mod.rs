mod video;
mod controls;

use sycamore::prelude::*;
use sycamore_futures::*;
use tracing::info;
use crate::AppState;

pub use video::Video;
pub use controls::Controls;

#[component]
pub async fn App<'a, G: Html>(ctx: Scope<'a>) -> View<G> {

    let state = AppState::wrap_state(ctx).await;

    create_effect(ctx, move || {
        state.track();
        spawn_local_scoped(ctx, async move {
            // info!("state: {:?}", state);
            provide_context_ref(ctx, state);
        });
    });

    // spawn_local_scoped(ctx, async move {
    //     let state = AppState::new().await;
    //     info!("state: {:?}", state);
    //     provide_context(ctx, state);
    // });


    // let view = provide_executor_scope(async {
    //     let state = use_context::<AppState>(ctx);
    //     info!("state: {:?}", state);

    //     render_to_string_await_suspense(|ctx| {
    //         view!{
    //             ctx,
    //             Suspense {
    //                 fallback: view!{ ctx, "Hello World!" },
    //                 App {}
    //             }
    //         }
    //     }).await;


    // });


    /* let view = provide_executor_scope(async {
        render_to_string_await_suspense(|cx| {
            view! { cx,
                Suspense {
                    fallback: view! { cx, "Loading..." },
                    App {}
                }
            }
        })
        .await
    }); */


    // let state = use_context::<AppState>(ctx);

    view!{
        ctx,
        div(class="p-2"){
            Video(state)
        }
    }

    // view!{
    //     ctx,
    //     div(class="container p-2"){
    //         p {"Hello World!"}
    //     }
    // }
}
